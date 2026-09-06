use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::faction::PlayerFaction;
use crate::map::hex::HexCoord;
use crate::map::interaction::{HoveredTile, SelectedTile};
use crate::map::MapGrid;
use crate::state::AppState;

use super::types::{MoveTargetMarker, SelectedUnit, Unit, UnitSelectionRing};

pub struct UnitInteractionPlugin;

impl Plugin for UnitInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedUnit>()
            .init_resource::<ReachableTiles>()
            .add_systems(
                Update,
                (
                    handle_unit_selection_and_move,
                    update_reachable_tiles,
                    update_selection_visuals,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_selection_visuals);
    }
}

/// 選択中ユニットの到達可能タイルキャッシュ
#[derive(Resource, Default, Debug)]
pub struct ReachableTiles {
    pub tiles: HashMap<HexCoord, u32>, // HexCoord -> 消費移動力
}

/// ユニットの選択および移動先クリックを処理
#[allow(clippy::too_many_arguments)]
pub fn handle_unit_selection_and_move(
    mouse_button: Res<ButtonInput<MouseButton>>,
    hovered_tile: Res<HoveredTile>,
    mut selected_tile: ResMut<SelectedTile>,
    mut selected_unit: ResMut<SelectedUnit>,
    reachable_tiles: Res<ReachableTiles>,
    player_faction: Res<PlayerFaction>,
    map_grid: Res<MapGrid>,
    mut units: Query<(Entity, &mut Unit, &mut Transform)>,
) {
    // 右クリックまたは左クリックの処理
    let left_clicked = mouse_button.just_released(MouseButton::Left);
    let right_clicked = mouse_button.just_released(MouseButton::Right);

    if !left_clicked && !right_clicked {
        return;
    }

    let Some(hovered) = hovered_tile.0 else {
        return;
    };

    // 1. 右クリック時: 選択中ユニットがいて、到達可能タイルであれば即時移動
    if right_clicked {
        if let Some(selected_entity) = selected_unit.0
            && let Ok((_, mut unit, mut transform)) = units.get_mut(selected_entity)
            && let Some(&cost) = reachable_tiles.tiles.get(&hovered)
        {
            execute_unit_move(&mut unit, &mut transform, hovered, cost, &map_grid);
            selected_tile.0 = Some(hovered);
            return;
        }
        return;
    }

    // 2. 左クリック時:
    // まずクリックされたタイルに味方ユニット（プレイヤー派閥）がいるか探索
    let player_fac = player_faction.0;
    let clicked_unit_entity = units
        .iter()
        .find(|(_, u, _)| u.coord == hovered && u.faction == player_fac)
        .map(|(e, _, _)| e);

    if let Some(unit_e) = clicked_unit_entity {
        // 自軍ユニットを選択
        selected_unit.0 = Some(unit_e);
        selected_tile.0 = Some(hovered);
    } else if let Some(selected_entity) = selected_unit.0 {
        // すでに自軍ユニットが選択されており、到達可能タイルをクリックした場合は移動
        if let Some(&cost) = reachable_tiles.tiles.get(&hovered) {
            if let Ok((_, mut unit, mut transform)) = units.get_mut(selected_entity) {
                execute_unit_move(&mut unit, &mut transform, hovered, cost, &map_grid);
                selected_tile.0 = Some(hovered);
            }
        } else {
            // 到達不能タイルをクリックした場合はユニット選択解除（タイル選択のみ残す）
            selected_unit.0 = None;
        }
    }
}

/// ユニットの移動処理を実行
fn execute_unit_move(
    unit: &mut Unit,
    transform: &mut Transform,
    destination: HexCoord,
    cost: u32,
    map_grid: &MapGrid,
) {
    unit.coord = destination;
    unit.current_movement = unit.current_movement.saturating_sub(cost);
    if unit.current_movement == 0 {
        unit.is_exhausted = true;
    }

    let terrain_height = map_grid
        .terrain_data
        .get(&destination)
        .map(|t| t.height())
        .unwrap_or(0.0);

    let world_pos = destination.to_world_pos(crate::map::HEX_RADIUS);
    transform.translation.x = world_pos.x;
    transform.translation.y = terrain_height;
    transform.translation.z = world_pos.z;

    info!(
        "Unit {:?} moved to {:?}, remaining movement: {}",
        unit.group_type, destination, unit.current_movement
    );
}

