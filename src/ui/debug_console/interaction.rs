use bevy::ecs::system::SystemParam;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use super::command::{CommandContext, execute_command};
use super::types::*;
use super::view::{spawn_console_ui, spawn_warning_modal, update_console_log_view};
use crate::ui::theme::{BUTTON_HOVERED, BUTTON_NORMAL, BUTTON_PRESSED};

type WarningButtonQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static DebugWarningAction),
    (Changed<Interaction>, With<Button>),
>;

type CloseButtonQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<DebugConsoleCloseButton>),
>;

#[derive(SystemParam)]
pub struct ConsoleQueries<'w, 's> {
    pub console_root: Query<'w, 's, Entity, With<DebugConsoleRoot>>,
    pub log_container: Query<'w, 's, Entity, With<DebugConsoleLogContainer>>,
    pub log_items: Query<'w, 's, Entity, With<DebugConsoleLogItem>>,
    pub input_text: Query<'w, 's, &'static mut Text, With<DebugConsoleInputText>>,
}

#[derive(SystemParam)]
pub struct ConsoleKeyboardInput<'w, 's> {
    pub keys: Res<'w, ButtonInput<KeyCode>>,
    pub key_events: MessageReader<'w, 's, KeyboardInput>,
}

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
    // バッククォートキー押下判定
    let toggle_pressed = keys.just_pressed(KeyCode::Backquote);

    if !toggle_pressed {
        return;
    }

    // 既に警告モーダルが開いている場合 -> 閉じる
    if state.show_warning_modal {
        for entity in &modal_query {
            commands.entity(entity).despawn();
        }
        state.show_warning_modal = false;
        return;
    }

    // コンソールが開いている場合 -> 閉じる
    if state.is_open {
        for entity in &console_query {
            commands.entity(entity).despawn();
        }
        state.is_open = false;
        return;
    }

    // コンソールが閉じている場合:
    if !state.is_unlocked {
        // 未アンロック -> 警告モーダルを表示
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
    btn_query: WarningButtonQuery,
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
    mut btn_query: CloseButtonQuery,
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
    mut input: ConsoleKeyboardInput,
    mut state: ResMut<DebugConsoleState>,
    asset_server: Res<AssetServer>,
    mut ui: ConsoleQueries,
    mut cmd_ctx: CommandContext,
) {
    if !state.is_open {
        return;
    }

    // Esc で即座に閉じる
    if input.keys.just_pressed(KeyCode::Escape) {
        for entity in &ui.console_root {
            commands.entity(entity).despawn();
        }
        state.is_open = false;
        return;
    }

    // Enter でコマンド実行
    if input.keys.just_pressed(KeyCode::Enter) || input.keys.just_pressed(KeyCode::NumpadEnter) {
        let input_text = std::mem::take(&mut state.input_text);
        if !input_text.trim().is_empty() {
            state.history.push(input_text.clone());
            state.history_index = None;
            execute_command(&input_text, &mut state, &mut cmd_ctx);
            update_console_log_view(
                &mut commands,
                &asset_server,
                &state,
                &ui.log_container,
                &ui.log_items,
            );
        }
    }

    // Backspace で一文字削除
    if input.keys.just_pressed(KeyCode::Backspace) {
        state.input_text.pop();
    }

    // 上キー: 履歴を遡る
    if input.keys.just_pressed(KeyCode::ArrowUp) && !state.history.is_empty() {
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
    if input.keys.just_pressed(KeyCode::ArrowDown)
        && let Some(idx) = state.history_index
    {
        if idx + 1 < state.history.len() {
            state.history_index = Some(idx + 1);
            state.input_text = state.history[idx + 1].clone();
        } else {
            state.history_index = None;
            state.input_text.clear();
        }
    }

    // 文字入力受付 (KeyboardInput の text フィールドを利用)
    for ev in input.key_events.read() {
        if ev.state.is_pressed()
            && let Some(ref text) = ev.text
        {
            for ch in text.chars() {
                // バッククォートや制御文字を除外
                if ch != '`' && ch != '~' && !ch.is_control() {
                    state.input_text.push(ch);
                }
            }
        }
    }

    // 入力テキスト表示の更新
    for mut text in &mut ui.input_text {
        **text = format!("{}_", state.input_text);
    }
}
