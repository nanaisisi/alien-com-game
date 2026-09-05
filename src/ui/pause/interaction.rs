use bevy::app::AppExit;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::types::*;
use super::view::spawn_pause_confirm_modal;
use crate::state::AppState;
use crate::ui::settings::GameSettings;
use crate::ui::theme::{
    ACCENT_COLOR, BORDER_COLOR, BORDER_DANGER, BUTTON_DANGER_HOVERED, BUTTON_DANGER_NORMAL,
    BUTTON_DANGER_PRESSED, BUTTON_HOVERED, BUTTON_NORMAL, BUTTON_PRESSED,
};

#[derive(SystemParam)]
pub struct PauseNavContext<'w, 's> {
    commands: Commands<'w, 's>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    focus: ResMut<'w, PauseMenuFocus>,
    next_state: ResMut<'w, NextState<AppState>>,
    settings: ResMut<'w, GameSettings>,
    exit_events: MessageWriter<'w, AppExit>,
    asset_server: Res<'w, AssetServer>,
    modal_query: Query<'w, 's, Entity, With<PauseConfirmModal>>,
}

pub fn pause_keyboard_navigation_system(mut ctx: PauseNavContext) {
    let PauseNavContext {
        ref mut commands,
        ref keys,
        ref mut focus,
        ref mut next_state,
        ref mut settings,
        ref mut exit_events,
        ref asset_server,
        ref modal_query,
    } = ctx;

    // ==========================================
    // 1. モーダル表示中のキーボード操作
    // ==========================================
    if focus.modal_open {
        if keys.just_pressed(KeyCode::ArrowLeft)
            || keys.just_pressed(KeyCode::ArrowRight)
            || keys.just_pressed(KeyCode::KeyA)
            || keys.just_pressed(KeyCode::KeyD)
            || keys.just_pressed(KeyCode::Tab)
        {
            focus.modal_focus = match focus.modal_focus {
                PauseModalFocusItem::Confirm => PauseModalFocusItem::Cancel,
                PauseModalFocusItem::Cancel => PauseModalFocusItem::Confirm,
            };
        }

        // ESC: 中止
        if keys.just_pressed(KeyCode::Escape) {
            info!("ESC pressed in PauseModal: Cancelling modal.");
            for entity in modal_query {
                commands.entity(entity).despawn();
            }
            focus.modal_open = false;
            focus.modal_focus = PauseModalFocusItem::Cancel;
            return;
        }

        // Enter / Space: 決定
        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
            match focus.modal_focus {
                PauseModalFocusItem::Confirm => {
                    for entity in modal_query {
                        commands.entity(entity).despawn();
                    }
                    focus.modal_open = false;
                    match focus.modal_type {
                        PauseModalType::ReturnToTitle => {
                            info!("Agreed to return to title. Transitioning to Title...");
                            settings.return_state = AppState::Title;
                            next_state.set(AppState::Title);
                        }
                        PauseModalType::QuitToDesktop => {
                            info!("Agreed to quit to desktop. Exiting game...");
                            exit_events.write(AppExit::Success);
                        }
                    }
                }
                PauseModalFocusItem::Cancel => {
                    info!("Cancelled pause modal.");
                    for entity in modal_query {
                        commands.entity(entity).despawn();
                    }
                    focus.modal_open = false;
                }
            }
        }
        return;
    }

    // ==========================================
    // 2. メインポーズメニューのキーボード操作
    // ==========================================
    let current_idx = PAUSE_MENU_ITEMS
        .iter()
        .position(|&item| item == focus.current_item)
        .unwrap_or(0);

    // [↑] / [W]
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        let prev_idx = if current_idx == 0 {
            PAUSE_MENU_ITEMS.len() - 1
        } else {
            current_idx - 1
        };
        focus.current_item = PAUSE_MENU_ITEMS[prev_idx];
    }

    // [↓] / [S]
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        let next_idx = (current_idx + 1) % PAUSE_MENU_ITEMS.len();
        focus.current_item = PAUSE_MENU_ITEMS[next_idx];
    }

    // [Enter] / [Space]
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        match focus.current_item {
            PauseMenuItem::Resume => {
                info!("Resuming game via Enter/Space...");
                next_state.set(AppState::InGame);
            }
            PauseMenuItem::Settings => {
                info!("Opening Settings from Pause Menu via Enter/Space...");
                settings.return_state = AppState::PauseMenu;
                next_state.set(AppState::Settings);
            }
            PauseMenuItem::ReturnToTitle => {
                info!("Requesting return to title via Enter/Space...");
                focus.modal_open = true;
                focus.modal_type = PauseModalType::ReturnToTitle;
                focus.modal_focus = PauseModalFocusItem::Cancel;
                spawn_pause_confirm_modal(commands, asset_server, PauseModalType::ReturnToTitle);
            }
            PauseMenuItem::QuitToDesktop => {
                info!("Requesting quit to desktop via Enter/Space...");
                focus.modal_open = true;
                focus.modal_type = PauseModalType::QuitToDesktop;
                focus.modal_focus = PauseModalFocusItem::Cancel;
                spawn_pause_confirm_modal(commands, asset_server, PauseModalType::QuitToDesktop);
            }
        }
    }

    // [ESC] ゲームに戻る
    if keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed in Pause Menu: Resuming game...");
        next_state.set(AppState::InGame);
    }
}

