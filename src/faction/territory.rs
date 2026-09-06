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

impl TerritoryMap {
    #[inline]
    pub fn get_owner(&self, coord: &HexCoord) -> Option<FactionId> {
        self.tile_owners.get(coord).copied()
    }

    #[cfg(test)]
    #[inline]
    pub fn is_owned_by(&self, coord: &HexCoord, faction: FactionId) -> bool {
        self.get_owner(coord) == Some(faction)
    }
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

/// 領土オーバーレイ表示用マーカーコンポーネント
#[derive(Component, Debug, Clone, Copy)]
pub struct TerritoryOverlay;

/// 領土変更・初期化時にメインマップ上に透過メッシュオーバーレイを生成・更新するシステム
pub fn update_territory_overlays(
    mut commands: Commands,
    map_grid: Res<MapGrid>,
    map_config: Res<crate::map::settings::MapConfig>,
    territory_map: Res<TerritoryMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing_overlays: Query<Entity, With<TerritoryOverlay>>,
) {
    if !territory_map.is_changed() {
        return;
    }

    // 既存のオーバーレイを一旦削除して再構築
    for entity in &existing_overlays {
        commands.entity(entity).despawn();
    }

    if territory_map.tile_owners.is_empty() || map_grid.tiles.is_empty() {
        return;
    }

    let map_w = if map_grid.width > 0 { map_grid.width } else { map_config.width() };
    let world_width = crate::map::hex::map_world_width_with_width(crate::map::HEX_RADIUS, map_w);

    // 派閥ごとの透過オーバーレイマテリアルキャッシュ
    let mut overlay_mats: HashMap<FactionId, Handle<StandardMaterial>> = HashMap::new();
    for &faction in FactionId::ALL.iter() {
        let base_c = faction.primary_color().to_srgba();
        // 透過度を高めに設定 (alpha = 0.40) し、発光(emissive)も加味して鮮明に浮かび上がらせる
        let color = Color::srgba(base_c.red, base_c.green, base_c.blue, 0.40);
        let mat = materials.add(StandardMaterial {
            base_color: color,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        });
        overlay_mats.insert(faction, mat);
    }

    // 六角形メッシュ（タイルの上面にぴったり合う薄いシリンダーメッシュ）
    // タイル半径よりわずかに小さく(0.96)して隣接タイルとの境界が綺麗に見えるようにする
    let mesh_handle = meshes.add(crate::map::create_hex_mesh(crate::map::HEX_RADIUS * 0.96, 0.04));

    // メインマップ(0)および左右ラップ(-1, +1)の全周回セクションにオーバーレイを配置
    for wrap_offset in [-1, 0, 1] {
        let section_offset_x = wrap_offset as f32 * world_width;

        for (&coord, &faction) in &territory_map.tile_owners {
            let Some(&terrain) = map_grid.terrain_data.get(&coord) else {
                continue;
            };
            let Some(mat_handle) = overlay_mats.get(&faction) else {
                continue;
            };

            let height = terrain.height();
            let world_pos = coord.to_world_pos(crate::map::HEX_RADIUS);
            // タイルの上面 (height) よりわずかに上 (height + 0.03) に配置
            let y = height + 0.03;

            commands.spawn((
                TerritoryOverlay,
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(mat_handle.clone()),
                Transform::from_xyz(world_pos.x + section_offset_x, y, world_pos.z)
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 6.0)),
            ));
        }
    }

    info!(
        "Spawned territory overlays for {} tiles across 3 wrapped sections.",
        territory_map.tile_owners.len()
    );
}

pub fn cleanup_territories(
    mut commands: Commands,
    mut territory_map: ResMut<TerritoryMap>,
    outposts: Query<Entity, With<FactionOutpost>>,
    overlays: Query<Entity, With<TerritoryOverlay>>,
) {
    territory_map.tile_owners.clear();
    for entity in &outposts {
        commands.entity(entity).despawn();
    }
    for entity in &overlays {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_territory_map_ownership() {
        let mut map = TerritoryMap::default();
        let coord = HexCoord::new(3, 4);

        assert_eq!(map.get_owner(&coord), None);
        assert!(!map.is_owned_by(&coord, FactionId::Empire));

        map.tile_owners.insert(coord, FactionId::Empire);
        assert_eq!(map.get_owner(&coord), Some(FactionId::Empire));
        assert!(map.is_owned_by(&coord, FactionId::Empire));
        assert!(!map.is_owned_by(&coord, FactionId::Federation));
    }
}
