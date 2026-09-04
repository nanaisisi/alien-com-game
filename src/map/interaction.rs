use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::hex::HexCoord;
use super::{HexTile, MapGrid, HEX_RADIUS};
use crate::camera::MapCamera;
use crate::state::AppState;

pub struct MapInteractionPlugin;

impl Plugin for MapInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTile>()
            .init_resource::<HoveredTile>()
            .init_resource::<LeftDragTracker>()
            .add_systems(OnEnter(AppState::InGame), setup_interaction_ui)
            .add_systems(
                Update,
                (
                    handle_tile_hover_and_click,
                    update_tile_highlight_system,
                    update_info_panel_system,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_interaction_ui);
    }
}

/// 現在選択されているタイル
#[derive(Resource, Default, Debug)]
pub struct SelectedTile(pub Option<HexCoord>);

/// 現在マウスが乗っているタイル
#[derive(Resource, Default, Debug)]
pub struct HoveredTile(pub Option<HexCoord>);

/// タイルクリック・ドラッグ判定用
#[derive(Resource, Default, Debug)]
struct LeftDragTracker {
    press_pos: Option<Vec2>,
    has_dragged: bool,
}

/// タイル情報UI表示用タグ
#[derive(Component)]
struct InfoPanelRoot;

#[derive(Component)]
struct InfoPanelText;

fn setup_interaction_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing: Query<Entity, With<InfoPanelRoot>>,
) {
    if !existing.is_empty() {
        return;
    }

    let font = asset_server.load("fonts/UDEVGothicNF-Regular.ttf");
    let font_bold = asset_server.load("fonts/UDEVGothicNF-Bold.ttf");

    // 左下に配置する半透明情報パネル
    commands
        .spawn((
            InfoPanelRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(24.0),
                bottom: Val::Px(24.0),
                width: Val::Px(320.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.10, 0.16, 0.90)),
            BorderColor::all(Color::srgb(0.25, 0.85, 0.75)),
        ))
        .with_children(|parent| {
            // パネル見出し
            parent.spawn((
                Text::new("PLANET SURVEYOR // 地形探査"),
                TextFont {
                    font: font_bold.clone().into(),
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.25, 0.85, 0.75)),
            ));

            // 詳細テキスト
            parent.spawn((
                InfoPanelText,
                Text::new("タイルをクリックして詳細情報を表示"),
                TextFont {
                    font: font.clone().into(),
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.90, 0.95, 0.98)),
            ));
        });
}

fn cleanup_interaction_ui(mut commands: Commands, query: Query<Entity, With<InfoPanelRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_tile_hover_and_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    map_grid: Res<MapGrid>,
    mut hovered_tile: ResMut<HoveredTile>,
    mut selected_tile: ResMut<SelectedTile>,
    mut drag_tracker: ResMut<LeftDragTracker>,
    mut map_camera_query: Query<&mut MapCamera>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = cameras.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        hovered_tile.0 = None;
        drag_tracker.press_pos = None;
        return;
    };

    let w = window.width();
    let h = window.height().max(1.0);

    // UI領域（上部バー y <= 58, 左下パネル x <= 360 && y >= h - 250, 右下ボタン x >= w - 240 && y >= h - 100）上ではタイル操作を無効化
    let is_over_top_bar = cursor_pos.y <= 58.0;
    let is_over_left_panel = cursor_pos.x <= 360.0 && cursor_pos.y >= (h - 260.0);
    let is_over_right_panel = cursor_pos.x >= (w - 240.0) && cursor_pos.y >= (h - 100.0);

    if is_over_top_bar || is_over_left_panel || is_over_right_panel {
        hovered_tile.0 = None;
        if mouse_button.just_released(MouseButton::Left) {
            drag_tracker.press_pos = None;
        }
        return;
    }

    // --- 左ボタンドラッグによるマップ移動処理 ---
    if mouse_button.just_pressed(MouseButton::Left) {
        drag_tracker.press_pos = Some(cursor_pos);
        drag_tracker.has_dragged = false;
    } else if mouse_button.pressed(MouseButton::Left)
        && let Some(press_pos) = drag_tracker.press_pos
    {
        let drag_vector = cursor_pos - press_pos;
        if drag_vector.length() > 5.0 {
            drag_tracker.has_dragged = true;
        }

        if drag_tracker.has_dragged {
            if let Ok(mut map_cam) = map_camera_query.single_mut() {
                let world_per_pixel = map_cam.current_viewport_height / h;
                let sin_angle = 14.0 / (14.0_f32.powi(2) + 12.0_f32.powi(2)).sqrt();
                let delta = cursor_pos - press_pos;
                let world_delta_x = delta.x * world_per_pixel;
                let world_delta_z = delta.y * world_per_pixel / sin_angle;

                let drag_offset = Vec3::new(-world_delta_x, 0.0, world_delta_z);
                map_cam.target_focal_point += drag_offset;
                map_cam.current_focal_point += drag_offset;
            }
            drag_tracker.press_pos = Some(cursor_pos);
        }
    }

    // カメラのレイを取得
    let Ok(ray) = camera.viewport_to_world(cam_transform, cursor_pos) else {
        return;
    };

    // 地面平面 (Normal = Vec3::Y, origin = Vec3::ZERO) との交差判定
    let normal = Vec3::Y;
    let denom = normal.dot(*ray.direction);
    if denom.abs() > 1e-6 {
        let t = -normal.dot(ray.origin) / denom;
        if t >= 0.0 {
            let hit_point = ray.origin + *ray.direction * t;
            let hex = HexCoord::from_world_pos(hit_point, HEX_RADIUS);

            if map_grid.tiles.contains_key(&hex) {
                hovered_tile.0 = Some(hex);

                // ドラッグしておらずクリックのみだった場合にタイル選択
                if mouse_button.just_released(MouseButton::Left) && !drag_tracker.has_dragged {
                    selected_tile.0 = Some(hex);
                }
            } else {
                hovered_tile.0 = None;
            }
        }
    }

    if mouse_button.just_released(MouseButton::Left) {
        drag_tracker.press_pos = None;
        drag_tracker.has_dragged = false;
    }
}

