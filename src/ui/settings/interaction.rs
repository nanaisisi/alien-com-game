use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::state::AppState;
use super::types::*;
use super::view::spawn_confirm_return_modal;

type SettingsButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static mut BackgroundColor, &'static mut BorderColor, &'static SettingsButtonAction),
    (Changed<Interaction>, With<Button>),
>;

type SettingsButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static SettingsButtonAction),
    (Changed<Interaction>, With<Button>),
>;

pub fn settings_button_interaction_system(
    mut query: SettingsButtonInteractionQuery,
) {
    for (interaction, mut bg_color, mut border_color, action) in &mut query {
        let is_danger = matches!(action, SettingsButtonAction::ConfirmReturnToTitle);
        match *interaction {
            Interaction::Pressed => {
                if is_danger {
                    *bg_color = BackgroundColor(DANGER_PRESSED_BUTTON);
                    *border_color = BorderColor::all(Color::WHITE);
                } else {
                    *bg_color = BackgroundColor(PRESSED_BUTTON);
                    *border_color = BorderColor::all(ACCENT_COLOR);
                }
            }
            Interaction::Hovered => {
                if is_danger {
                    *bg_color = BackgroundColor(DANGER_HOVERED_BUTTON);
                    *border_color = BorderColor::all(DANGER_BORDER);
                } else {
                    *bg_color = BackgroundColor(HOVERED_BUTTON);
                    *border_color = BorderColor::all(ACCENT_COLOR);
                }
            }
            Interaction::None => {
                if is_danger {
                    *bg_color = BackgroundColor(DANGER_NORMAL_BUTTON);
                    *border_color = BorderColor::all(DANGER_BORDER);
                } else {
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                    *border_color = BorderColor::all(BORDER_COLOR);
                }
            }
        }
    }
}

#[derive(SystemParam)]
pub struct SettingsButtonActionContext<'w, 's> {
    pub settings: ResMut<'w, GameSettings>,
    pub next_state: ResMut<'w, NextState<AppState>>,
    pub windows: Query<'w, 's, &'static mut Window>,
    pub asset_server: Res<'w, AssetServer>,
    pub modal_query: Query<'w, 's, Entity, With<ReturnToTitleConfirmModal>>,
    pub adapter_info: Option<Res<'w, bevy::render::renderer::RenderAdapterInfo>>,
    pub system_info: Option<Res<'w, bevy::diagnostic::SystemInfo>>,
}

