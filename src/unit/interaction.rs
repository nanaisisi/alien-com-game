use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::faction::PlayerFaction;
use crate::map::hex::HexCoord;
use crate::map::interaction::{HoveredTile, SelectedTile};
use crate::map::MapGrid;
use crate::state::AppState;

use super::types::{
    MoveModeState, MoveTargetMarker, SelectedUnit, Unit, UnitActionState, UnitSelectionRing,
};

pub struct UnitInteractionPlugin;

impl Plugin for UnitInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedUnit>()
            .init_resource::<MoveModeState>()
            .init_resource::<ReachableTiles>()
            .add_systems(
                Update,
                (
                    handle_unit_keyboard_shortcuts,
                    handle_unit_selection_and_move,
                    update_reachable_tiles,
                    update_selection_visuals,
                    process_unit_turn_updates,
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

/// キーボードによるユニット操作（M: 移動モード／移動、Tab/Shift+Tab: ユニット巡回、Escape: 選択解除等）
#[allow(clippy::too_many_arguments)]
pub fn handle_unit_keyboard_shortcuts(
    mut commands: Commands,
    mut action_events: MessageReader<crate::map::input::InGameActionEvent>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selected_tile: ResMut<SelectedTile>,
    hovered_tile: Res<HoveredTile>,
    mut move_mode: ResMut<MoveModeState>,
    reachable_tiles: Res<ReachableTiles>,
    player_faction: Res<PlayerFaction>,
    map_grid: Res<MapGrid>,
    mut units: Query<(Entity, &mut Unit, &mut Transform)>,
    mut map_camera: Query<&mut crate::camera::MapCamera>,
    outposts_query: Query<(Entity, &crate::faction::FactionOutpost)>,
    mut faction_resources: ResMut<crate::faction::FactionResources>,
) {
    let player_fac = player_faction.0;

    for event in action_events.read() {
        match event.0 {
            // 1. 選択解除・キャンセル
            crate::map::input::InGameAction::CancelOrDeselect => {
                if move_mode.0 {
                    move_mode.0 = false;
                } else if selected_unit.0.is_some() {
                    selected_unit.0 = None;
                }
            }

            // 2. 移動モード切り替え／即時移動
            crate::map::input::InGameAction::UnitMove => {
                if let Some(selected_entity) = selected_unit.0
                    && let Ok((_, mut unit, mut transform)) = units.get_mut(selected_entity)
                {
                    if let Some(hovered) = hovered_tile.0
                        && let Some(&cost) = reachable_tiles.tiles.get(&hovered)
                        && hovered != unit.coord
                    {
                        execute_unit_move(&mut unit, &mut transform, hovered, cost, &map_grid);
                        selected_tile.0 = Some(hovered);
                        move_mode.0 = false;
                    } else {
                        move_mode.0 = !move_mode.0;
                        info!("Unit Move Mode: {}", if move_mode.0 { "ENABLED" } else { "DISABLED" });
                    }
                }
            }

            // 3. 防御態勢 (Fortify)
            crate::map::input::InGameAction::UnitFortify => {
                if let Some(selected_entity) = selected_unit.0
                    && let Ok((_, mut unit, _)) = units.get_mut(selected_entity)
                {
                    unit.action_state = UnitActionState::Fortified { turns: 0 };
                    unit.is_exhausted = true;
                    unit.current_movement = 0;
                    move_mode.0 = false;
                    info!("Unit {:?} fortified (Defend posture).", unit.group_type);
                }
                select_next_ready_unit(&units, player_fac, &mut selected_unit, &mut selected_tile, &mut map_camera);
            }

            // 4. 警戒監視 (Alert / Overwatch)
            crate::map::input::InGameAction::UnitAlert => {
                if let Some(selected_entity) = selected_unit.0
                    && let Ok((_, mut unit, _)) = units.get_mut(selected_entity)
                {
                    unit.action_state = UnitActionState::Alert;
                    unit.is_exhausted = true;
                    unit.current_movement = 0;
                    move_mode.0 = false;
                    info!("Unit {:?} set to Alert/Overwatch.", unit.group_type);
                }
                select_next_ready_unit(&units, player_fac, &mut selected_unit, &mut selected_tile, &mut map_camera);
            }

            // 5. 回復・修理 (Heal)
            crate::map::input::InGameAction::UnitHeal => {
                if let Some(selected_entity) = selected_unit.0
                    && let Ok((_, mut unit, _)) = units.get_mut(selected_entity)
                {
                    if unit.hp >= unit.max_hp {
                        info!("Unit {:?} is already at full HP.", unit.group_type);
                    } else {
                        unit.action_state = UnitActionState::Healing;
                        unit.is_exhausted = true;
                        unit.current_movement = 0;
                        move_mode.0 = false;
                        info!("Unit {:?} set to Healing mode (HP: {}/{}).", unit.group_type, unit.hp, unit.max_hp);
                        select_next_ready_unit(&units, player_fac, &mut selected_unit, &mut selected_tile, &mut map_camera);
                    }
                }
            }

            // 6. 休眠待機 (Sleep)
            crate::map::input::InGameAction::UnitSleep => {
                if let Some(selected_entity) = selected_unit.0
                    && let Ok((_, mut unit, _)) = units.get_mut(selected_entity)
                {
                    unit.action_state = UnitActionState::Sleeping;
                    unit.is_exhausted = true;
                    unit.current_movement = 0;
                    move_mode.0 = false;
                    info!("Unit {:?} is now Sleeping.", unit.group_type);
                }
                select_next_ready_unit(&units, player_fac, &mut selected_unit, &mut selected_tile, &mut map_camera);
            }

            // 7. 白兵突撃 (Charge / Attack)
            crate::map::input::InGameAction::UnitCharge => {
                if let Some(selected_entity) = selected_unit.0 {
                    execute_unit_attack_action(
                        selected_entity,
                        false,
                        &mut commands,
                        &mut units,
                        &map_grid,
                        player_fac,
                        &mut selected_unit,
                        &mut selected_tile,
                        &mut move_mode,
                    );
                }
            }

            // 8. 遠隔射撃 (Ranged Attack)
            crate::map::input::InGameAction::UnitRangedAttack => {
                if let Some(selected_entity) = selected_unit.0 {
                    execute_unit_attack_action(
                        selected_entity,
                        true,
                        &mut commands,
                        &mut units,
                        &map_grid,
                        player_fac,
                        &mut selected_unit,
                        &mut selected_tile,
                        &mut move_mode,
                    );
                }
            }

            // 9. 装備・物資移転 (Transfer Equipment)
            crate::map::input::InGameAction::UnitTransfer => {
                if let Some(selected_entity) = selected_unit.0 {
                    execute_unit_transfer_action(
                        selected_entity,
                        &mut units,
                        &outposts_query,
                        player_fac,
                        &map_grid,
                    );
                }
            }

            // 10. ターンスキップ (Wait)
            crate::map::input::InGameAction::UnitWait => {
                if let Some(selected_entity) = selected_unit.0
                    && let Ok((_, mut unit, _)) = units.get_mut(selected_entity)
                {
                    unit.is_exhausted = true;
                    unit.current_movement = 0;
                    unit.action_state = UnitActionState::Idle;
                    move_mode.0 = false;
                    info!("Unit {:?} turn skipped (waiting).", unit.group_type);
                }
                select_next_ready_unit(&units, player_fac, &mut selected_unit, &mut selected_tile, &mut map_camera);
            }

            // 11. 部隊解体 (Disband)
            crate::map::input::InGameAction::UnitDisband => {
                if let Some(selected_entity) = selected_unit.0 {
                    if let Ok((_, unit, _)) = units.get(selected_entity) {
                        let (prod_refund, energy_refund) = match unit.group_type {
                            super::types::CombatGroupType::Scout => (12, 6),
                            super::types::CombatGroupType::LightInfantry => (18, 10),
                            super::types::CombatGroupType::Colonist => (30, 0),
                        };
                        faction_resources.production += prod_refund;
                        faction_resources.energy += energy_refund;
                        info!(
                            "Unit {:?} disbanded. Refunded +{} production, +{} energy.",
                            unit.group_type, prod_refund, energy_refund
                        );
                    }
                    commands.entity(selected_entity).despawn();
                    selected_unit.0 = None;
                    move_mode.0 = false;
                }
            }

            // 12. 自軍ユニット巡回 (Next / Prev)
            crate::map::input::InGameAction::NextUnit | crate::map::input::InGameAction::PrevUnit => {
                let is_prev = event.0 == crate::map::input::InGameAction::PrevUnit;
                let mut player_units: Vec<(Entity, HexCoord, bool)> = units
                    .iter()
                    .filter(|(_, u, _)| u.faction == player_fac)
                    .map(|(e, u, _)| {
                        let is_ready = !u.is_exhausted
                            && u.current_movement > 0
                            && !matches!(u.action_state, UnitActionState::Sleeping | UnitActionState::Alert);
                        (e, u.coord, is_ready)
                    })
                    .collect();

                if !player_units.is_empty() {
                    player_units.sort_by(|a, b| {
                        b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0))
                    });

                    let current_idx = selected_unit.0.and_then(|current_e| {
                        player_units.iter().position(|(e, _, _)| *e == current_e)
                    });

                    let len = player_units.len();
                    let next_idx = match current_idx {
                        Some(idx) => {
                            if is_prev {
                                (idx + len - 1) % len
                            } else {
                                (idx + 1) % len
                            }
                        }
                        None => 0,
                    };

                    let (target_e, target_coord, _) = player_units[next_idx];
                    selected_unit.0 = Some(target_e);
                    selected_tile.0 = Some(target_coord);
                    move_mode.0 = false;

                    if let Ok(mut cam) = map_camera.single_mut() {
                        let world_pos = target_coord.to_world_pos(crate::map::HEX_RADIUS);
                        cam.target_focal_point.x = world_pos.x;
                        cam.target_focal_point.z = world_pos.z;
                    }
                }
            }

            // 13. 首都フォーカス
            crate::map::input::InGameAction::JumpToCapital => {
                let mut player_outposts: Vec<(Entity, HexCoord)> = outposts_query
                    .iter()
                    .filter(|(_, o)| o.faction == player_fac)
                    .map(|(e, o)| (e, o.coord))
                    .collect();
                player_outposts.sort_by_key(|(e, _)| *e);

                if let Some(&(_, capital_coord)) = player_outposts.first() {
                    selected_tile.0 = Some(capital_coord);
                    if let Ok(mut cam) = map_camera.single_mut() {
                        let world_pos = capital_coord.to_world_pos(crate::map::HEX_RADIUS);
                        cam.target_focal_point.x = world_pos.x;
                        cam.target_focal_point.z = world_pos.z;
                    }
                    info!("Camera focused on Capital Outpost at {:?}", capital_coord);
                }
            }

            // 14. 拠点巡回 (NextCity / PrevCity)
            crate::map::input::InGameAction::NextCity | crate::map::input::InGameAction::PrevCity => {
                let is_prev = event.0 == crate::map::input::InGameAction::PrevCity;
                let mut player_outposts: Vec<(Entity, HexCoord)> = outposts_query
                    .iter()
                    .filter(|(_, o)| o.faction == player_fac)
                    .map(|(e, o)| (e, o.coord))
                    .collect();
                player_outposts.sort_by_key(|(e, _)| *e);

                if !player_outposts.is_empty() {
                    let current_outpost_idx = selected_tile.0.and_then(|t| {
                        player_outposts.iter().position(|(_, c)| *c == t)
                    });

                    let len = player_outposts.len();
                    let next_idx = match current_outpost_idx {
                        Some(idx) => {
                            if is_prev {
                                (idx + len - 1) % len
                            } else {
                                (idx + 1) % len
                            }
                        }
                        None => 0,
                    };

                    let (_, target_coord) = player_outposts[next_idx];
                    selected_tile.0 = Some(target_coord);
                    if let Ok(mut cam) = map_camera.single_mut() {
                        let world_pos = target_coord.to_world_pos(crate::map::HEX_RADIUS);
                        cam.target_focal_point.x = world_pos.x;
                        cam.target_focal_point.z = world_pos.z;
                    }
                    info!("Navigated to Outpost at {:?}", target_coord);
                }
            }

            _ => {}
        }
    }
}

