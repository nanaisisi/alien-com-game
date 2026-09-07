use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use super::command::{execute_command, CommandContext};
use super::types::*;
use super::view::{spawn_console_ui, spawn_warning_modal, update_console_log_view};
use crate::ui::theme::{BUTTON_HOVERED, BUTTON_NORMAL, BUTTON_PRESSED};

/// 隠しデバッグトリガーエリアのクリック処理（5回クリックで発火）
pub fn handle_secret_trigger_click(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<DebugConsoleState>,
    asset_server: Res<AssetServer>,
    modal_query: Query<Entity, With<DebugWarningModal>>,
    console_query: Query<Entity, With<DebugConsoleRoot>>,
    trigger_query: Query<&Interaction, (Changed<Interaction>, With<DebugSecretTriggerArea>)>,
) {
    for interaction in &trigger_query {
        if *interaction == Interaction::Pressed {
            let triggered = state.register_secret_click(time.elapsed_secs());
            if triggered {
                // 既にモーダルが開いている場合 -> 閉じる
                if state.show_warning_modal {
                    for entity in &modal_query {
                        commands.entity(entity).despawn();
                    }
                    state.show_warning_modal = false;
                    return;
                }

                // 既にコンソールが開いている場合 -> 閉じる
                if state.is_open {
                    for entity in &console_query {
                        commands.entity(entity).despawn();
                    }
                    state.is_open = false;
                    return;
                }

                // まだアンロックされていない場合 -> 初回警告モーダルを表示
                if !state.is_unlocked {
                    state.show_warning_modal = true;
                    spawn_warning_modal(&mut commands, &asset_server);
                } else {
                    // アンロック済み -> コンソールを直接開く
                    state.is_open = true;
                    spawn_console_ui(&mut commands, &asset_server, &state);
                }
            }
        }
    }
}

/// バッククォート（`KeyCode::Backquote`）またはショートカット押下によるトグル・開閉制御
pub fn handle_toggle_debug_console(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugConsoleState>,
    asset_server: Res<AssetServer>,
    modal_query: Query<Entity, With<DebugWarningModal>>,
    console_query: Query<Entity, With<DebugConsoleRoot>>,
) {
    if !keys.just_pressed(KeyCode::Backquote) {
        return;
    }

    // 既にモーダルが開いている場合 -> 閉じる
    if state.show_warning_modal {
        for entity in &modal_query {
            commands.entity(entity).despawn();
        }
        state.show_warning_modal = false;
        return;
    }

    // 既にコンソールが開いている場合 -> 閉じる
    if state.is_open {
        for entity in &console_query {
            commands.entity(entity).despawn();
        }
        state.is_open = false;
        return;
    }

    // まだアンロック（承認）されていない場合 -> 初回警告モーダルを表示
    if !state.is_unlocked {
        state.show_warning_modal = true;
        spawn_warning_modal(&mut commands, &asset_server);
    } else {
        // アンロック済み -> コンソールを直接開く
        state.is_open = true;
        spawn_console_ui(&mut commands, &asset_server, &state);
    }
}

/// 警告モーダルのボタンおよびキー操作
pub fn handle_warning_modal_interaction(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<DebugConsoleState>,
    asset_server: Res<AssetServer>,
    modal_query: Query<Entity, With<DebugWarningModal>>,
    btn_query: Query<(&Interaction, &DebugWarningAction), (Changed<Interaction>, With<Button>)>,
) {
    if !state.show_warning_modal {
        return;
    }

    let mut enable = false;
    let mut cancel = false;

    // Esc でキャンセル
    if keys.just_pressed(KeyCode::Escape) {
        cancel = true;
    }
    // Enter で有効化
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        enable = true;
    }

    // ボタンクリック判定
    for (interaction, action) in &btn_query {
        if *interaction == Interaction::Pressed {
            match action {
                DebugWarningAction::Enable => enable = true,
                DebugWarningAction::Cancel => cancel = true,
            }
        }
    }

    if enable {
        for entity in &modal_query {
            commands.entity(entity).despawn();
        }
        state.show_warning_modal = false;
        state.is_unlocked = true;
        state.is_open = true;
        spawn_console_ui(&mut commands, &asset_server, &state);
    } else if cancel {
        for entity in &modal_query {
            commands.entity(entity).despawn();
        }
        state.show_warning_modal = false;
    }
}

