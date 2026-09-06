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

    // 派閥ごとのマテリアルキャッシュ（外側：Primary Color、内側：Accent Color、内部薄塗り：Interior）
    let mut outer_mats: HashMap<FactionId, Handle<StandardMaterial>> = HashMap::new();
    let mut inner_mats: HashMap<FactionId, Handle<StandardMaterial>> = HashMap::new();
    let mut interior_mats: HashMap<FactionId, Handle<StandardMaterial>> = HashMap::new();

    for &faction in FactionId::ALL.iter() {
        let p_col = faction.primary_color().to_srgba();
        let a_col = faction.accent_color().to_srgba();

        // 外側境界ライン (Primary Color, 不透明度 0.90, 発光感のあるUnlit)
        let outer_mat = materials.add(StandardMaterial {
            base_color: Color::srgba(p_col.red, p_col.green, p_col.blue, 0.90),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        });
        outer_mats.insert(faction, outer_mat);

        // 内側境界ライン (Accent/Secondary Color, 不透明度 0.85)
        let inner_mat = materials.add(StandardMaterial {
            base_color: Color::srgba(a_col.red, a_col.green, a_col.blue, 0.85),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        });
        inner_mats.insert(faction, inner_mat);

        // 領土内部の薄いオーバーレイ (Primary Color, 不透明度 0.08 で地形を極力邪魔しない)
        let interior_mat = materials.add(StandardMaterial {
            base_color: Color::srgba(p_col.red, p_col.green, p_col.blue, 0.08),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        });
        interior_mats.insert(faction, interior_mat);
    }

    // 各辺 (0..6) ごとの外側・内側ボーダーメッシュを事前生成
    // 六角形の上面 (Y = 0) のエッジに沿った細長いクアッドメッシュ
    let hex_r = crate::map::HEX_RADIUS;
    let (outer_edge_meshes, inner_edge_meshes) = create_border_edge_meshes(hex_r);
    let mut outer_mesh_handles = Vec::with_capacity(6);
    let mut inner_mesh_handles = Vec::with_capacity(6);
    for i in 0..6 {
        outer_mesh_handles.push(meshes.add(outer_edge_meshes[i].clone()));
        inner_mesh_handles.push(meshes.add(inner_edge_meshes[i].clone()));
    }

    // 領土内部の薄いフィル用メッシュ
    let interior_mesh = meshes.add(crate::map::create_hex_mesh(hex_r * 0.96, 0.02));

    // メインマップ(0)および左右ラップ(-1, +1)の全周回セクションにボーダーとオーバーレイを配置
    for wrap_offset in [-1, 0, 1] {
        let section_offset_x = wrap_offset as f32 * world_width;

        for (&coord, &faction) in &territory_map.tile_owners {
            let Some(&terrain) = map_grid.terrain_data.get(&coord) else {
                continue;
            };

            let height = terrain.height();
            let world_pos = coord.to_world_pos(hex_r);
            let y = height + 0.035;

            // 1. 領土内部の薄いフィルを描画
            if let Some(mat_handle) = interior_mats.get(&faction) {
                commands.spawn((
                    TerritoryOverlay,
                    Mesh3d(interior_mesh.clone()),
                    MeshMaterial3d(mat_handle.clone()),
                    Transform::from_xyz(world_pos.x + section_offset_x, y, world_pos.z)
                        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 6.0)),
                ));
            }

            // 2. 6方向の隣接タイルを検査し、自国領土以外と接する辺に2色ボーダーを配置
            let neighbors = coord.neighbors_with_width(map_w);
            for dir_idx in 0..6 {
                let neighbor_coord = neighbors[dir_idx];
                let is_border = territory_map.get_owner(&neighbor_coord) != Some(faction);

                if is_border {
                    // 外側ボーダー (Primary Color)
                    if let Some(mat_handle) = outer_mats.get(&faction) {
                        commands.spawn((
                            TerritoryOverlay,
                            Mesh3d(outer_mesh_handles[dir_idx].clone()),
                            MeshMaterial3d(mat_handle.clone()),
                            Transform::from_xyz(world_pos.x + section_offset_x, y + 0.005, world_pos.z),
                        ));
                    }
                    // 内側ボーダー (Accent/Secondary Color)
                    if let Some(mat_handle) = inner_mats.get(&faction) {
                        commands.spawn((
                            TerritoryOverlay,
                            Mesh3d(inner_mesh_handles[dir_idx].clone()),
                            MeshMaterial3d(mat_handle.clone()),
                            Transform::from_xyz(world_pos.x + section_offset_x, y + 0.006, world_pos.z),
                        ));
                    }
                }
            }
        }
    }

    info!(
        "Spawned Civilization-style two-tone border overlays for {} tiles across 3 wrapped sections.",
        territory_map.tile_owners.len()
    );
}

