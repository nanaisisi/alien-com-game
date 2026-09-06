use bevy::prelude::*;

use super::MinimapState;
use crate::faction::{FactionOutpost, PlayerFaction, TerritoryMap};
use crate::map::hex::{HexCoord, MAP_HEIGHT, MAP_WIDTH};
use crate::map::terrain::TerrainType;
use crate::map::MapGrid;

/// マップ生成完了後、または領土変更時にミニマップ用テクスチャをピクセル描画
pub fn update_minimap_texture_system(
    mut minimap_state: ResMut<MinimapState>,
    mut images: ResMut<Assets<Image>>,
    map_grid: Res<MapGrid>,
    territory_map: Res<TerritoryMap>,
    outposts_query: Query<&FactionOutpost>,
    player_faction: Res<PlayerFaction>,
) {
    if !minimap_state.needs_update && !territory_map.is_changed() {
        return;
    }

    if map_grid.terrain_data.is_empty() {
        return;
    }

    let Some(handle) = &minimap_state.texture_handle else {
        return;
    };
    let Some(mut image) = images.get_mut(handle) else {
        return;
    };

    let w = image.width() as usize;
    let h = image.height() as usize;
    let map_w = if map_grid.width > 0 { map_grid.width } else { MAP_WIDTH };
    let map_h = if map_grid.height > 0 { map_grid.height } else { MAP_HEIGHT };
    let half_h = map_h / 2;

    let mut pixels = vec![0u8; w * h * 4];

    // 各ピクセル (px, py) がどの HexCoord (col, row) に該当するかを計算して着色
    // ミニマップ上端 (py = 0) を画面上部（-Z, row = -half_h）、下端 (py = h - 1) を画面下部（+Z, row = +half_h）
    // 左端 (px = 0) を col = 0、右端 (px = w - 1) を col = map_w - 1
    for py in 0..h {
        let norm_y = py as f32 / h as f32;
        let row = -half_h + ((norm_y * (map_h as f32)) as i32).clamp(0, map_h - 1);

        for px in 0..w {
            let norm_x = px as f32 / w as f32;
            let col = ((norm_x * (map_w as f32)) as i32).clamp(0, map_w - 1);

            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
            let terrain = map_grid.terrain_data.get(&coord).copied().unwrap_or(TerrainType::Ocean);

            // 地形の基本色 (RGBA)
            let base_c = terrain.base_color().to_srgba();
            let mut r = (base_c.red * 255.0) as u8;
            let mut g = (base_c.green * 255.0) as u8;
            let mut b = (base_c.blue * 255.0) as u8;
            let a = 255u8;

            // 領土オーバーレイ（ミニマップ上では控えめに表示）
            if let Some(owner) = territory_map.tile_owners.get(&coord) {
                let owner_color = owner.primary_color().to_srgba();
                let blend = 0.12; // 派閥カラーのブレンド率（控えめにして地形の視認性を優先）
                r = ((1.0 - blend) * (r as f32) + blend * owner_color.red * 255.0) as u8;
                g = ((1.0 - blend) * (g as f32) + blend * owner_color.green * 255.0) as u8;
                b = ((1.0 - blend) * (b as f32) + blend * owner_color.blue * 255.0) as u8;
            }

            let idx = (py * w + px) * 4;
            pixels[idx] = r;
            pixels[idx + 1] = g;
            pixels[idx + 2] = b;
            pixels[idx + 3] = a;
        }
    }

    // 各派閥の拠点をミニマップ上にハイライト描画 (3x3 ピクセルの輝点)
    for outpost in &outposts_query {
        let (col, row) = outpost.coord.to_col_row_with_width(map_w);
        let center_x = ((col as f32 + 0.5) / (map_w as f32) * (w as f32)) as i32;
        let center_y = (((row + half_h) as f32 + 0.5) / (map_h as f32) * (h as f32)) as i32;

        let is_player = outpost.faction == player_faction.0;
        let highlight_c = if is_player {
            [255, 255, 255, 255]
        } else {
            let col = outpost.faction.accent_color().to_srgba();
            [
                (col.red * 255.0) as u8,
                (col.green * 255.0) as u8,
                (col.blue * 255.0) as u8,
                255,
            ]
        };

        for dy in -1..=1 {
            for dx in -1..=1 {
                let px = (center_x + dx).rem_euclid(w as i32) as usize;
                let py = (center_y + dy).clamp(0, (h - 1) as i32) as usize;
                let idx = (py * w + px) * 4;
                pixels[idx] = highlight_c[0];
                pixels[idx + 1] = highlight_c[1];
                pixels[idx + 2] = highlight_c[2];
                pixels[idx + 3] = 255;
            }
        }
    }

    image.data = Some(pixels);
    minimap_state.needs_update = false;
}