pub fn settings_button_action_system(
    mut commands: Commands,
    interaction_query: SettingsButtonActionQuery,
    mut ctx: SettingsButtonActionContext,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                SettingsButtonAction::MasterVolumeDown => {
                    ctx.settings.master_volume = ctx.settings.master_volume.saturating_sub(10);
                }
                SettingsButtonAction::MasterVolumeUp => {
                    ctx.settings.master_volume = (ctx.settings.master_volume + 10).min(100);
                }
                SettingsButtonAction::BgmVolumeDown => {
                    ctx.settings.bgm_volume = ctx.settings.bgm_volume.saturating_sub(10);
                }
                SettingsButtonAction::BgmVolumeUp => {
                    ctx.settings.bgm_volume = (ctx.settings.bgm_volume + 10).min(100);
                }
                SettingsButtonAction::SfxVolumeDown => {
                    ctx.settings.sfx_volume = ctx.settings.sfx_volume.saturating_sub(10);
                }
                SettingsButtonAction::SfxVolumeUp => {
                    ctx.settings.sfx_volume = (ctx.settings.sfx_volume + 10).min(100);
                }
                SettingsButtonAction::ResolutionPrev => {
                    if ctx.settings.resolution_index == 0 {
                        ctx.settings.resolution_index = RESOLUTION_PRESETS.len() - 1;
                    } else {
                        ctx.settings.resolution_index -= 1;
                    }
                }
                SettingsButtonAction::ResolutionNext => {
                    ctx.settings.resolution_index =
                        (ctx.settings.resolution_index + 1) % RESOLUTION_PRESETS.len();
                }
                SettingsButtonAction::ToggleFullscreen => {
                    ctx.settings.fullscreen = !ctx.settings.fullscreen;
                }
                SettingsButtonAction::FpsLimitPrev => {
                    ctx.settings.fps_limit = ctx.settings.fps_limit.prev();
                }
                SettingsButtonAction::FpsLimitNext => {
                    ctx.settings.fps_limit = ctx.settings.fps_limit.next();
                }
                SettingsButtonAction::ToggleVsync => {
                    ctx.settings.vsync = !ctx.settings.vsync;
                }
                SettingsButtonAction::ToggleShadows => {
                    ctx.settings.shadows_enabled = !ctx.settings.shadows_enabled;
                }
                SettingsButtonAction::AntiAliasingPrev => {
                    ctx.settings.anti_aliasing = ctx.settings.anti_aliasing.prev();
                }
                SettingsButtonAction::AntiAliasingNext => {
                    ctx.settings.anti_aliasing = ctx.settings.anti_aliasing.next();
                }
                SettingsButtonAction::ResetDefaults => {
                    info!("Resetting settings to system environment default values...");
                    let defaults = GameSettings::default_for_environment(
                        ctx.adapter_info.as_deref(),
                        ctx.system_info.as_deref(),
                    );
                    ctx.settings.master_volume = defaults.master_volume;
                    ctx.settings.bgm_volume = defaults.bgm_volume;
                    ctx.settings.sfx_volume = defaults.sfx_volume;
                    ctx.settings.fullscreen = defaults.fullscreen;
                    ctx.settings.resolution_index = defaults.resolution_index;
                    ctx.settings.fps_limit = defaults.fps_limit;
                    ctx.settings.vsync = defaults.vsync;
                    ctx.settings.shadows_enabled = defaults.shadows_enabled;
                    ctx.settings.anti_aliasing = defaults.anti_aliasing;
                    if let Ok(mut window) = ctx.windows.single_mut() {
                        window.mode = bevy::window::WindowMode::Windowed;
                    }
                }
                SettingsButtonAction::ResumeGame => {
                    info!("Resuming game...");
                    ctx.next_state.set(AppState::InGame);
                }
                SettingsButtonAction::RequestReturnToTitle => {
                    info!("Requesting return to title (opening confirmation modal)...");
                    if ctx.modal_query.is_empty() {
                        spawn_confirm_return_modal(&mut commands, &ctx.asset_server);
                    }
                }
                SettingsButtonAction::ConfirmReturnToTitle => {
                    info!("Agreed to return to title. Discarding unsaved session...");
                    for entity in &ctx.modal_query {
                        commands.entity(entity).despawn();
                    }
                    ctx.settings.return_state = AppState::Title;
                    ctx.next_state.set(AppState::Title);
                }
                SettingsButtonAction::CancelReturnToTitle => {
                    info!("Cancelled return to title.");
                    for entity in &ctx.modal_query {
                        commands.entity(entity).despawn();
                    }
                }
                SettingsButtonAction::Back => {
                    info!("Returning to {:?}...", ctx.settings.return_state);
                    ctx.next_state.set(ctx.settings.return_state);
                }
            }
        }
    }
}

pub fn update_settings_display_system(
    settings: Res<GameSettings>,
    mut query: Query<(&mut Text, &SettingValueLabel)>,
) {
    if !settings.is_changed() {
        return;
    }

    for (mut text, marker) in &mut query {
        match marker {
            SettingValueLabel::MasterVolume => {
                **text = format!("{}%", settings.master_volume);
            }
            SettingValueLabel::BgmVolume => {
                **text = format!("{}%", settings.bgm_volume);
            }
            SettingValueLabel::SfxVolume => {
                **text = format!("{}%", settings.sfx_volume);
            }
            SettingValueLabel::Resolution => {
                let res = RESOLUTION_PRESETS[settings.resolution_index];
                **text = format!("{}x{}", res.0, res.1);
            }
            SettingValueLabel::Fullscreen => {
                **text = if settings.fullscreen {
                    "フルスクリーン".to_string()
                } else {
                    "ウィンドウ".to_string()
                };
            }
            SettingValueLabel::FpsLimit => {
                **text = settings.fps_limit.display_label().to_string();
            }
            SettingValueLabel::Vsync => {
                **text = if settings.vsync {
                    "有効 (ON)".to_string()
                } else {
                    "無効 (OFF)".to_string()
                };
            }
            SettingValueLabel::Shadows => {
                **text = if settings.shadows_enabled {
                    "有効 (ON)".to_string()
                } else {
                    "無効 (OFF)".to_string()
                };
            }
            SettingValueLabel::AntiAliasing => {
                **text = settings.anti_aliasing.display_label().to_string();
            }
        }
    }
}

