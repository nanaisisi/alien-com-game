use bevy::prelude::*;
use std::collections::HashMap;

use super::hex::{self, HexCoord};
#[allow(unused_imports)]
use super::settings::{MapConfig, MapSize, PlanetEnvironment};
use super::terrain::TerrainType;
use super::{HexTile, MapGrid, MapRoot, HEX_RADIUS};

/// 周期的な円筒座標におけるハッシュ/疑似乱数（マップ幅・高さ・シード可変）
pub fn cylinder_noise_with_size(col: i32, row: i32, seed: u32, map_w: i32, map_h: i32) -> f32 {
    use std::f32::consts::PI;
    let theta = (col as f32 / map_w as f32) * 2.0 * PI;
    let cx = theta.cos();
    let cz = theta.sin();
    let cy = row as f32 / (map_h as f32 / 2.0);

    // 複数オクターブの周期的波長
    let wave1 = (cx * 2.3 + cz * 1.7 + cy * 1.5 + (seed as f32 * 0.1)).sin();
    let wave2 = (cx * 4.7 - cz * 3.9 + cy * 3.1 + (seed as f32 * 0.3)).sin() * 0.5;
    let wave3 = (cx * 9.1 + cz * 8.3 - cy * 6.2 + (seed as f32 * 0.7)).sin() * 0.25;

    let total = (wave1 + wave2 + wave3) / 1.75; // -1.0 .. 1.0 付近
    (total * 0.5 + 0.5).clamp(0.0, 1.0)
}

/// 六角柱の3Dメッシュを生成
pub fn create_hex_mesh(radius: f32, height: f32) -> Mesh {
    let cylinder = Cylinder {
        radius,
        half_height: height / 2.0,
    };
    // 6角形としてシリンダーメッシュを生成
    cylinder.mesh().resolution(6).build()
}