/// コンソール右上の「×」閉じるボタン、およびホバー演出
pub fn handle_console_close_button(
    mut commands: Commands,
    mut state: ResMut<DebugConsoleState>,
    console_query: Query<Entity, With<DebugConsoleRoot>>,
    mut btn_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DebugConsoleCloseButton>),
    >,
) {
    if !state.is_open {
        return;
    }

    for (interaction, mut bg_color) in &mut btn_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(BUTTON_PRESSED);
                for entity in &console_query {
                    commands.entity(entity).despawn();
                }
                state.is_open = false;
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(BUTTON_HOVERED);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(BUTTON_NORMAL);
            }
        }
    }
}

/// コンソール表示中のキーボード入力（文字入力、Enter、Backspace、Esc、履歴）
pub fn handle_console_keyboard_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut key_events: MessageReader<KeyboardInput>,
    mut state: ResMut<DebugConsoleState>,
    asset_server: Res<AssetServer>,
    console_query: Query<Entity, With<DebugConsoleRoot>>,
    log_container_query: Query<Entity, With<DebugConsoleLogContainer>>,
    log_items_query: Query<Entity, With<DebugConsoleLogItem>>,
    mut input_text_query: Query<&mut Text, With<DebugConsoleInputText>>,
    mut cmd_ctx: CommandContext,
) {
    if !state.is_open {
        return;
    }

    // Esc で即座に閉じる
    if keys.just_pressed(KeyCode::Escape) {
        for entity in &console_query {
            commands.entity(entity).despawn();
        }
        state.is_open = false;
        return;
    }

    // Enter でコマンド実行
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
        let input = std::mem::take(&mut state.input_text);
        if !input.trim().is_empty() {
            state.history.push(input.clone());
            state.history_index = None;
            execute_command(&input, &mut state, &mut cmd_ctx);
            update_console_log_view(
                &mut commands,
                &asset_server,
                &state,
                &log_container_query,
                &log_items_query,
            );
        }
    }

    // Backspace で一文字削除
    if keys.just_pressed(KeyCode::Backspace) {
        state.input_text.pop();
    }

    // 上キー: 履歴を遡る
    if keys.just_pressed(KeyCode::ArrowUp) && !state.history.is_empty() {
        let new_idx = match state.history_index {
            Some(idx) => idx.saturating_sub(1),
            None => state.history.len().saturating_sub(1),
        };
        state.history_index = Some(new_idx);
        if let Some(hist_cmd) = state.history.get(new_idx) {
            state.input_text = hist_cmd.clone();
        }
    }

    // 下キー: 履歴を進める
    if keys.just_pressed(KeyCode::ArrowDown) {
        if let Some(idx) = state.history_index {
            if idx + 1 < state.history.len() {
                state.history_index = Some(idx + 1);
                state.input_text = state.history[idx + 1].clone();
            } else {
                state.history_index = None;
                state.input_text.clear();
            }
        }
    }

    // 文字入力受付 (KeyboardInput の text フィールドを利用)
    for ev in key_events.read() {
        if ev.state.is_pressed() {
            if let Some(ref text) = ev.text {
                for ch in text.chars() {
                    // バッククォートや制御文字を除外
                    if ch != '`' && ch != '~' && !ch.is_control() {
                        state.input_text.push(ch);
                    }
                }
            }
        }
    }

    // 入力テキスト表示の更新
    for mut text in &mut input_text_query {
        **text = format!("{}_", state.input_text);
    }
}