pub type PauseButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static mut BackgroundColor, &'static mut BorderColor, &'static PauseButtonAction),
    (Changed<Interaction>, With<Button>),
>;

pub fn pause_button_interaction_system(mut query: PauseButtonInteractionQuery) {
    for (interaction, mut bg_color, mut border_color, action) in &mut query {
        let is_danger = matches!(
            action,
            PauseButtonAction::ConfirmModal
                | PauseButtonAction::RequestReturnToTitle
                | PauseButtonAction::RequestQuitToDesktop
        );
        match *interaction {
            Interaction::Pressed => {
                if is_danger {
                    *bg_color = BackgroundColor(BUTTON_DANGER_PRESSED);
                    *border_color = BorderColor::all(Color::WHITE);
                } else {
                    *bg_color = BackgroundColor(BUTTON_PRESSED);
                    *border_color = BorderColor::all(ACCENT_COLOR);
                }
            }
            Interaction::Hovered => {
                if is_danger {
                    *bg_color = BackgroundColor(BUTTON_DANGER_HOVERED);
                    *border_color = BorderColor::all(BORDER_DANGER);
                } else {
                    *bg_color = BackgroundColor(BUTTON_HOVERED);
                    *border_color = BorderColor::all(ACCENT_COLOR);
                }
            }
            Interaction::None => {
                if matches!(action, PauseButtonAction::ConfirmModal) {
                    *bg_color = BackgroundColor(BUTTON_DANGER_NORMAL);
                    *border_color = BorderColor::all(BORDER_DANGER);
                } else if is_danger {
                    *bg_color = BackgroundColor(BUTTON_NORMAL);
                    *border_color = BorderColor::all(Color::srgb(0.55, 0.25, 0.25));
                } else {
                    *bg_color = BackgroundColor(BUTTON_NORMAL);
                    *border_color = BorderColor::all(BORDER_COLOR);
                }
            }
        }
    }
}

pub type PauseButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static PauseButtonAction),
    (Changed<Interaction>, With<Button>),
>;

#[derive(SystemParam)]
pub struct PauseActionContext<'w, 's> {
    commands: Commands<'w, 's>,
    focus: ResMut<'w, PauseMenuFocus>,
    settings: ResMut<'w, GameSettings>,
    next_state: ResMut<'w, NextState<AppState>>,
    exit_events: MessageWriter<'w, AppExit>,
    asset_server: Res<'w, AssetServer>,
    modal_query: Query<'w, 's, Entity, With<PauseConfirmModal>>,
}

