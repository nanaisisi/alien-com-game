use bevy::prelude::*;
use std::collections::HashMap;

use crate::state::AppState;
use self::hex::HexCoord;
use self::terrain::TerrainType;

pub mod hex;
pub mod interaction;
pub mod terrain;

pub const HEX_RADIUS: f32 = 1.0;
pub const MAP_RADIUS: i32 = 7; // 半径7の六角形領域（約169タイル）

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

/// 疑似乱数/ハッシュ関数でプロシージャルに地形を決定
fn get_procedural_terrain(q: i32, r: i32) -> TerrainType {
    let dist = (q.abs() + r.abs() + (-q - r).abs()) / 2;
    // 外周付近は海洋や山岳にしやすい (オーバーフロー回避のため wrapping_mul を使用)
    let q_u = q as u32;
    let r_u = r as u32;
    let hash = q_u.wrapping_mul(374761393) ^ r_u.wrapping_mul(668265263);
    let val = (hash % 100) as usize;

    if dist >= MAP_RADIUS - 1 {
        if val < 60 {
            return TerrainType::Ocean;
        } else if val < 85 {
            return TerrainType::Mountains;
        }
    }

    match val {
        0..=35 => TerrainType::Plains,
        36..=50 => TerrainType::Forest,
        51..=65 => TerrainType::Hills,
        66..=75 => TerrainType::ToxicSwamp,
        76..=85 => TerrainType::CrystalFields,
        86..=93 => TerrainType::Mountains,
        _ => TerrainType::Ocean,
    }
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

fn generate_hex_map(
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

    info!("Generating Overworld Hex Map (Radius: {})...", MAP_RADIUS);

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

    commands
        .spawn((
            MapRoot,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|root| {
            // 各ヘックスを生成
            for q in -MAP_RADIUS..=MAP_RADIUS {
                let r1 = (-MAP_RADIUS).max(-q - MAP_RADIUS);
                let r2 = MAP_RADIUS.min(-q + MAP_RADIUS);
                for r in r1..=r2 {
                    let coord = HexCoord::new(q, r);
                    let terrain = get_procedural_terrain(q, r);
                    let height = terrain.height();
                    let world_pos = coord.to_world_pos(HEX_RADIUS);

                    // 六角柱メッシュ
                    let mesh_handle = meshes.add(create_hex_mesh(HEX_RADIUS, height));
                    let material_handle = mat_cache.get(&terrain).unwrap().clone();

                    let tile_entity = root
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

                    map_grid.tiles.insert(coord, tile_entity);
                    map_grid.terrain_data.insert(coord, terrain);
                }
            }
        });

    info!("Generated {} hex tiles.", map_grid.tiles.len());
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