#[derive(SystemParam)]
pub struct SettingsNavState<'w> {
    pub focus: ResMut<'w, SettingsNavFocus>,
    pub settings: ResMut<'w, GameSettings>,
    pub next_state: ResMut<'w, NextState<AppState>>,
    pub adapter_info: Option<Res<'w, bevy::render::renderer::RenderAdapterInfo>>,
    pub system_info: Option<Res<'w, bevy::diagnostic::SystemInfo>>,
}

/// 矢印キーおよび決定・キャンセルキーによる設定画面のキーボード操作システム
pub fn settings_keyboard_navigation_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: SettingsNavState,
    asset_server: Res<AssetServer>,
    modal_query: Query<Entity, With<ReturnToTitleConfirmModal>>,
) {
    let modal_open = !modal_query.is_empty();

    // ==========================================
    // 1. 確認モーダル表示中のキーボード操作
    // ==========================================
    if modal_open {
        if keys.just_pressed(KeyCode::ArrowLeft)
            || keys.just_pressed(KeyCode::ArrowRight)
            || keys.just_pressed(KeyCode::KeyA)
            || keys.just_pressed(KeyCode::KeyD)
            || keys.just_pressed(KeyCode::Tab)
        {
            state.focus.modal_item = match state.focus.modal_item {
                ModalFocusItem::Confirm => ModalFocusItem::Cancel,
                ModalFocusItem::Cancel => ModalFocusItem::Confirm,
            };
        }

        if keys.just_pressed(KeyCode::Escape) {
            info!("ESC pressed: Cancelling return to title.");
            for entity in &modal_query {
                commands.entity(entity).despawn();
            }
            state.focus.modal_item = ModalFocusItem::Cancel;
            return;
        }

        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
            match state.focus.modal_item {
                ModalFocusItem::Confirm => {
                    info!("Agreed to return to title. Discarding unsaved session...");
                    for entity in &modal_query {
                        commands.entity(entity).despawn();
                    }
                    state.settings.return_state = AppState::Title;
                    state.next_state.set(AppState::Title);
                }
                ModalFocusItem::Cancel => {
                    info!("Cancelled return to title.");
                    for entity in &modal_query {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
        return;
    }

    // ==========================================
    // 2. メイン設定画面でのキーボード操作
    // ==========================================
    let is_ingame = state.settings.return_state == AppState::InGame;

    let nav_items: &[SettingsFocusItem] = if is_ingame {
        &[
            SettingsFocusItem::MasterVolume,
            SettingsFocusItem::BgmVolume,
            SettingsFocusItem::SfxVolume,
            SettingsFocusItem::Resolution,
            SettingsFocusItem::Fullscreen,
            SettingsFocusItem::FpsLimit,
            SettingsFocusItem::Vsync,
            SettingsFocusItem::Shadows,
            SettingsFocusItem::AntiAliasing,
            SettingsFocusItem::ResetDefaults,
            SettingsFocusItem::Resume,
            SettingsFocusItem::ReturnToTitle,
        ]
    } else {
        &[
            SettingsFocusItem::MasterVolume,
            SettingsFocusItem::BgmVolume,
            SettingsFocusItem::SfxVolume,
            SettingsFocusItem::Resolution,
            SettingsFocusItem::Fullscreen,
            SettingsFocusItem::FpsLimit,
            SettingsFocusItem::Vsync,
            SettingsFocusItem::Shadows,
            SettingsFocusItem::AntiAliasing,
            SettingsFocusItem::ResetDefaults,
            SettingsFocusItem::Back,
        ]
    };

    let current_idx = nav_items
        .iter()
        .position(|&item| item == state.focus.current_item)
        .unwrap_or(0);

    // [↑] / [W] で上移動
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        let prev_idx = if current_idx == 0 {
            nav_items.len() - 1
        } else {
            current_idx - 1
        };
        state.focus.current_item = nav_items[prev_idx];
    }

    // [↓] / [S] で下移動
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        let next_idx = (current_idx + 1) % nav_items.len();
        state.focus.current_item = nav_items[next_idx];
    }

    // [←] / [A] で減少または切り替え
    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        match state.focus.current_item {
            SettingsFocusItem::MasterVolume => {
                state.settings.master_volume = state.settings.master_volume.saturating_sub(10);
            }
            SettingsFocusItem::BgmVolume => {
                state.settings.bgm_volume = state.settings.bgm_volume.saturating_sub(10);
            }
            SettingsFocusItem::SfxVolume => {
                state.settings.sfx_volume = state.settings.sfx_volume.saturating_sub(10);
            }
            SettingsFocusItem::Resolution => {
                if state.settings.resolution_index == 0 {
                    state.settings.resolution_index = RESOLUTION_PRESETS.len() - 1;
                } else {
                    state.settings.resolution_index -= 1;
                }
            }
            SettingsFocusItem::Fullscreen => {
                state.settings.fullscreen = !state.settings.fullscreen;
            }
            SettingsFocusItem::FpsLimit => {
                state.settings.fps_limit = state.settings.fps_limit.prev();
            }
            SettingsFocusItem::Vsync => {
                state.settings.vsync = !state.settings.vsync;
            }
            SettingsFocusItem::Shadows => {
                state.settings.shadows_enabled = !state.settings.shadows_enabled;
            }
            SettingsFocusItem::AntiAliasing => {
                state.settings.anti_aliasing = state.settings.anti_aliasing.prev();
            }
            SettingsFocusItem::ReturnToTitle => {
                state.focus.current_item = SettingsFocusItem::Resume;
            }
            _ => {}
        }
    }

    // [→] / [D] で増加または切り替え
    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        match state.focus.current_item {
            SettingsFocusItem::MasterVolume => {
                state.settings.master_volume = (state.settings.master_volume + 10).min(100);
            }
            SettingsFocusItem::BgmVolume => {
                state.settings.bgm_volume = (state.settings.bgm_volume + 10).min(100);
            }
            SettingsFocusItem::SfxVolume => {
                state.settings.sfx_volume = (state.settings.sfx_volume + 10).min(100);
            }
            SettingsFocusItem::Resolution => {
                state.settings.resolution_index =
                    (state.settings.resolution_index + 1) % RESOLUTION_PRESETS.len();
            }
            SettingsFocusItem::Fullscreen => {
                state.settings.fullscreen = !state.settings.fullscreen;
            }
            SettingsFocusItem::FpsLimit => {
                state.settings.fps_limit = state.settings.fps_limit.next();
            }
            SettingsFocusItem::Vsync => {
                state.settings.vsync = !state.settings.vsync;
            }
            SettingsFocusItem::Shadows => {
                state.settings.shadows_enabled = !state.settings.shadows_enabled;
            }
            SettingsFocusItem::AntiAliasing => {
                state.settings.anti_aliasing = state.settings.anti_aliasing.next();
            }
            SettingsFocusItem::Resume => {
                state.focus.current_item = SettingsFocusItem::ReturnToTitle;
            }
            _ => {}
        }
    }

    // [Enter] / [Space] 決定キー
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        match state.focus.current_item {
            SettingsFocusItem::Resolution => {
                state.settings.resolution_index =
                    (state.settings.resolution_index + 1) % RESOLUTION_PRESETS.len();
            }
            SettingsFocusItem::Fullscreen => {
                state.settings.fullscreen = !state.settings.fullscreen;
            }
            SettingsFocusItem::FpsLimit => {
                state.settings.fps_limit = state.settings.fps_limit.next();
            }
            SettingsFocusItem::Vsync => {
                state.settings.vsync = !state.settings.vsync;
            }
            SettingsFocusItem::Shadows => {
                state.settings.shadows_enabled = !state.settings.shadows_enabled;
            }
            SettingsFocusItem::AntiAliasing => {
                state.settings.anti_aliasing = state.settings.anti_aliasing.next();
            }
            SettingsFocusItem::ResetDefaults => {
                info!("Resetting settings to system environment default values via Enter/Space...");
                let defaults = GameSettings::default_for_environment(
                    state.adapter_info.as_deref(),
                    state.system_info.as_deref(),
                );
                state.settings.master_volume = defaults.master_volume;
                state.settings.bgm_volume = defaults.bgm_volume;
                state.settings.sfx_volume = defaults.sfx_volume;
                state.settings.fullscreen = defaults.fullscreen;
                state.settings.resolution_index = defaults.resolution_index;
                state.settings.fps_limit = defaults.fps_limit;
                state.settings.vsync = defaults.vsync;
                state.settings.shadows_enabled = defaults.shadows_enabled;
                state.settings.anti_aliasing = defaults.anti_aliasing;
            }
            SettingsFocusItem::Resume => {
                info!("Resuming game via Enter/Space...");
                state.next_state.set(AppState::InGame);
            }
            SettingsFocusItem::ReturnToTitle => {
                info!("Requesting return to title via Enter/Space (opening modal)...");
                state.focus.modal_item = ModalFocusItem::Cancel;
                spawn_confirm_return_modal(&mut commands, &asset_server);
            }
            SettingsFocusItem::Back => {
                info!("Returning to {:?} via Enter/Space...", state.settings.return_state);
                state.next_state.set(state.settings.return_state);
            }
            _ => {}
        }
    }

    // [Escape] キャンセル/戻るキー
    if keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed: Returning to {:?}...", state.settings.return_state);
        state.next_state.set(state.settings.return_state);
    }
}