/// 選択中ユニットの到達可能タイルをBFS探索して更新
pub fn update_reachable_tiles(
    selected_unit: Res<SelectedUnit>,
    units: Query<&Unit>,
    map_grid: Res<MapGrid>,
    mut reachable_tiles: ResMut<ReachableTiles>,
) {
    if !selected_unit.is_changed() {
        // ユニットの残り移動力変化なども反映したいが、主要なトリガーは選択変化
        return;
    }

    reachable_tiles.tiles.clear();

    let Some(unit_entity) = selected_unit.0 else {
        return;
    };

    let Ok(unit) = units.get(unit_entity) else {
        return;
    };

    if unit.current_movement == 0 {
        return;
    }

    let map_w = map_grid.width.max(1);
    let max_move = unit.current_movement;

    // BFS探索
    let mut queue = VecDeque::new();
    queue.push_back((unit.coord, 0));
    reachable_tiles.tiles.insert(unit.coord, 0);

    while let Some((curr, cost)) = queue.pop_front() {
        if cost >= max_move {
            continue;
        }

        for next_coord in curr.neighbors_with_width(map_w) {
            // 地形通行判定（陸上ユニットはPassableGroundのみ通行可能）
            let is_passable = map_grid
                .terrain_data
                .get(&next_coord)
                .map(|t| t.is_passable_ground())
                .unwrap_or(false);

            if !is_passable {
                continue;
            }

            let next_cost = cost + 1; // 現状は1タイルにつき移動コスト1
            if next_cost <= max_move {
                let recorded_cost = reachable_tiles.tiles.get(&next_coord).copied();
                if recorded_cost.is_none() || next_cost < recorded_cost.unwrap() {
                    reachable_tiles.tiles.insert(next_coord, next_cost);
                    queue.push_back((next_coord, next_cost));
                }
            }
        }
    }
}

/// 選択リングおよび移動可能範囲マーカーの3D描画更新
#[allow(clippy::too_many_arguments)]
pub fn update_selection_visuals(
    mut commands: Commands,
    selected_unit: Res<SelectedUnit>,
    reachable_tiles: Res<ReachableTiles>,
    units: Query<(&Unit, &Transform)>,
    map_grid: Res<MapGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing_rings: Query<Entity, With<UnitSelectionRing>>,
    existing_markers: Query<Entity, With<MoveTargetMarker>>,
) {
    if !selected_unit.is_changed() && !reachable_tiles.is_changed() {
        return;
    }

    // 既存のリングとマーカーを全削除
    for entity in &existing_rings {
        commands.entity(entity).despawn();
    }
    for entity in &existing_markers {
        commands.entity(entity).despawn();
    }

    let Some(selected_entity) = selected_unit.0 else {
        return;
    };

    let Ok((unit, transform)) = units.get(selected_entity) else {
        return;
    };

    // 1. ユニット足元に選択サークルリングをスポーン
    let ring_mesh = meshes.add(Torus::new(0.48, 0.04));
    let ring_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.95, 0.9),
        emissive: LinearRgba::rgb(0.4, 1.8, 1.6),
        unlit: true,
        ..default()
    });

    commands.spawn((
        UnitSelectionRing,
        Mesh3d(ring_mesh),
        MeshMaterial3d(ring_mat),
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y + 0.05,
            transform.translation.z,
        ),
    ));

    // 2. 到達可能タイルの上面に淡い移動マーカー（ドット/サークル）を配置
    let marker_mesh = meshes.add(Cylinder::new(0.22, 0.02));
    let marker_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.9, 0.8, 0.65),
        emissive: LinearRgba::rgb(0.1, 0.5, 0.4),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    for (&coord, &cost) in &reachable_tiles.tiles {
        if coord == unit.coord || cost == 0 {
            continue;
        }

        let height = map_grid
            .terrain_data
            .get(&coord)
            .map(|t| t.height())
            .unwrap_or(0.0);

        let pos = coord.to_world_pos(crate::map::HEX_RADIUS);

        commands.spawn((
            MoveTargetMarker { target_coord: coord },
            Mesh3d(marker_mesh.clone()),
            MeshMaterial3d(marker_mat.clone()),
            Transform::from_xyz(pos.x, height + 0.04, pos.z),
        ));
    }
}

pub fn cleanup_selection_visuals(
    mut commands: Commands,
    rings: Query<Entity, With<UnitSelectionRing>>,
    markers: Query<Entity, With<MoveTargetMarker>>,
) {
    for entity in &rings {
        commands.entity(entity).despawn();
    }
    for entity in &markers {
        commands.entity(entity).despawn();
    }
}
