use bevy::prelude::*;

use crate::state::AppState;
use crate::ui::city::CityModalState;
use crate::ui::diplomacy::DiplomacyModalState;
use crate::unit::types::{MoveModeState, SelectedUnit};

/// インゲーム（マップ画面）で発生しうるキーボードアクション一覧
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameAction {
    // --- ターン・システム操作 ---
    EndTurn,
    ForceEndTurn,
    OpenPauseMenu,
    CancelOrDeselect,

    // --- マップ表示・レンズ ---
    ToggleGrid,
    ToggleYields,

    // --- ナビゲーション・画面開閉 ---
    NextUnit,
    PrevUnit,
    JumpToCapital,
    NextCity,
    PrevCity,
    ToggleCityModal,
    ToggleDiplomacyModal,

    // --- ユニット行動 (ユニット選択時) ---
    UnitMove,
    UnitFortify,
    UnitAlert,
    UnitHeal,
    UnitSleep,
    UnitCharge,
    UnitRangedAttack,
    UnitTransfer,
    UnitWait,
    UnitDisband,

    // --- 都市モーダル内ショートカット ---
    CityProduceScout,
    CityProduceInfantry,
    CityProduceColonist,
    CityUpgradeOutpost,
}

/// インゲームアクションの発火を通知するメッセージ
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct InGameActionEvent(pub InGameAction);

/// キー設定とコンテキスト判定を一元管理するリソース
#[derive(Resource, Default, Debug, Clone)]
pub struct InGameKeyBindings;

/// キー入力と現在のゲーム状態から、発火すべきアクションを判定する純粋ロジック
pub fn evaluate_ingame_actions(
    keys: &ButtonInput<KeyCode>,
    debug_active: bool,
    city_modal_open: bool,
    diplomacy_modal_open: bool,
    has_selected_unit: bool,
    is_move_mode: bool,
) -> Vec<InGameAction> {
    let mut actions = Vec::new();

    // 1. デバッグコンソールまたは警告モーダルがアクティブな場合は、全てのインゲームショートカットを遮断
    if debug_active {
        return actions;
    }

    let is_shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    // 2. モーダル表示中の処理（都市または外交）
    if city_modal_open || diplomacy_modal_open {
        if keys.just_pressed(KeyCode::Escape) {
            actions.push(InGameAction::CancelOrDeselect);
        }

        // 都市モーダル固有のキー操作
        if city_modal_open {
            if keys.just_pressed(KeyCode::KeyU) {
                actions.push(InGameAction::CityUpgradeOutpost);
            }
            if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::Numpad1) {
                actions.push(InGameAction::CityProduceScout);
            }
            if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::Numpad2) {
                actions.push(InGameAction::CityProduceInfantry);
            }
            if keys.just_pressed(KeyCode::Digit3) || keys.just_pressed(KeyCode::Numpad3) {
                actions.push(InGameAction::CityProduceColonist);
            }
            if keys.just_pressed(KeyCode::KeyC) {
                actions.push(InGameAction::ToggleCityModal);
            }
        }

        if diplomacy_modal_open && keys.just_pressed(KeyCode::KeyF) {
            actions.push(InGameAction::ToggleDiplomacyModal);
        }

        // モーダル表示中はマップ操作やターン送りは受け付けない
        return actions;
    }

    // 3. 通常マップ画面（モーダル非表示）
    // 3-A. [Escape] の優先度処理
    if keys.just_pressed(KeyCode::Escape) {
        if is_move_mode || has_selected_unit {
            actions.push(InGameAction::CancelOrDeselect);
        } else {
            actions.push(InGameAction::OpenPauseMenu);
        }
        return actions;
    }

    // 3-B. ターン送り（Shift+Enter, Enter, Space）
    let enter_pressed = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter);
    if enter_pressed && is_shift {
        actions.push(InGameAction::ForceEndTurn);
    } else if enter_pressed {
        actions.push(InGameAction::EndTurn);
    }

    // 3-C. ユニット選択中固有のアクション（Space, F, M, A, R, O, H, Z, T, Delete）
    if has_selected_unit {
        if keys.just_pressed(KeyCode::Space) {
            actions.push(InGameAction::UnitWait);
        }
        if keys.just_pressed(KeyCode::KeyF) {
            actions.push(InGameAction::UnitFortify);
        }
        if keys.just_pressed(KeyCode::KeyM) {
            actions.push(InGameAction::UnitMove);
        }
        if keys.just_pressed(KeyCode::KeyA) {
            actions.push(InGameAction::UnitCharge);
        }
        if keys.just_pressed(KeyCode::KeyR) {
            actions.push(InGameAction::UnitRangedAttack);
        }
        if keys.just_pressed(KeyCode::KeyO) {
            actions.push(InGameAction::UnitAlert);
        }
        if keys.just_pressed(KeyCode::KeyH) {
            actions.push(InGameAction::UnitHeal);
        }
        if keys.just_pressed(KeyCode::KeyZ) {
            actions.push(InGameAction::UnitSleep);
        }
        if keys.just_pressed(KeyCode::KeyT) {
            actions.push(InGameAction::UnitTransfer);
        }
        if keys.just_pressed(KeyCode::Delete) {
            actions.push(InGameAction::UnitDisband);
        }
    } else {
        // ユニット非選択時のみの Space / F
        if keys.just_pressed(KeyCode::Space) {
            actions.push(InGameAction::EndTurn);
        }
        if keys.just_pressed(KeyCode::KeyF) {
            actions.push(InGameAction::ToggleDiplomacyModal);
        }
    }

    // 3-D. 共通のマップ表示・ナビゲーション
    if keys.just_pressed(KeyCode::KeyC) {
        actions.push(InGameAction::ToggleCityModal);
    }
    if keys.just_pressed(KeyCode::KeyG) {
        actions.push(InGameAction::ToggleGrid);
    }
    if keys.just_pressed(KeyCode::KeyY) {
        actions.push(InGameAction::ToggleYields);
    }

    // 部隊巡回: Tab / Shift+Tab / . / ,
    let is_next_unit = (keys.just_pressed(KeyCode::Tab) && !is_shift) || keys.just_pressed(KeyCode::Period);
    let is_prev_unit = (keys.just_pressed(KeyCode::Tab) && is_shift) || keys.just_pressed(KeyCode::Comma);
    if is_next_unit {
        actions.push(InGameAction::NextUnit);
    } else if is_prev_unit {
        actions.push(InGameAction::PrevUnit);
    }

    // 首都フォーカス: Home / Backslash
    if keys.just_pressed(KeyCode::Home) || keys.just_pressed(KeyCode::Backslash) {
        actions.push(InGameAction::JumpToCapital);
    }

    // 拠点巡回: [ / ]
    if keys.just_pressed(KeyCode::BracketLeft) {
        actions.push(InGameAction::PrevCity);
    }
    if keys.just_pressed(KeyCode::BracketRight) {
        actions.push(InGameAction::NextCity);
    }

    actions
}

