use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::hex::HexCoord;
use super::{HexTile, MapGrid, HEX_RADIUS};
use crate::state::AppState;

pub struct MapInteractionPlugin;

impl Plugin for MapInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTile>()
            .init_resource::<HoveredTile>()
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
            .add_systems(OnExit(AppState::InGame), cleanup_interaction_ui);
    }
}

/// 現在選択されているタイル
#[derive(Resource, Default, Debug)]
pub struct SelectedTile(pub Option<HexCoord>);

/// 現在マウスが乗っているタイル
#[derive(Resource, Default, Debug)]
pub struct HoveredTile(pub Option<HexCoord>);

/// タイル情報UI表示用タグ
#[derive(Component)]
struct InfoPanelRoot;

#[derive(Component)]
struct InfoPanelText;

fn setup_interaction_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/BIZUDGothic-Regular.ttf");
    let font_bold = asset_server.load("fonts/BIZUDGothic-Bold.ttf");

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

/// マウスカーソルの位置からY=0平面との交差を計算し、ホバー・クリックされたヘックスを判定
fn handle_tile_hover_and_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    map_grid: Res<MapGrid>,
    mut hovered_tile: ResMut<HoveredTile>,
    mut selected_tile: ResMut<SelectedTile>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((camera, cam_transform)) = cameras.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        hovered_tile.0 = None;
        return;
    };

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

                if mouse_button.just_pressed(MouseButton::Left) {
                    selected_tile.0 = Some(hex);
                }
            } else {
                hovered_tile.0 = None;
            }
        }
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

            let info = format!(
                "【ヘックス座標】\n  q: {}, r: {} (距離: {})\n\n\
                 【地形種別】\n  {}\n\n\
                 【移動コスト】: {}\n\
                 【地上移動】: {}\n\n\
                 【環境特性】\n  エイリアン活動兆候あり\n  資源探査可能",
                coord.q,
                coord.r,
                (coord.q.abs() + coord.r.abs() + (-coord.q - coord.r).abs()) / 2,
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
