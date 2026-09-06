use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::view::MinimapImageNode;
use super::MinimapState;
use crate::camera::MapCamera;
use crate::map::hex::{self, MAP_HEIGHT, MAP_WIDTH};
use crate::map::{MapGrid, HEX_RADIUS};

/// ミニマップ上でのクリック／ホールドによるカメラ位置の追随移動
pub fn handle_minimap_interaction_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut minimap_state: ResMut<MinimapState>,
    map_grid: Res<MapGrid>,
    image_query: Query<(
        &GlobalTransform,
        &ComputedNode,
        &Interaction,
        &bevy::ui::RelativeCursorPosition,
    ), With<MinimapImageNode>>,
    mut camera_query: Query<&mut MapCamera>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((gt, computed_node, interaction, rel_cursor)) = image_query.single() else {
        return;
    };
    let Ok(mut map_cam) = camera_query.single_mut() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        minimap_state.is_dragging = false;
        return;
    };

    let size = computed_node.size();
    if size.x <= 0.0 || size.y <= 0.0 {
        return;
    }

    // ノードの左上と右下のスクリーン座標（Bevy UI GlobalTransformはノードの中心）
    let center = gt.translation().truncate();
    let min = center - size * 0.5;
    let max = center + size * 0.5;

    // カーソルがミニマップ画像領域内にあるかどうかの判定
    let is_inside = rel_cursor.normalized.is_some()
        || (cursor_pos.x >= min.x
            && cursor_pos.x <= max.x
            && cursor_pos.y >= min.y
            && cursor_pos.y <= max.y);

    // ミニマップ上でクリック（またはタップ）された瞬間に追随状態を開始
    if *interaction == Interaction::Pressed
        || (mouse_button.just_pressed(MouseButton::Left) && is_inside)
    {
        minimap_state.is_dragging = true;
    }

    // 左ボタンが離されたら追随終了
    if mouse_button.just_released(MouseButton::Left) {
        minimap_state.is_dragging = false;
    }

    // 左ボタンを押している間（クリック中・ドラッグ中・長押し中）、視点中心をカーソル位置へ追随
    if minimap_state.is_dragging && mouse_button.pressed(MouseButton::Left) {
        // RelativeCursorPosition::normalized (0.0..1.0) が取得できれば優先して使用。
        // ミニマップ枠外へ少し出た場合でも cursor_pos と min/size から正確にクランプ計算。
        let (norm_x, norm_y) = if let Some(normalized) = rel_cursor.normalized {
            (normalized.x.clamp(0.0, 1.0), normalized.y.clamp(0.0, 1.0))
        } else {
            let nx = ((cursor_pos.x - min.x) / size.x).clamp(0.0, 1.0);
            let ny = ((cursor_pos.y - min.y) / size.y).clamp(0.0, 1.0);
            (nx, ny)
        };

        let map_w = if map_grid.width > 0 { map_grid.width } else { MAP_WIDTH };
        let map_h = if map_grid.height > 0 { map_grid.height } else { MAP_HEIGHT };

        let world_width = hex::map_world_width_with_width(HEX_RADIUS, map_w);
        let world_height = (map_h as f32) * 1.5 * HEX_RADIUS;

        // ミニマップ左端 (norm_x = 0.0) が X = 0、右端 (norm_x = 1.0) が X = world_width
        // cameraのwrap範囲 [-world_width/2, world_width/2] に合わせる
        let mut world_x = norm_x * world_width;
        if world_x > world_width / 2.0 {
            world_x -= world_width;
        }
        // 上端 (norm_y = 0.0) が -half_world_h (-Z)、下端 (norm_y = 1.0) が +half_world_h (+Z)
        let world_z = (norm_y - 0.5) * world_height;

        let target = Vec3::new(world_x, 0.0, world_z);

        // クリック／押下中は注視点（カメラ中心）を瞬時かつ滑らかに追随させる
        map_cam.target_focal_point = target;
        map_cam.current_focal_point = target;
    }
}