pub fn pause_button_action_system(
    interaction_query: PauseButtonActionQuery,
    mut ctx: PauseActionContext,
) {
    let PauseActionContext {
        ref mut commands,
        ref mut focus,
        ref mut settings,
        ref mut next_state,
        ref mut exit_events,
        ref asset_server,
        ref modal_query,
    } = ctx;
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                PauseButtonAction::Resume => {
                    info!("Resuming game from Pause Menu...");
                    next_state.set(AppState::InGame);
                }
                PauseButtonAction::Settings => {
                    info!("Opening Settings from Pause Menu...");
                    settings.return_state = AppState::PauseMenu;
                    next_state.set(AppState::Settings);
                }
                PauseButtonAction::RequestReturnToTitle => {
                    info!("Requesting return to title modal...");
                    if modal_query.is_empty() {
                        focus.modal_open = true;
                        focus.modal_type = PauseModalType::ReturnToTitle;
                        focus.modal_focus = PauseModalFocusItem::Cancel;
                        spawn_pause_confirm_modal(commands, asset_server, PauseModalType::ReturnToTitle);
                    }
                }
                PauseButtonAction::RequestQuitToDesktop => {
                    info!("Requesting quit to desktop modal...");
                    if modal_query.is_empty() {
                        focus.modal_open = true;
                        focus.modal_type = PauseModalType::QuitToDesktop;
                        focus.modal_focus = PauseModalFocusItem::Cancel;
                        spawn_pause_confirm_modal(commands, asset_server, PauseModalType::QuitToDesktop);
                    }
                }
                PauseButtonAction::ConfirmModal => {
                    for entity in modal_query {
                        commands.entity(entity).despawn();
                    }
                    focus.modal_open = false;
                    match focus.modal_type {
                        PauseModalType::ReturnToTitle => {
                            info!("Agreed to return to title.");
                            settings.return_state = AppState::Title;
                            next_state.set(AppState::Title);
                        }
                        PauseModalType::QuitToDesktop => {
                            info!("Agreed to quit to desktop.");
                            exit_events.write(AppExit::Success);
                        }
                    }
                }
                PauseButtonAction::CancelModal => {
                    info!("Cancelled pause modal.");
                    for entity in modal_query {
                        commands.entity(entity).despawn();
                    }
                    focus.modal_open = false;
                }
            }
        }
    }
}

pub fn update_pause_focus_highlight_system(
    focus: Res<PauseMenuFocus>,
    mut menu_query: Query<
        (&PauseMenuButton, &mut BorderColor, &mut BackgroundColor),
        Without<PauseModalButton>,
    >,
    mut modal_query: Query<
        (&PauseModalButton, &mut BorderColor, &mut BackgroundColor),
        Without<PauseMenuButton>,
    >,
) {
    if focus.modal_open {
        for (modal_btn, mut border_color, mut bg_color) in &mut modal_query {
            let is_focused = focus.modal_focus == modal_btn.0;
            match modal_btn.0 {
                PauseModalFocusItem::Confirm => {
                    if is_focused {
                        *border_color = BorderColor::all(Color::WHITE);
                        *bg_color = BackgroundColor(BUTTON_DANGER_HOVERED);
                    } else {
                        *border_color = BorderColor::all(BORDER_DANGER);
                        *bg_color = BackgroundColor(BUTTON_DANGER_NORMAL);
                    }
                }
                PauseModalFocusItem::Cancel => {
                    if is_focused {
                        *border_color = BorderColor::all(ACCENT_COLOR);
                        *bg_color = BackgroundColor(BUTTON_HOVERED);
                    } else {
                        *border_color = BorderColor::all(BORDER_COLOR);
                        *bg_color = BackgroundColor(BUTTON_NORMAL);
                    }
                }
            }
        }
    } else {
        for (menu_btn, mut border_color, mut bg_color) in &mut menu_query {
            let is_focused = focus.current_item == menu_btn.0;
            let is_danger = matches!(
                menu_btn.0,
                PauseMenuItem::ReturnToTitle | PauseMenuItem::QuitToDesktop
            );

            if is_danger {
                if is_focused {
                    *border_color = BorderColor::all(BORDER_DANGER);
                    *bg_color = BackgroundColor(BUTTON_DANGER_HOVERED);
                } else {
                    *border_color = BorderColor::all(Color::srgb(0.55, 0.25, 0.25));
                    *bg_color = BackgroundColor(BUTTON_NORMAL);
                }
            } else if is_focused {
                *border_color = BorderColor::all(ACCENT_COLOR);
                *bg_color = BackgroundColor(BUTTON_HOVERED);
            } else {
                *border_color = BorderColor::all(BORDER_COLOR);
                *bg_color = BackgroundColor(BUTTON_NORMAL);
            }
        }
    }
}
