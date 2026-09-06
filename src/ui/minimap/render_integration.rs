use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::view::{
    MinimapCameraBoxPart, MinimapCoordText, MINIMAP_HEIGHT, MINIMAP_WIDTH,
};
use crate::camera::MapCamera;
use crate::map::hex::{self, HexCoord, MAP_HEIGHT, MAP_WIDTH};
use crate::map::{MapGrid, HEX_RADIUS};

/// カメラの現在位置とズーム率に合わせて、ミニマップ上の視野矩形を更新（東西ラップ時の入れ違い・反対側表示に対応）
pub fn update_minimap_viewport_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    map_camera_query: Query<&MapCamera>,
    map_grid: Res<MapGrid>,
    mut box_query: Query<(&MinimapCameraBoxPart, &mut Node, &mut Visibility)>,
    mut text_query: Query<&mut Text, With<MinimapCoordText>>,
) {
    let Ok(map_cam) = map_camera_query.single() else {
        return;
    };

    let map_w = if map_grid.width > 0 { map_grid.width } else { MAP_WIDTH };
    let map_h = if map_grid.height > 0 { map_grid.height } else { MAP_HEIGHT };

    let world_width = hex::map_world_width_with_width(HEX_RADIUS, map_w);
    let world_height = (map_h as f32) * 1.5 * HEX_RADIUS;

    // ワールド座標 X とミニマップ X の対応：
    let norm_x = (map_cam.current_focal_point.x / world_width).rem_euclid(1.0);
    // ワールド座標 Z とミニマップ Y の対応：
    let half_world_h = world_height / 2.0;
    let norm_y = if half_world_h > 0.0 {
        ((map_cam.current_focal_point.z + half_world_h) / world_height).clamp(0.0, 1.0)
    } else {
        0.5
    };

    // カメラの視野の幅と高さをミニマップ上のピクセルサイズに換算
    let aspect_ratio = if let Ok(win) = windows.single() {
        if win.height() > 0.0 {
            win.width() / win.height()
        } else {
            16.0 / 9.0
        }
    } else {
        16.0 / 9.0
    };

    let cam_viewport_h = map_cam.current_viewport_height;
    let cam_w = cam_viewport_h * aspect_ratio;
    // カメラの傾斜（Y=14, Z=12）を考慮して地表面上での実質的な視野長さを計算
    let sin_angle = 14.0 / (14.0_f32.powi(2) + 12.0_f32.powi(2)).sqrt();
    let cam_h = cam_viewport_h / sin_angle;

    // ミニマップ上のピクセルサイズ（極端な最小値による正方形化を防ぐため、アスペクト比を保持して最小サイズを制限）
    let raw_box_w = (cam_w / world_width) * MINIMAP_WIDTH;
    let raw_box_h = (cam_h / world_height) * MINIMAP_HEIGHT;
    let min_scale = if raw_box_w > 0.0 && raw_box_h > 0.0 {
        (16.0 / raw_box_w).max(10.0 / raw_box_h).max(1.0)
    } else {
        1.0
    };
    let box_w = (raw_box_w * min_scale).min(MINIMAP_WIDTH);
    let box_h = (raw_box_h * min_scale).min(MINIMAP_HEIGHT);

    let center_px_x = norm_x * MINIMAP_WIDTH;
    let center_px_y = norm_y * MINIMAP_HEIGHT;

    let primary_left = center_px_x - box_w * 0.5;
    let top = center_px_y - box_h * 0.5;

    // 端を超えているかどうかの判定:
    let secondary_info = if primary_left < 0.0 {
        Some(primary_left + MINIMAP_WIDTH)
    } else if primary_left + box_w > MINIMAP_WIDTH {
        Some(primary_left - MINIMAP_WIDTH)
    } else {
        None
    };

    for (part, mut node, mut vis) in &mut box_query {
        match part {
            MinimapCameraBoxPart::Primary => {
                node.width = Val::Px(box_w);
                node.height = Val::Px(box_h);
                node.left = Val::Px(primary_left);
                node.top = Val::Px(top);
                *vis = Visibility::Visible;
            }
            MinimapCameraBoxPart::Secondary => {
                if let Some(sec_left) = secondary_info {
                    node.width = Val::Px(box_w);
                    node.height = Val::Px(box_h);
                    node.left = Val::Px(sec_left);
                    node.top = Val::Px(top);
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
                }
            }
        }
    }

    // 座標テキスト表示の更新
    if let Ok(mut text) = text_query.single_mut() {
        let current_hex = HexCoord::from_world_pos_with_width(map_cam.current_focal_point, HEX_RADIUS, map_w);
        let (c, r) = current_hex.to_col_row_with_width(map_w);
        **text = format!("COL {:02} ROW {:+02}", c, r);
    }
}
