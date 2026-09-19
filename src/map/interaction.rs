use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::hex::HexCoord;
use super::{HEX_RADIUS, HexTile, MapGrid};
use crate::camera::MapCamera;
use crate::state::AppState;

use super::types::{GridOverlayLine, MapDisplaySettings, YieldOverlayTag};

pub struct MapInteractionPlugin;

impl Plugin for MapInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTile>()
            .init_resource::<HoveredTile>()
            .init_resource::<LeftDragTracker>()
            .add_systems(
                Update,
                (
                    handle_tile_hover_and_click,
                    update_tile_highlight_system,
                    handle_map_display_shortcuts,
                    sync_grid_overlay_visibility,
                    sync_yield_overlay_visibility,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::InGame), setup_map_overlays);
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
    ui_blockers: Query<(&GlobalTransform, &ComputedNode), With<crate::ui::UiBlockMapInteraction>>,
    minimap_state: Option<Res<crate::ui::minimap::MinimapState>>,
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

    let h = window.height().max(1.0);

    // ミニマップドラッグ中か判定
    let is_minimap_dragging = minimap_state.as_ref().is_some_and(|s| s.is_dragging);

    // UIブロッカー要素（上部バー、情報パネル、アクションボタン、ミニマップ等）の上にカーソルがあるか動的に判定
    let is_over_ui = is_minimap_dragging
        || ui_blockers.iter().any(|(gt, computed_node)| {
            let size = computed_node.size();
            if size.x <= 0.0 || size.y <= 0.0 {
                return false;
            }

            let translation = gt.translation();
            let half_w = size.x * 0.5;
            let half_h = size.y * 0.5;
            let min_x = translation.x - half_w;
            let max_x = translation.x + half_w;
            let min_y = translation.y - half_h;
            let max_y = translation.y + half_h;

            cursor_pos.x >= min_x
                && cursor_pos.x <= max_x
                && cursor_pos.y >= min_y
                && cursor_pos.y <= max_y
        });

    if is_over_ui {
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

                let drag_offset = Vec3::new(-world_delta_x, 0.0, -world_delta_z);
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
            let hex = if map_grid.width > 0 {
                HexCoord::from_world_pos_with_width(hit_point, HEX_RADIUS, map_grid.width)
            } else {
                HexCoord::from_world_pos(hit_point, HEX_RADIUS)
            };

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

/// [G]: グリッド切り替え, [Y]: 産出表示切り替え
pub fn handle_map_display_shortcuts(
    mut action_events: MessageReader<crate::map::input::InGameActionEvent>,
    mut display_settings: ResMut<MapDisplaySettings>,
) {
    for event in action_events.read() {
        match event.0 {
            crate::map::input::InGameAction::ToggleGrid => {
                display_settings.show_grid = !display_settings.show_grid;
                info!(
                    "Hex Grid Display: {}",
                    if display_settings.show_grid {
                        "ENABLED"
                    } else {
                        "DISABLED"
                    }
                );
            }
            crate::map::input::InGameAction::ToggleYields => {
                display_settings.show_yields = !display_settings.show_yields;
                info!(
                    "Tile Yield Display: {}",
                    if display_settings.show_yields {
                        "ENABLED"
                    } else {
                        "DISABLED"
                    }
                );
            }
            _ => {}
        }
    }
}

/// グリッド線の表示・非表示を同期
pub fn sync_grid_overlay_visibility(
    display_settings: Res<MapDisplaySettings>,
    mut grid_query: Query<&mut Visibility, With<GridOverlayLine>>,
) {
    if !display_settings.is_changed() {
        return;
    }

    let target_vis = if display_settings.show_grid {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    for mut vis in &mut grid_query {
        *vis = target_vis;
    }
}

/// 産出アイコン/テキストの表示・非表示を同期
pub fn sync_yield_overlay_visibility(
    display_settings: Res<MapDisplaySettings>,
    mut yield_query: Query<&mut Visibility, With<YieldOverlayTag>>,
) {
    if !display_settings.is_changed() {
        return;
    }

    let target_vis = if display_settings.show_yields {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    for mut vis in &mut yield_query {
        *vis = target_vis;
    }
}

/// マップ生成時等にグリッド線・産出マーカーを生成
#[allow(clippy::too_many_arguments)]
pub fn setup_map_overlays(
    mut commands: Commands,
    map_grid: Res<MapGrid>,
    display_settings: Res<MapDisplaySettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing_grids: Query<Entity, With<GridOverlayLine>>,
    asset_server: Res<AssetServer>,
) {
    if !existing_grids.is_empty() || map_grid.tiles.is_empty() {
        return;
    }

    let hex_r = HEX_RADIUS;
    let map_w = map_grid.width.max(1);
    let world_width = crate::map::hex::map_world_width_with_width(hex_r, map_w);

    // 1. 六角形グリッド線用メッシュ（上面外周リング）
    let hex_corners = {
        let mut corners = [Vec3::ZERO; 6];
        for (k, corner) in corners.iter_mut().enumerate() {
            let angle = std::f32::consts::PI / 6.0 + (k as f32) * (std::f32::consts::PI / 3.0);
            *corner = Vec3::new(hex_r * angle.cos(), 0.0, hex_r * angle.sin());
        }
        corners
    };

    // 6辺の細いラインクアッドメッシュ
    let edge_mesh = {
        use bevy::asset::RenderAssetUsages;
        use bevy::render::mesh::{Indices, PrimitiveTopology};

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        let r_outer = hex_r * 0.99;
        let r_inner = hex_r * 0.94;

        for i in 0..6 {
            let next_i = (i + 1) % 6;
            let v1 = hex_corners[i] / hex_r;
            let v2 = hex_corners[next_i] / hex_r;

            let base_idx = (i * 4) as u32;
            let p0 = v1 * r_outer;
            let p1 = v2 * r_outer;
            let p2 = v2 * r_inner;
            let p3 = v1 * r_inner;

            positions.push([p0.x, 0.0, p0.z]);
            positions.push([p1.x, 0.0, p1.z]);
            positions.push([p2.x, 0.0, p2.z]);
            positions.push([p3.x, 0.0, p3.z]);

            for _ in 0..4 {
                normals.push([0.0, 1.0, 0.0]);
                uvs.push([0.0, 0.0]);
            }

            indices.extend_from_slice(&[
                base_idx,
                base_idx + 1,
                base_idx + 2,
                base_idx,
                base_idx + 2,
                base_idx + 3,
            ]);
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));
        meshes.add(mesh)
    };

    let grid_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.25, 0.45, 0.60, 0.45),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    let font_regular = asset_server.load(crate::ui::theme::FONT_REGULAR);

    let initial_grid_vis = if display_settings.show_grid {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    let initial_yield_vis = if display_settings.show_yields {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    // メインマップおよび左右ラップにグリッド線と産出情報を生成
    for wrap_offset in [-1, 0, 1] {
        let section_offset_x = wrap_offset as f32 * world_width;

        for (&coord, &terrain) in &map_grid.terrain_data {
            let height = terrain.height();
            let world_pos = coord.to_world_pos(hex_r);
            let y = height + 0.02;

            // グリッド線
            commands.spawn((
                GridOverlayLine,
                Mesh3d(edge_mesh.clone()),
                MeshMaterial3d(grid_mat.clone()),
                Transform::from_xyz(world_pos.x + section_offset_x, y, world_pos.z),
                initial_grid_vis,
            ));

            // タイル産出（Yield）3D Text表示 (食料, 生産力, エネルギー, 科学力)
            let (food, prod, energy, science) = terrain.base_yields();
            let mut yield_parts = Vec::new();
            if food > 0 {
                yield_parts.push(format!("{}食", food));
            }
            if prod > 0 {
                yield_parts.push(format!("{}生", prod));
            }
            if energy > 0 {
                yield_parts.push(format!("{}電", energy));
            }
            if science > 0 {
                yield_parts.push(format!("{}科", science));
            }

            if !yield_parts.is_empty() {
                let yield_text = yield_parts.join(" ");
                // 3D空間にX-Z平面に沿って横たわるテキスト
                commands.spawn((
                    YieldOverlayTag,
                    Text2d::new(yield_text),
                    TextFont {
                        font: font_regular.clone().into(),
                        font_size: FontSize::Px(12.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.95, 0.85)),
                    Transform::from_xyz(world_pos.x + section_offset_x, y + 0.03, world_pos.z)
                        .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                    initial_yield_vis,
                ));
            }
        }
    }
}
