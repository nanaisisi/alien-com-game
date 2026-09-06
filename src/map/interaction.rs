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
            .add_systems(
                Update,
                (
                    handle_tile_hover_and_click,
                    update_tile_highlight_system,
                )
                    .run_if(in_state(AppState::InGame)),
            );
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
    let is_over_ui = is_minimap_dragging || ui_blockers.iter().any(|(gt, computed_node)| {
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

        cursor_pos.x >= min_x && cursor_pos.x <= max_x && cursor_pos.y >= min_y && cursor_pos.y <= max_y
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
