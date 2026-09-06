use bevy::prelude::*;

use crate::faction::FactionOutpost;
use crate::map::MapGrid;

use super::mesh::spawn_unit_model;
use super::types::{CombatGroupType, SelectedUnit, Unit};

/// 初期ゲーム開始時、各派閥の前哨基地周辺に初期戦闘団をスポーンする
pub fn spawn_initial_units(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map_grid: Res<MapGrid>,
    outposts: Query<(&FactionOutpost, &Transform)>,
    existing_units: Query<Entity, With<Unit>>,
) {
    if !existing_units.is_empty() {
        return;
    }

    if outposts.is_empty() || map_grid.tiles.is_empty() {
        return;
    }

    let map_w = map_grid.width.max(1);

    info!("Spawning initial combat groups for all factions...");

    for (outpost, _outpost_transform) in &outposts {
        let faction = outpost.faction;
        let base_coord = outpost.coord;

        // 前哨基地の隣接タイル候補を取得
        let neighbors = base_coord.neighbors_with_width(map_w);

        // 3種類の初期戦闘団
        let initial_groups = [
            CombatGroupType::Scout,
            CombatGroupType::Colonist,
            CombatGroupType::LightInfantry,
        ];

        let mut spawned_coords = Vec::new();

        for (idx, &group_type) in initial_groups.iter().enumerate() {
            // 通行可能な隣接タイルを探す（なければ前哨基地と同じタイルまたは次候補）
            let mut target_coord = base_coord;
            for n_idx in idx..(idx + 6) {
                let candidate = neighbors[n_idx % 6];
                if let Some(&terrain) = map_grid.terrain_data.get(&candidate)
                    && terrain.is_passable_ground()
                    && !spawned_coords.contains(&candidate)
                {
                    target_coord = candidate;
                    break;
                }
            }
            spawned_coords.push(target_coord);

            // ユニットエンティティのスポーン
            let terrain_height = map_grid
                .terrain_data
                .get(&target_coord)
                .map(|t| t.height())
                .unwrap_or(0.0);

            let world_pos = target_coord.to_world_pos(crate::map::HEX_RADIUS);

            let unit_entity = commands
                .spawn((
                    Unit::new(faction, group_type, target_coord),
                    Transform::from_xyz(world_pos.x, terrain_height, world_pos.z),
                    Visibility::default(),
                ))
                .id();

            // SFプロシージャルメッシュの子階層を追加
            spawn_unit_model(
                &mut commands,
                unit_entity,
                &mut meshes,
                &mut materials,
                faction,
                group_type,
            );
        }
    }
}

/// タイトル画面等への遷移時に全ユニットをクリーンアップ
pub fn cleanup_units(
    mut commands: Commands,
    units: Query<Entity, With<Unit>>,
    mut selected_unit: ResMut<SelectedUnit>,
) {
    selected_unit.0 = None;
    for entity in &units {
        commands.entity(entity).despawn();
    }
}