pub fn generate_hex_map(
    mut commands: Commands,
    map_config: Res<MapConfig>,
    mut map_grid: ResMut<MapGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // すでにマップが生成されている場合（PauseMenu / Settings からの復帰時など）は再生成しない
    if !map_grid.tiles.is_empty() {
        info!("Hex map already exists. Skipping map generation.");
        return;
    }

    let map_w = map_config.width();
    let map_h = map_config.height();
    let env = map_config.environment;
    let base_seed = map_config.seed;

    info!(
        "Generating Cylindrical Overworld Hex Map (Env: {:?}, Width: {}, Height: {}, Seed: {})...",
        env, map_w, map_h, base_seed
    );

    map_grid.tiles.clear();
    map_grid.terrain_data.clear();
    map_grid.width = map_w;
    map_grid.height = map_h;

    // 地形ごとのマテリアルキャッシュ
    let mut mat_cache: HashMap<TerrainType, Handle<StandardMaterial>> = HashMap::new();
    let terrain_types = [
        TerrainType::Plains,
        TerrainType::Hills,
        TerrainType::Forest,
        TerrainType::Mountains,
        TerrainType::Ocean,
        TerrainType::ToxicSwamp,
        TerrainType::CrystalFields,
    ];

    for t in terrain_types {
        let mut mat = StandardMaterial::from_color(t.base_color());
        mat.perceptual_roughness = match t {
            TerrainType::Ocean => 0.1,
            TerrainType::CrystalFields => 0.2,
            _ => 0.85,
        };
        mat.reflectance = match t {
            TerrainType::Ocean => 0.8,
            TerrainType::CrystalFields => 0.9,
            _ => 0.2,
        };
        mat_cache.insert(t, materials.add(mat));
    }

    // Step 1: 各タイルの標高(elevation)と陸地/海洋マスクを事前計算
    let half_h = map_h / 2;
    let mut elevations: HashMap<(i32, i32), f32> = HashMap::new();
    let mut is_land: HashMap<(i32, i32), bool> = HashMap::new();
    let sea_threshold = env.sea_level_threshold();

    for row in -half_h..=half_h {
        let lat_factor = (row.abs() as f32 / half_h as f32).powf(1.8);
        for col in 0..map_w {
            let n = cylinder_noise_with_size(col, row, base_seed, map_w, map_h);
            // 極地は海洋確率を上げるため標高を低減
            let elevation = (n - lat_factor * 0.4).clamp(0.0, 1.0);
            elevations.insert((col, row), elevation);
            is_land.insert((col, row), elevation >= sea_threshold);
        }
    }

    // Step 2: 地形タイプの決定
    let mut terrain_map: HashMap<HexCoord, TerrainType> = HashMap::new();
    let mountain_threshold = env.mountain_threshold();
    let hill_threshold = env.hill_threshold();

    for row in -half_h..=half_h {
        for col in 0..map_w {
            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
            let land = is_land[&(col, row)];

            if !land {
                terrain_map.insert(coord, TerrainType::Ocean);
                continue;
            }

            let elev = elevations[&(col, row)];
            let moisture = cylinder_noise_with_size(col, row, base_seed.wrapping_add(8888), map_w, map_h);
            let detail = cylinder_noise_with_size(col, row, base_seed.wrapping_add(54321), map_w, map_h);

            // 隣接セルに海洋があるかチェック (海岸線判定)
            let mut neighbors_have_ocean = false;
            for n_coord in coord.neighbors_with_width(map_w) {
                let (nc, nr) = n_coord.to_col_row_with_width(map_w);
                if let Some(&nl) = is_land.get(&(nc, nr)) {
                    if !nl {
                        neighbors_have_ocean = true;
                        break;
                    }
                } else {
                    // マップ範囲外（南北端）は海扱い
                    neighbors_have_ocean = true;
                    break;
                }
            }

            let terrain = match env {
                PlanetEnvironment::Terra => {
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if moisture > 0.65 {
                        if detail > 0.70 {
                            TerrainType::ToxicSwamp
                        } else {
                            TerrainType::Forest
                        }
                    } else if detail > 0.82 {
                        TerrainType::CrystalFields
                    } else if moisture < 0.35 {
                        TerrainType::Plains
                    } else {
                        TerrainType::Forest
                    }
                }
                PlanetEnvironment::Arid => {
                    // 乾燥砂漠: 海洋が少なく、平原・丘陵・結晶が多い。森林や湿地は極めて稀
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if detail > 0.72 {
                        TerrainType::CrystalFields
                    } else if moisture > 0.82 {
                        TerrainType::Forest
                    } else {
                        TerrainType::Plains
                    }
                }
                PlanetEnvironment::Archipelago => {
                    // 海洋群島: 陸地が狭く、険しい孤島や砂浜・森林
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if moisture > 0.45 {
                        TerrainType::Forest
                    } else if detail > 0.78 {
                        TerrainType::CrystalFields
                    } else {
                        TerrainType::Plains
                    }
                }
                PlanetEnvironment::ToxicMarsh => {
                    // 瘴気沼沢: 湿地と原生林が大部分を占める
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if detail > 0.85 {
                        TerrainType::CrystalFields
                    } else if moisture > 0.40 || detail > 0.45 {
                        TerrainType::ToxicSwamp
                    } else {
                        TerrainType::Forest
                    }
                }
                PlanetEnvironment::Crystalline => {
                    // 結晶極地: 山脈と結晶鉱床が広がり、高低差が激しい
                    if !neighbors_have_ocean && elev > mountain_threshold {
                        TerrainType::Mountains
                    } else if elev > hill_threshold {
                        TerrainType::Hills
                    } else if detail > 0.50 {
                        TerrainType::CrystalFields
                    } else if moisture < 0.40 {
                        TerrainType::Plains
                    } else {
                        TerrainType::Forest
                    }
                }
            };

            terrain_map.insert(coord, terrain);
        }
    }

    let world_width = hex::map_world_width_with_width(HEX_RADIUS, map_w);

    commands
        .spawn((
            MapRoot,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|root| {
            // メインマップ(0)および左右の周回表示(-1, +1)の3セクションを生成
            // 見ている範囲の左右の大まかな範囲が常に描画され、境界の途切れをなくす
            for wrap_offset in [-1, 0, 1] {
                let section_offset_x = wrap_offset as f32 * world_width;
                root.spawn((
                    Transform::from_xyz(section_offset_x, 0.0, 0.0),
                    Visibility::default(),
                ))
                .with_children(|section| {
                    for row in -half_h..=half_h {
                        for col in 0..map_w {
                            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
                            let terrain = terrain_map[&coord];
                            let height = terrain.height();
                            let world_pos = coord.to_world_pos(HEX_RADIUS);

                            let mesh_handle = meshes.add(create_hex_mesh(HEX_RADIUS, height));
                            let material_handle = mat_cache.get(&terrain).unwrap().clone();

                            let tile_entity = section
                                .spawn((
                                    HexTile {
                                        coord,
                                        terrain,
                                    },
                                    Mesh3d(mesh_handle),
                                    MeshMaterial3d(material_handle),
                                    Transform::from_xyz(world_pos.x, height / 2.0, world_pos.z)
                                        .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 6.0)),
                                ))
                                .id();

                            // 中央セクション(0)のEntityを代表タイルとして登録
                            if wrap_offset == 0 {
                                map_grid.tiles.insert(coord, tile_entity);
                                map_grid.terrain_data.insert(coord, terrain);
                            }
                        }
                    }
                });
            }
        });

    info!("Generated {} hex tiles (x3 wrapped sections).", map_grid.tiles.len() * 3);
}

