use bevy::app::AppExit;
use bevy::prelude::*;

use super::types::*;
use crate::state::AppState;
use crate::ui::theme::UiTheme;

pub fn title_keyboard_navigation_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut focus: ResMut<TitleMenuFocus>,
    mut next_state: ResMut<NextState<AppState>>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut exit_events: bevy::ecs::message::MessageWriter<AppExit>,
) {
    let count = MENU_ACTIONS.len();
    let current = focus.selected_index.unwrap_or(0);

    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        focus.selected_index = Some(if current == 0 { count - 1 } else { current - 1 });
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        focus.selected_index = Some((current + 1) % count);
    }

    if (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space))
        && let Some(selected) = focus.selected_index
            && selected < count {
                execute_title_action(
                    MENU_ACTIONS[selected],
                    &mut settings,
                    &mut next_state,
                    &mut exit_events,
                );
            }
}

pub type TitleButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        &'static TitleMenuButton,
    ),
    With<Button>,
>;

pub type TitleButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static MenuButtonAction),
    (Changed<Interaction>, With<Button>),
>;

pub fn button_interaction_system(
    mut interaction_query: TitleButtonInteractionQuery,
    mut focus: ResMut<TitleMenuFocus>,
) {
    // マウスホバーされた場合はフォーカスインデックスを更新
    for (interaction, _, _, menu_btn) in &interaction_query {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            focus.selected_index = Some(menu_btn.0);
        }
    }

    let current_selected = focus.selected_index;

    // スタイルの同期
    for (interaction, mut bg_color, mut border_color, menu_btn) in &mut interaction_query {
        let is_selected = current_selected == Some(menu_btn.0);

        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(UiTheme::BUTTONS.standard.pressed);
                *border_color = BorderColor::all(UiTheme::SURFACES.accent);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(UiTheme::BUTTONS.standard.hovered);
                *border_color = BorderColor::all(UiTheme::SURFACES.accent);
            }
            Interaction::None => {
                if is_selected {
                    *bg_color = BackgroundColor(UiTheme::BUTTONS.standard.hovered);
                    *border_color = BorderColor::all(UiTheme::SURFACES.accent);
                } else {
                    *bg_color = BackgroundColor(UiTheme::BUTTONS.standard.normal);
                    *border_color = BorderColor::all(Color::srgb(0.25, 0.38, 0.50));
                }
            }
        }
    }
}

pub fn execute_title_action(
    action: MenuButtonAction,
    settings: &mut ResMut<crate::ui::settings::GameSettings>,
    next_state: &mut ResMut<NextState<AppState>>,
    exit_events: &mut bevy::ecs::message::MessageWriter<AppExit>,
) {
    match action {
        MenuButtonAction::NewGame => {
            info!("Transitioning to Faction Selection...");
            next_state.set(AppState::FactionSelect);
        }
        MenuButtonAction::LoadGame => {
            info!("Load Game clicked (WIP)");
        }
        MenuButtonAction::Settings => {
            info!("Transitioning to Settings...");
            settings.return_state = AppState::Title;
            next_state.set(AppState::Settings);
        }
        MenuButtonAction::Exit => {
            info!("Exiting Game...");
            exit_events.write(AppExit::Success);
        }
    }
}

pub fn button_action_system(
    interaction_query: TitleButtonActionQuery,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_events: bevy::ecs::message::MessageWriter<AppExit>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            execute_title_action(*action, &mut settings, &mut next_state, &mut exit_events);
        }
    }
}