/// 次の行動可能な未処理ユニットへフォーカスを移すヘルパー
fn select_next_ready_unit(
    units: &Query<(Entity, &mut Unit, &mut Transform)>,
    player_fac: crate::faction::types::FactionId,
    selected_unit: &mut ResMut<SelectedUnit>,
    selected_tile: &mut ResMut<SelectedTile>,
    map_camera: &mut Query<&mut crate::camera::MapCamera>,
) {
    let mut ready_units: Vec<(Entity, HexCoord)> = units
        .iter()
        .filter(|(e, u, _)| {
            u.faction == player_fac
                && !u.is_exhausted
                && u.current_movement > 0
                && !matches!(u.action_state, UnitActionState::Sleeping | UnitActionState::Alert)
                && Some(*e) != selected_unit.0
        })
        .map(|(e, u, _)| (e, u.coord))
        .collect();
    ready_units.sort_by_key(|(e, _)| *e);

    if let Some(&(next_e, next_coord)) = ready_units.first() {
        selected_unit.0 = Some(next_e);
        selected_tile.0 = Some(next_coord);
        if let Ok(mut cam) = map_camera.single_mut() {
            let world_pos = next_coord.to_world_pos(crate::map::HEX_RADIUS);
            cam.target_focal_point.x = world_pos.x;
            cam.target_focal_point.z = world_pos.z;
        }
    } else {
        selected_unit.0 = None;
    }
}