/// 六角形の6つの辺に対応する外側ボーダーと内側ボーダーのメッシュ（クアッド）を生成
fn create_border_edge_meshes(radius: f32) -> ([Mesh; 6], [Mesh; 6]) {
    use bevy::asset::RenderAssetUsages;
    use bevy::render::mesh::{Indices, PrimitiveTopology};

    // Pointy-topped 六角形の頂点 (k = 0..6, angle = 30° + k * 60°)
    // directions = [(1,0), (1,-1), (0,-1), (-1,0), (-1,1), (0,1)]
    // 各方向の法線角度とエッジを対応づける
    // 6つの頂点座標を計算
    let mut hex_corners = [Vec3::ZERO; 6];
    for (k, corner) in hex_corners.iter_mut().enumerate() {
        let angle = std::f32::consts::PI / 6.0 + (k as f32) * (std::f32::consts::PI / 3.0);
        *corner = Vec3::new(radius * angle.cos(), 0.0, radius * angle.sin());
    }

    // directions[i] に対応する辺の頂点インデックスのペア (start, end)
    // directions:
    // 0: (1, 0)   -> angle ~ 0°     -> 頂点 5 から 0
    // 1: (1, -1)  -> angle ~ -60°   -> 頂点 4 から 5
    // 2: (0, -1)  -> angle ~ -120°  -> 頂点 3 から 4
    // 3: (-1, 0)  -> angle ~ 180°   -> 頂点 2 から 3
    // 4: (-1, 1)  -> angle ~ 120°   -> 頂点 1 から 2
    // 5: (0, 1)   -> angle ~ 60°    -> 頂点 0 から 1
    let edge_corners: [(usize, usize); 6] = [
        (5, 0),
        (4, 5),
        (3, 4),
        (2, 3),
        (1, 2),
        (0, 1),
    ];

    let create_edge_quad = |v1: Vec3, v2: Vec3, r_start: f32, r_end: f32| -> Mesh {
        // v1, v2 は正規化された半径 1.0 における頂点
        let p0 = v1 * r_end;
        let p1 = v2 * r_end;
        let p2 = v2 * r_start;
        let p3 = v1 * r_start;

        let positions = vec![
            [p0.x, p0.y, p0.z],
            [p1.x, p1.y, p1.z],
            [p2.x, p2.y, p2.z],
            [p3.x, p3.y, p3.z],
        ];
        let normals = vec![[0.0, 1.0, 0.0]; 4];
        let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let indices = Indices::U32(vec![0, 1, 2, 0, 2, 3]);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(indices);
        mesh
    };

    let mut outer_meshes = Vec::with_capacity(6);
    let mut inner_meshes = Vec::with_capacity(6);

    for &(i1, i2) in &edge_corners {
        let v1 = hex_corners[i1] / radius;
        let v2 = hex_corners[i2] / radius;

        // 外側ボーダー: 0.90 ~ 0.98 * radius
        outer_meshes.push(create_edge_quad(v1, v2, 0.90 * radius, 0.98 * radius));
        // 内側ボーダー: 0.80 ~ 0.90 * radius
        inner_meshes.push(create_edge_quad(v1, v2, 0.80 * radius, 0.90 * radius));
    }

    (
        outer_meshes.try_into().unwrap(),
        inner_meshes.try_into().unwrap(),
    )
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