/// 毎フレームのキーボード入力を評価し、InGameActionEventを発火するBevyシステム
pub fn dispatch_map_keyboard_inputs(
    keys: Res<ButtonInput<KeyCode>>,
    debug_state: Option<Res<crate::ui::debug_console::DebugConsoleState>>,
    city_modal: Option<Res<CityModalState>>,
    diplomacy_modal: Option<Res<DiplomacyModalState>>,
    selected_unit: Option<Res<SelectedUnit>>,
    move_mode: Option<Res<MoveModeState>>,
    mut event_writer: MessageWriter<InGameActionEvent>,
) {
    let debug_active = debug_state
        .as_ref()
        .is_some_and(|s| s.is_open || s.show_warning_modal);
    let city_modal_open = city_modal.as_ref().is_some_and(|m| m.is_open);
    let diplomacy_modal_open = diplomacy_modal.as_ref().is_some_and(|m| m.is_open);
    let has_selected_unit = selected_unit.as_ref().is_some_and(|u| u.0.is_some());
    let is_move_mode = move_mode.as_ref().is_some_and(|m| m.0);

    let actions = evaluate_ingame_actions(
        &keys,
        debug_active,
        city_modal_open,
        diplomacy_modal_open,
        has_selected_unit,
        is_move_mode,
    );

    for action in actions {
        event_writer.write(InGameActionEvent(action));
    }
}

pub struct MapInputPlugin;

impl Plugin for MapInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InGameKeyBindings>()
            .add_message::<InGameActionEvent>()
            .add_systems(
                PreUpdate,
                dispatch_map_keyboard_inputs.run_if(in_state(AppState::InGame)),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_key_priority() {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Space);

        // 1. ユニット選択中 -> UnitWait
        let actions = evaluate_ingame_actions(&keys, false, false, false, true, false);
        assert_eq!(actions, vec![InGameAction::UnitWait]);

        // 2. ユニット非選択時 -> EndTurn
        let actions = evaluate_ingame_actions(&keys, false, false, false, false, false);
        assert_eq!(actions, vec![InGameAction::EndTurn]);
    }

    #[test]
    fn test_f_key_priority() {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::KeyF);

        // 1. ユニット選択中 -> UnitFortify
        let actions = evaluate_ingame_actions(&keys, false, false, false, true, false);
        assert_eq!(actions, vec![InGameAction::UnitFortify]);

        // 2. ユニット非選択時 -> ToggleDiplomacyModal
        let actions = evaluate_ingame_actions(&keys, false, false, false, false, false);
        assert_eq!(actions, vec![InGameAction::ToggleDiplomacyModal]);
    }

    #[test]
    fn test_escape_key_hierarchy() {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Escape);

        // モーダル表示中 -> CancelOrDeselect
        let actions = evaluate_ingame_actions(&keys, false, true, false, false, false);
        assert_eq!(actions, vec![InGameAction::CancelOrDeselect]);

        // ユニット選択中 -> CancelOrDeselect
        let actions = evaluate_ingame_actions(&keys, false, false, false, true, false);
        assert_eq!(actions, vec![InGameAction::CancelOrDeselect]);

        // 移動モード中 -> CancelOrDeselect
        let actions = evaluate_ingame_actions(&keys, false, false, false, false, true);
        assert_eq!(actions, vec![InGameAction::CancelOrDeselect]);

        // 何も選択・モーダルなし -> OpenPauseMenu
        let actions = evaluate_ingame_actions(&keys, false, false, false, false, false);
        assert_eq!(actions, vec![InGameAction::OpenPauseMenu]);
    }

    #[test]
    fn test_debug_active_blocks_shortcuts() {
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::Space);
        keys.press(KeyCode::KeyM);
        keys.press(KeyCode::KeyG);

        let actions = evaluate_ingame_actions(&keys, true, false, false, true, false);
        assert!(actions.is_empty());
    }
}