/// タイル選択・ホバー時のハイライト表現（マテリアルカラー変更またはインジケータ）
fn update_tile_highlight_system(
    hovered: Res<HoveredTile>,
    selected: Res<SelectedTile>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tiles_query: Query<(&HexTile, &MeshMaterial3d<StandardMaterial>)>,
) {
    if !hovered.is_changed() && !selected.is_changed() {
        return;
    }

    let h_coord = hovered.0;
    let s_coord = selected.0;

    for (tile, mat_handle) in &tiles_query {
        if let Some(mut mat) = materials.get_mut(mat_handle) {
            let is_selected = s_coord == Some(tile.coord);
            let is_hovered = h_coord == Some(tile.coord);

            if is_selected {
                // 選択中はシアンの強い発光ハイライト
                mat.base_color = Color::srgb(0.35, 0.95, 0.90);
                mat.emissive = LinearRgba::rgb(0.2, 0.8, 0.7);
            } else if is_hovered {
                // ホバー中は明るく強調
                mat.base_color = tile.terrain.hovered_color();
                mat.emissive = LinearRgba::rgb(0.1, 0.1, 0.1);
            } else {
                // 通常状態
                mat.base_color = tile.terrain.base_color();
                mat.emissive = LinearRgba::BLACK;
            }
        }
    }
}

/// 選択中タイルの情報を左下HUDに反映
fn update_info_panel_system(
    selected: Res<SelectedTile>,
    map_grid: Res<MapGrid>,
    territory_map: Res<crate::faction::territory::TerritoryMap>,
    mut query: Query<&mut Text, With<InfoPanelText>>,
) {
    if !selected.is_changed() {
        return;
    }

    let Ok(mut text) = query.single_mut() else {
        return;
    };

    if let Some(coord) = selected.0 {
        if let Some(&terrain) = map_grid.terrain_data.get(&coord) {
            let passable_str = if terrain.is_passable_ground() {
                "通行可能"
            } else {
                "通行不能 (特殊部隊/航空機のみ)"
            };

            let move_cost_str = if terrain.movement_cost() > 100.0 {
                "N/A".to_string()
            } else {
                format!("{:.1}", terrain.movement_cost())
            };

            let (col, row) = coord.to_col_row();
            let center_coord = HexCoord::from_col_row(crate::map::GRID_WIDTH / 2, 0);
            let dist_from_center = coord.distance(center_coord);

            let owner_str = if let Some(owner) = territory_map.tile_owners.get(&coord) {
                format!("国{}【{}】({})", owner.code(), owner.name_ja(), owner.formal_title())
            } else {
                "未領有・未開拓域 (Neutral Territory)".to_string()
            };

            let info = format!(
                "【ヘックス座標】\n  col: {}, row: {} (q: {}, r: {})\n  中心からの距離: {}\n\n\
                 【領有勢力】\n  {}\n\n\
                 【地形種別】\n  {}\n\n\
                 【移動コスト】: {}\n\
                 【地上移動】: {}\n\n\
                 【環境特性】\n  エイリアン活動兆候あり\n  資源探査可能",
                col,
                row,
                coord.q,
                coord.r,
                dist_from_center,
                owner_str,
                terrain.name(),
                move_cost_str,
                passable_str
            );
            **text = info;
        }
    } else {
        **text = "タイルをクリックして詳細情報を表示".to_string();
    }
}