/// フォーカスされている設定行やモーダルボタンのボーダー/背景ハイライト
pub fn update_settings_focus_highlight_system(
    focus: Res<SettingsNavFocus>,
    modal_query: Query<Entity, With<ReturnToTitleConfirmModal>>,
    mut row_query: Query<
        (&SettingsNavRow, &mut BorderColor, &mut BackgroundColor, Option<&Button>),
        Without<ModalNavButton>,
    >,
    mut modal_btn_query: Query<
        (&ModalNavButton, &mut BorderColor, &mut BackgroundColor),
        Without<SettingsNavRow>,
    >,
) {
    let modal_open = !modal_query.is_empty();

    if modal_open {
        for (modal_btn, mut border_color, mut bg_color) in &mut modal_btn_query {
            let is_focused = focus.modal_item == modal_btn.0;
            match modal_btn.0 {
                ModalFocusItem::Confirm => {
                    if is_focused {
                        *border_color = BorderColor::all(Color::WHITE);
                        *bg_color = BackgroundColor(DANGER_HOVERED_BUTTON);
                    } else {
                        *border_color = BorderColor::all(DANGER_BORDER);
                        *bg_color = BackgroundColor(DANGER_NORMAL_BUTTON);
                    }
                }
                ModalFocusItem::Cancel => {
                    if is_focused {
                        *border_color = BorderColor::all(ACCENT_COLOR);
                        *bg_color = BackgroundColor(HOVERED_BUTTON);
                    } else {
                        *border_color = BorderColor::all(BORDER_COLOR);
                        *bg_color = BackgroundColor(NORMAL_BUTTON);
                    }
                }
            }
        }
    } else {
        for (nav_row, mut border_color, mut bg_color, opt_button) in &mut row_query {
            let is_focused = focus.current_item == nav_row.0;

            if opt_button.is_some() {
                let is_danger = nav_row.0 == SettingsFocusItem::ReturnToTitle;
                if is_danger {
                    if is_focused {
                        *border_color = BorderColor::all(DANGER_BORDER);
                        *bg_color = BackgroundColor(DANGER_HOVERED_BUTTON);
                    } else {
                        *border_color = BorderColor::all(Color::srgb(0.55, 0.25, 0.25));
                        *bg_color = BackgroundColor(NORMAL_BUTTON);
                    }
                } else if is_focused {
                    *border_color = BorderColor::all(ACCENT_COLOR);
                    *bg_color = BackgroundColor(HOVERED_BUTTON);
                } else {
                    *border_color = BorderColor::all(BORDER_COLOR);
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                }
            } else {
                if is_focused {
                    *border_color = BorderColor::all(ACCENT_COLOR);
                    *bg_color = BackgroundColor(Color::srgba(0.14, 0.20, 0.30, 0.90));
                } else {
                    *border_color = BorderColor::all(Color::NONE);
                    *bg_color = BackgroundColor(ROW_BG);
                }
            }
        }
    }
}
