use bevy::prelude::*;
use std::collections::HashMap;

use crate::state::AppState;
use self::hex::HexCoord;
use self::terrain::TerrainType;

pub mod hex;
pub mod interaction;
pub mod terrain;

use self::hex::{MAP_HEIGHT, MAP_WIDTH};
pub use self::hex::{MAP_HEIGHT as GRID_HEIGHT, MAP_WIDTH as GRID_WIDTH};

pub const HEX_RADIUS: f32 = 1.0;

/// マップ全体の状態を保持するリソース
#[derive(Resource, Default)]
pub struct MapGrid {
    pub tiles: HashMap<HexCoord, Entity>,
    pub terrain_data: HashMap<HexCoord, TerrainType>,
}

#[derive(Component)]
pub struct HexTile {
    pub coord: HexCoord,
    pub terrain: TerrainType,
}

#[derive(Component)]
pub struct MapRoot;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapGrid>()
            .add_plugins(interaction::MapInteractionPlugin)
            .add_systems(OnEnter(AppState::InGame), generate_hex_map)
            .add_systems(OnEnter(AppState::Title), cleanup_hex_map);
    }
}

/// 周期的な円筒座標におけるハッシュ/疑似乱数
fn cylinder_noise(col: i32, row: i32, seed: u32) -> f32 {
    use std::f32::consts::PI;
    let theta = (col as f32 / MAP_WIDTH as f32) * 2.0 * PI;
    let cx = theta.cos();
    let cz = theta.sin();
    let cy = row as f32 / (MAP_HEIGHT as f32 / 2.0);

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
    mut map_grid: ResMut<MapGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // すでにマップが生成されている場合（PauseMenu / Settings からの復帰時など）は再生成しない
    if !map_grid.tiles.is_empty() {
        info!("Hex map already exists. Skipping map generation.");
        return;
    }

    info!(
        "Generating Cylindrical Overworld Hex Map (Width: {}, Height: {})...",
        MAP_WIDTH, MAP_HEIGHT
    );

    map_grid.tiles.clear();
    map_grid.terrain_data.clear();

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
    // 南北の極地 (row が極端に端) は海洋に偏らせる
    let half_h = MAP_HEIGHT / 2;
    let mut elevations: HashMap<(i32, i32), f32> = HashMap::new();
    let mut is_land: HashMap<(i32, i32), bool> = HashMap::new();

    for row in -half_h..=half_h {
        let lat_factor = (row.abs() as f32 / half_h as f32).powf(1.8);
        for col in 0..MAP_WIDTH {
            let n = cylinder_noise(col, row, 1337);
            // 極地は海洋確率を上げるため標高を低減
            let elevation = (n - lat_factor * 0.4).clamp(0.0, 1.0);
            elevations.insert((col, row), elevation);
            // 標高 0.42 未満は海洋 (Ocean)
            is_land.insert((col, row), elevation >= 0.42);
        }
    }

    // Step 2: 地形タイプの決定
    // 【重要】山岳(Mountains)は「海ではない」かつ「十分な標高」かつ「海岸線から一定以上離れた陸地」にのみ生成
    let mut terrain_map: HashMap<HexCoord, TerrainType> = HashMap::new();

    for row in -half_h..=half_h {
        for col in 0..MAP_WIDTH {
            let coord = HexCoord::from_col_row(col, row);
            let land = is_land[&(col, row)];

            if !land {
                terrain_map.insert(coord, TerrainType::Ocean);
                continue;
            }

            let elev = elevations[&(col, row)];
            let moisture = cylinder_noise(col, row, 9999);
            let detail = cylinder_noise(col, row, 54321);

            // 隣接セルに海洋があるかチェック (海岸線判定)
            let mut neighbors_have_ocean = false;
            for n_coord in coord.neighbors() {
                let (nc, nr) = n_coord.to_col_row();
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

            let terrain = if !neighbors_have_ocean && elev > 0.72 {
                // 海岸線に面しておらず、標高が高い内陸部のみ山岳に
                TerrainType::Mountains
            } else if elev > 0.60 {
                // やや標高が高い場所は丘陵
                TerrainType::Hills
            } else if moisture > 0.65 {
                // 湿潤・水気のある場所
                if detail > 0.70 {
                    TerrainType::ToxicSwamp
                } else {
                    TerrainType::Forest
                }
            } else if detail > 0.82 {
                // 特殊資源地帯
                TerrainType::CrystalFields
            } else if moisture < 0.35 {
                // 乾燥・平原
                TerrainType::Plains
            } else {
                TerrainType::Forest
            };

            terrain_map.insert(coord, terrain);
        }
    }

    let world_width = hex::map_world_width(HEX_RADIUS);

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
                        for col in 0..MAP_WIDTH {
                            let coord = HexCoord::from_col_row(col, row);
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

fn cleanup_hex_map(
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

    #[test]
    fn test_no_mountains_in_ocean() {
        let half_h = MAP_HEIGHT / 2;
        let mut elevations: HashMap<(i32, i32), f32> = HashMap::new();
        let mut is_land: HashMap<(i32, i32), bool> = HashMap::new();

        for row in -half_h..=half_h {
            let lat_factor = (row.abs() as f32 / half_h as f32).powf(1.8);
            for col in 0..MAP_WIDTH {
                let n = cylinder_noise(col, row, 1337);
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
}
