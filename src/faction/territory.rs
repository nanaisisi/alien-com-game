use bevy::prelude::*;
use std::collections::HashMap;

use super::types::{FactionId, FactionManager};
use crate::map::hex::HexCoord;
use crate::map::terrain::TerrainType;
use crate::map::MapGrid;

/// タイルの領有権コンポーネント
#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct TileTerritory {
    pub owner: Option<FactionId>,
}

/// 派閥の開拓拠点・前哨基地
#[allow(dead_code)]
#[derive(Component, Debug, Clone)]
pub struct FactionOutpost {
    pub faction: FactionId,
    pub name: String,
    pub coord: HexCoord,
    pub level: u32,
}

/// マップ上の全タイルの領有権マップリソース
#[derive(Resource, Default, Debug)]
pub struct TerritoryMap {
    pub tile_owners: HashMap<HexCoord, FactionId>,
}

/// 各派閥の初期着陸位置と領土の初期化システム
pub fn setup_initial_faction_territories(
    mut commands: Commands,
    map_grid: Res<MapGrid>,
    map_config: Res<crate::map::settings::MapConfig>,
    mut territory_map: ResMut<TerritoryMap>,
    mut faction_manager: ResMut<FactionManager>,
) {
    if map_grid.tiles.is_empty() {
        return;
    }

    // すでに初期化されている場合はスキップ
    if !territory_map.tile_owners.is_empty() {
        return;
    }

    info!("Initializing initial faction outposts and territories...");

    let map_w = if map_grid.width > 0 { map_grid.width } else { map_config.width() };
    let map_h = if map_grid.height > 0 { map_grid.height } else { map_config.height() };
    let half_h = map_h / 2;

    // 6派閥の理想的な上陸候補地（経度方向 col を均等に分割し、通行可能陸地を探索）
    let col_step = (map_w / 6).max(1);
    let mut initial_outposts = Vec::new();

    for (i, &faction) in FactionId::ALL.iter().enumerate() {
        let base_col = (i as i32 * col_step + col_step / 2) % map_w;
        
        // 陸地でかつ進入可能なタイルを検索
        let mut chosen_coord = None;
        for row_offset in [0, 1, -1, 2, -2, 3, -3, 4, -4] {
            let row = row_offset.clamp(-half_h, half_h);
            let coord = HexCoord::from_col_row_with_width(base_col, row, map_w);
            if let Some(&terrain) = map_grid.terrain_data.get(&coord)
                && terrain.is_passable_ground() && terrain != TerrainType::ToxicSwamp {
                    chosen_coord = Some(coord);
                    break;
                }
        }

        // 見つからなければ平原などの通行可能タイルを広く探索
        let center_coord = chosen_coord.unwrap_or_else(|| {
            for row in -half_h..=half_h {
                for dc in -3..=3 {
                    let c = (base_col + dc).rem_euclid(map_w);
                    let coord = HexCoord::from_col_row_with_width(c, row, map_w);
                    if let Some(&t) = map_grid.terrain_data.get(&coord)
                        && t.is_passable_ground() {
                            return coord;
                        }
                }
            }
            HexCoord::from_col_row_with_width(base_col, 0, map_w)
        });

        initial_outposts.push((faction, center_coord));
    }

    // 各派閥の首都・初期拠点をスポーンし、周辺タイルを領土化
    for (faction, center) in initial_outposts {
        let outpost_name = match faction {
            FactionId::Empire => "ニュー・ソフィア第一司令基地",
            FactionId::GrandDuchy => "ハイムダール開拓ドーム",
            FactionId::Federation => "フォート・リバティ主拠点",
            FactionId::Republic => "紅星・第一人民工廠",
            FactionId::Commonwealth => "オセアニア・バイオハブ",
            FactionId::Union => "アヴァロン統合司令部",
        };

        commands.spawn(FactionOutpost {
            faction,
            name: outpost_name.to_string(),
            coord: center,
            level: 1,
        });

        // 中心と隣接タイル（半径1）を自国領土に
        let mut claimed = 0;
        territory_map.tile_owners.insert(center, faction);
        claimed += 1;

        for neighbor in center.neighbors_with_width(map_w) {
            if map_grid.tiles.contains_key(&neighbor) {
                territory_map.tile_owners.insert(neighbor, faction);
                claimed += 1;
            }
        }

        faction_manager.set_territory_count(faction, claimed);
    }

    info!(
        "Faction initial territories generated. Total claimed tiles: {}",
        territory_map.tile_owners.len()
    );
}

pub fn cleanup_territories(
    mut commands: Commands,
    mut territory_map: ResMut<TerritoryMap>,
    outposts: Query<Entity, With<FactionOutpost>>,
) {
    territory_map.tile_owners.clear();
    for entity in &outposts {
        commands.entity(entity).despawn();
    }
}