/// ユニットの戦闘・突撃・遠隔攻撃を実行
#[allow(clippy::too_many_arguments)]
fn execute_unit_attack_action(
    attacker_entity: Entity,
    is_ranged: bool,
    commands: &mut Commands,
    units: &mut Query<(Entity, &mut Unit, &mut Transform)>,
    map_grid: &MapGrid,
    player_fac: crate::faction::types::FactionId,
    selected_unit: &mut ResMut<SelectedUnit>,
    selected_tile: &mut ResMut<SelectedTile>,
    move_mode: &mut ResMut<MoveModeState>,
) {
    let map_w = map_grid.width.max(1);

    // 攻撃側の座標・基本攻撃力を取得
    let Ok((_, attacker_unit, _)) = units.get(attacker_entity) else {
        return;
    };
    let attacker_coord = attacker_unit.coord;
    let attacker_power = attacker_unit.group_type.attack_power();
    let attacker_hp = attacker_unit.hp;
    let max_range = if is_ranged { 2 } else { 1 };

    if attacker_power == 0 {
        info!("This unit cannot attack.");
        return;
    }

    if attacker_unit.current_movement == 0 || attacker_unit.is_exhausted {
        info!("This unit has no movement points remaining.");
        return;
    }

    // 射程内の敵ユニットを検索
    let mut targets: Vec<(Entity, HexCoord, i32)> = Vec::new();
    for (target_e, target_u, _) in units.iter() {
        if target_u.faction != player_fac {
            let dist = attacker_coord.distance_with_width(target_u.coord, map_w);
            if dist <= max_range && dist > 0 {
                targets.push((target_e, target_u.coord, dist));
            }
        }
    }

    if targets.is_empty() {
        info!(
            "No enemy units within {} range (Max range: {}).",
            if is_ranged { "ranged" } else { "melee" },
            max_range
        );
        return;
    }

    // 最も近い敵をターゲット（同距離なら最初）
    targets.sort_by_key(|(_, _, d)| *d);
    let (target_e, target_coord, dist) = targets[0];

    // 戦闘計算
    let target_snapshot = if let Ok((_, target_unit, _)) = units.get(target_e) {
        let def_mult = target_unit.action_state.defense_multiplier();
        let target_power = target_unit.group_type.attack_power();
        let target_hp = target_unit.hp;
        Some((target_hp, target_power, def_mult))
    } else {
        None
    };

    let Some((target_hp, target_power, def_mult)) = target_snapshot else {
        return;
    };

    // 与ダメージ計算
    let charge_bonus = if !is_ranged && dist == 1 { 1.25 } else { 1.0 };
    let damage_to_target = ((attacker_power as f32) * (attacker_hp as f32 / 100.0) * charge_bonus / def_mult).ceil() as u32;
    let damage_to_target = damage_to_target.max(5);

    // 反撃ダメージ計算（遠隔射撃の場合は反撃なし、近接白兵の場合は反撃あり）
    let damage_to_attacker = if !is_ranged && target_power > 0 {
        let counter_dmg = ((target_power as f32) * (target_hp as f32 / 100.0) * 0.75).ceil() as u32;
        counter_dmg.max(3)
    } else {
        0
    };

    info!(
        "COMBAT: Attacker dealt {} dmg to Enemy (dist {}). Enemy counter dealt {} dmg.",
        damage_to_target, dist, damage_to_attacker
    );

    // 攻撃側のHPと移動力を反映
    let mut attacker_died = false;
    if let Ok((_, mut att_unit, _)) = units.get_mut(attacker_entity) {
        att_unit.current_movement = 0;
        att_unit.is_exhausted = true;
        att_unit.action_state = UnitActionState::Idle;
        if damage_to_attacker >= att_unit.hp {
            attacker_died = true;
        } else {
            att_unit.hp -= damage_to_attacker;
        }
    }

    // 防御側のHPを反映
    let mut target_died = false;
    if let Ok((_, mut tar_unit, _)) = units.get_mut(target_e) {
        if damage_to_target >= tar_unit.hp {
            target_died = true;
        } else {
            tar_unit.hp -= damage_to_target;
            // 警戒中だった敵が攻撃を受けたら警戒解除
            if matches!(tar_unit.action_state, UnitActionState::Alert | UnitActionState::Sleeping) {
                tar_unit.action_state = UnitActionState::Idle;
            }
        }
    }

    if target_died {
        commands.entity(target_e).despawn();
        info!("Enemy unit destroyed!");
        // 白兵突撃で敵を撃破した場合、そのタイルへ踏み込み移動
        if !is_ranged && !attacker_died
            && let Ok((_, mut att_unit, mut att_transform)) = units.get_mut(attacker_entity) {
                att_unit.coord = target_coord;
                let world_pos = target_coord.to_world_pos(crate::map::HEX_RADIUS);
                let height = map_grid.terrain_data.get(&target_coord).map(|t| t.height()).unwrap_or(0.0);
                att_transform.translation.x = world_pos.x;
                att_transform.translation.y = height;
                att_transform.translation.z = world_pos.z;
                selected_tile.0 = Some(target_coord);
            }
    }

    if attacker_died {
        commands.entity(attacker_entity).despawn();
        selected_unit.0 = None;
        info!("Friendly unit was destroyed in combat.");
    }

    move_mode.0 = false;
}