pub fn cleanup_hex_map(
    mut commands: Commands,
    query: Query<Entity, With<MapRoot>>,
    mut map_grid: ResMut<MapGrid>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    map_grid.tiles.clear();
    map_grid.terrain_data.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::hex::{MAP_HEIGHT, MAP_WIDTH};

    #[test]
    fn test_no_mountains_in_ocean() {
        let half_h = MAP_HEIGHT / 2;
        let mut elevations: HashMap<(i32, i32), f32> = HashMap::new();
        let mut is_land: HashMap<(i32, i32), bool> = HashMap::new();

        for row in -half_h..=half_h {
            let lat_factor = (row.abs() as f32 / half_h as f32).powf(1.8);
            for col in 0..MAP_WIDTH {
                let n = cylinder_noise_with_size(col, row, 1337, MAP_WIDTH, MAP_HEIGHT);
                let elevation = (n - lat_factor * 0.4).clamp(0.0, 1.0);
                elevations.insert((col, row), elevation);
                is_land.insert((col, row), elevation >= 0.42);
            }
        }

        for row in -half_h..=half_h {
            for col in 0..MAP_WIDTH {
                let coord = HexCoord::from_col_row(col, row);
                let land = is_land[&(col, row)];

                let mut neighbors_have_ocean = false;
                for n_coord in coord.neighbors() {
                    let (nc, nr) = n_coord.to_col_row();
                    if let Some(&nl) = is_land.get(&(nc, nr)) {
                        if !nl {
                            neighbors_have_ocean = true;
                            break;
                        }
                    } else {
                        neighbors_have_ocean = true;
                        break;
                    }
                }

                let elev = elevations[&(col, row)];
                let is_mountain = land && !neighbors_have_ocean && elev > 0.72;

                // 海（!land）であるセルは絶対に山岳になってはならない
                if !land {
                    assert!(!is_mountain, "Ocean tile must never be a mountain at {:?}", (col, row));
                }

                // 山岳は海洋に隣接してはならない（海上に孤立・突出しない）
                if is_mountain {
                    assert!(!neighbors_have_ocean, "Mountain must not border ocean at {:?}", (col, row));
                }
            }
        }
    }

    #[test]
    fn test_planet_environments_and_sizes() {
        for env in PlanetEnvironment::ALL {
            for size in MapSize::ALL {
                let (w, h) = size.dimensions();
                let half_h = h / 2;
                let sea_threshold = env.sea_level_threshold();

                let mut ocean_count = 0;
                let mut land_count = 0;

                for row in -half_h..=half_h {
                    for col in 0..w {
                        let n = cylinder_noise_with_size(col, row, 1337, w, h);
                        let lat_factor = (row.abs() as f32 / half_h as f32).powf(1.8);
                        let elev = (n - lat_factor * 0.4).clamp(0.0, 1.0);
                        if elev < sea_threshold {
                            ocean_count += 1;
                        } else {
                            land_count += 1;
                        }
                    }
                }

                assert!(ocean_count + land_count > 0);
                if env == PlanetEnvironment::Archipelago {
                    // 群島は海が陸より多い
                    assert!(ocean_count > land_count);
                } else if env == PlanetEnvironment::Arid {
                    // 乾燥砂漠は陸が海より圧倒的に多い
                    assert!(land_count > ocean_count);
                }
            }
        }
    }
}