/// 装備・応急資材移転アクション
fn execute_unit_transfer_action(
    unit_entity: Entity,
    units: &mut Query<(Entity, &mut Unit, &mut Transform)>,
    outposts_query: &Query<(Entity, &crate::faction::FactionOutpost)>,
    player_fac: crate::faction::types::FactionId,
    map_grid: &MapGrid,
) {
    let map_w = map_grid.width.max(1);

    // ユニット座標・種別を先にイミュータブルに取得
    let Some((unit_coord, group_type)) = units
        .get(unit_entity)
        .ok()
        .map(|(_, u, _)| (u.coord, u.group_type))
    else {
        return;
    };

    // 1. 同一または隣接タイルに自軍拠点がある場合: 補給（HP全快＋移動力+1）
    let near_outpost = outposts_query.iter().any(|(_, o)| {
        o.faction == player_fac && unit_coord.distance_with_width(o.coord, map_w) <= 1
    });

    if near_outpost {
        if let Ok((_, mut unit, _)) = units.get_mut(unit_entity) {
            unit.hp = unit.max_hp;
            unit.current_movement = (unit.current_movement + 1).min(unit.max_movement);
            info!("Unit {:?} resupplied from nearby outpost (HP restored to full).", group_type);
        }
        return;
    }

    // 2. 隣接する傷ついた友軍部隊を検索
    let injured_ally = units
        .iter()
        .find(|(e, u, _)| {
            *e != unit_entity
                && u.faction == player_fac
                && u.hp < u.max_hp
                && unit_coord.distance_with_width(u.coord, map_w) <= 1
        })
        .map(|(e, _, _)| e);

    if let Some(ally_e) = injured_ally {
        // 先に味方のHPを回復
        if let Ok((_, mut ally, _)) = units.get_mut(ally_e) {
            let heal_amt = 20;
            ally.hp = (ally.hp + heal_amt).min(ally.max_hp);
            info!("Transferred emergency field repair kit to ally {:?} (+{} HP).", ally.group_type, heal_amt);
        }
        // 次に自部隊の移動力を消費
        if let Ok((_, mut unit, _)) = units.get_mut(unit_entity) {
            unit.current_movement = unit.current_movement.saturating_sub(1);
        }
    } else {
        info!("No nearby outpost or damaged ally within 1 hex to transfer equipment/supplies.");
    }
}

/// ユニットの選択および移動先クリックを処理
#[allow(clippy::too_many_arguments)]
pub fn handle_unit_selection_and_move(
    mouse_button: Res<ButtonInput<MouseButton>>,
    hovered_tile: Res<HoveredTile>,
    mut selected_tile: ResMut<SelectedTile>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut move_mode: ResMut<MoveModeState>,
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
            move_mode.0 = false;
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
        // 自軍ユニットを選択（移動モード中なら自軍別ユニット選択に切り替え）
        selected_unit.0 = Some(unit_e);
        selected_tile.0 = Some(hovered);
        move_mode.0 = false;
    } else if let Some(selected_entity) = selected_unit.0 {
        // すでに自軍ユニットが選択されており、到達可能タイルをクリックした場合は移動
        if let Some(&cost) = reachable_tiles.tiles.get(&hovered) {
            if let Ok((_, mut unit, mut transform)) = units.get_mut(selected_entity) {
                execute_unit_move(&mut unit, &mut transform, hovered, cost, &map_grid);
                selected_tile.0 = Some(hovered);
                move_mode.0 = false;
            }
        } else {
            // 到達不能タイルをクリックした場合はユニット選択解除（タイル選択のみ残す）
            selected_unit.0 = None;
            move_mode.0 = false;
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
    move_mode: Res<MoveModeState>,
    units: Query<(&Unit, &Transform)>,
    map_grid: Res<MapGrid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing_rings: Query<Entity, With<UnitSelectionRing>>,
    existing_markers: Query<Entity, With<MoveTargetMarker>>,
) {
    if !selected_unit.is_changed() && !reachable_tiles.is_changed() && !move_mode.is_changed() {
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

    // 1. ユニット足元に選択サークルリングをスポーン（移動モード時は黄色～ゴールドに発光）
    let ring_mesh = meshes.add(Torus::new(0.48, 0.04));
    let (ring_col, ring_emissive) = if move_mode.0 {
        (Color::srgb(1.0, 0.85, 0.2), LinearRgba::rgb(2.0, 1.6, 0.3))
    } else {
        (Color::srgb(0.2, 0.95, 0.9), LinearRgba::rgb(0.4, 1.8, 1.6))
    };

    let ring_mat = materials.add(StandardMaterial {
        base_color: ring_col,
        emissive: ring_emissive,
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

    // 2. 到達可能タイルの上面に移動マーカー（ドット/サークル）を配置
    let marker_mesh = meshes.add(Cylinder::new(0.22, 0.02));
    let (marker_col, marker_emissive) = if move_mode.0 {
        (Color::srgba(1.0, 0.85, 0.2, 0.85), LinearRgba::rgb(0.6, 0.5, 0.1))
    } else {
        (Color::srgba(0.2, 0.9, 0.8, 0.65), LinearRgba::rgb(0.1, 0.5, 0.4))
    };

    let marker_mat = materials.add(StandardMaterial {
        base_color: marker_col,
        emissive: marker_emissive,
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

/// ターン経過時のユニット状態更新（HP回復、防御ボーナス蓄積、敵接近検知による警戒・休眠自動解除）
pub fn process_unit_turn_updates(
    faction_resources: Res<crate::faction::FactionResources>,
    territory_map: Res<crate::faction::TerritoryMap>,
    map_grid: Res<MapGrid>,
    mut units: Query<(Entity, &mut Unit)>,
) {
    if !faction_resources.is_changed() {
        return;
    }

    let map_w = map_grid.width.max(1);

    // 敵ユニットの全座標一覧を事前取得（警戒・休眠解除用）
    let enemy_coords: Vec<(crate::faction::types::FactionId, HexCoord)> = units
        .iter()
        .map(|(_, u)| (u.faction, u.coord))
        .collect();

    for (_, mut unit) in &mut units {
        let fac = unit.faction;
        let coord = unit.coord;

        // 1. 移動力リセット
        unit.current_movement = unit.max_movement;
        unit.is_exhausted = false;

        // 2. 状態ごとのターン更新処理
        match unit.action_state {
            UnitActionState::Healing => {
                // 回復処理: 自軍領内なら+20、中立なら+10
                let is_friendly_territory = territory_map.get_owner(&coord) == Some(fac);
                let heal_amount = if is_friendly_territory { 20 } else { 10 };
                unit.hp = (unit.hp + heal_amount).min(unit.max_hp);
                info!(
                    "Unit {:?} healed +{} HP (Current: {}/{})",
                    unit.group_type, heal_amount, unit.hp, unit.max_hp
                );

                if unit.hp >= unit.max_hp {
                    unit.action_state = UnitActionState::Idle;
                    info!("Unit {:?} fully healed. Returned to Idle.", unit.group_type);
                } else {
                    // まだ回復中のため待機継続
                    unit.is_exhausted = true;
                    unit.current_movement = 0;
                }
            }
            UnitActionState::Fortified { turns } => {
                // 防御姿勢の維持（ターンカウント加算、最大2）
                unit.action_state = UnitActionState::Fortified {
                    turns: (turns + 1).min(2),
                };
                unit.is_exhausted = true;
                unit.current_movement = 0;
            }
            UnitActionState::Sleeping => {
                // 休眠待機: 視界内（2ヘクス内）に敵が接近したら自動起床
                let enemy_near = enemy_coords.iter().any(|&(enemy_fac, enemy_coord)| {
                    enemy_fac != fac && coord.distance_with_width(enemy_coord, map_w) <= 2
                });
                if enemy_near {
                    unit.action_state = UnitActionState::Idle;
                    info!("Sleeping unit {:?} woke up due to approaching enemy!", unit.group_type);
                } else {
                    unit.is_exhausted = true;
                    unit.current_movement = 0;
                }
            }
            UnitActionState::Alert => {
                // 警戒監視: 2ヘクス内に敵が入ったら自動覚醒
                let enemy_near = enemy_coords.iter().any(|&(enemy_fac, enemy_coord)| {
                    enemy_fac != fac && coord.distance_with_width(enemy_coord, map_w) <= 2
                });
                if enemy_near {
                    unit.action_state = UnitActionState::Idle;
                    info!("Alert unit {:?} detected enemy in range and woke up!", unit.group_type);
                } else {
                    unit.is_exhausted = true;
                    unit.current_movement = 0;
                }
            }
            UnitActionState::Idle => {}
        }
    }
}
