use bevy::prelude::*;

use crate::faction::{FactionId, FactionManager, FactionResources, PlayerFaction};
use crate::state::AppState;
use crate::ui::theme::UiTheme;

use super::types::{
    HudAction, HudLabel, END_TURN_HOVER, END_TURN_NORMAL, END_TURN_PRESSED,
};

type ButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        &'static HudAction,
    ),
    (Changed<Interaction>, With<Button>),
>;

pub fn hud_button_interaction_system(mut query: ButtonInteractionQuery) {
    let surfaces = UiTheme::surfaces();
    let standard_btn = UiTheme::button(false);

    for (interaction, mut bg_color, mut border_color, action) in &mut query {
        match action {
            HudAction::EndTurn => match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(END_TURN_PRESSED);
                    *border_color = BorderColor::all(Color::WHITE);
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(END_TURN_HOVER);
                    *border_color = BorderColor::all(Color::srgb(0.5, 0.95, 0.9));
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(END_TURN_NORMAL);
                    *border_color = BorderColor::all(surfaces.accent());
                }
            },
            HudAction::OpenDiplomacy => match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(standard_btn.pressed());
                    *border_color = BorderColor::all(Color::WHITE);
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(Color::srgb(0.16, 0.35, 0.44));
                    *border_color = BorderColor::all(Color::WHITE);
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(Color::srgba(0.10, 0.25, 0.32, 0.85));
                    *border_color = BorderColor::all(surfaces.accent());
                }
            },
            HudAction::OpenMenu => match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(standard_btn.pressed());
                    *border_color = BorderColor::all(surfaces.accent());
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(standard_btn.hovered());
                    *border_color = BorderColor::all(surfaces.accent());
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(standard_btn.normal());
                    *border_color = BorderColor::all(surfaces.border());
                }
            },
        }
    }
}

type ButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static HudAction),
    (Changed<Interaction>, With<Button>),
>;

#[allow(clippy::too_many_arguments)]
pub fn hud_button_action_system(
    query: ButtonActionQuery,
    mut resources: ResMut<FactionResources>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_faction: Res<PlayerFaction>,
    faction_mgr: Res<FactionManager>,
    mut modal_state: ResMut<crate::ui::diplomacy::DiplomacyModalState>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                HudAction::EndTurn => {
                    advance_turn(&mut resources);
                }
                HudAction::OpenDiplomacy => {
                    modal_state.is_open = !modal_state.is_open;
                    if modal_state.is_open {
                        if modal_state.selected_target.is_none() {
                            modal_state.selected_target = FactionId::ALL
                                .iter()
                                .copied()
                                .find(|&f| f != player_faction.0);
                        }
                        crate::ui::diplomacy::spawn_diplomacy_modal(
                            &mut commands,
                            &asset_server,
                            player_faction.0,
                            &modal_state,
                            &faction_mgr,
                        );
                    }
                }
                HudAction::OpenMenu => {
                    info!("Opening Pause Menu...");
                    settings.return_state = AppState::InGame;
                    next_state.set(AppState::PauseMenu);
                }
            }
        }
    }
}

pub fn handle_keyboard_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    mut resources: ResMut<FactionResources>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        advance_turn(&mut resources);
    }
    if keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed: Opening Pause Menu...");
        settings.return_state = AppState::InGame;
        next_state.set(AppState::PauseMenu);
    }
}

fn advance_turn(resources: &mut FactionResources) {
    resources.turn += 1;
    resources.energy += resources.energy_per_turn;
    resources.production += resources.production_per_turn;
    resources.science += resources.science_per_turn;
    resources.food += resources.food_per_turn;
    info!("Advancing to Turn {}", resources.turn);
}

pub fn update_hud_display_system(
    resources: Res<FactionResources>,
    mut query: Query<(&mut Text, &HudLabel)>,
) {
    if !resources.is_changed() {
        return;
    }

    for (mut text, label) in &mut query {
        match label {
            HudLabel::Turn => {
                **text = format!("TURN {:02}", resources.turn);
            }
            HudLabel::Energy => {
                **text = format!("{} (+{})", resources.energy, resources.energy_per_turn);
            }
            HudLabel::Production => {
                **text = format!(
                    "{} (+{})",
                    resources.production, resources.production_per_turn
                );
            }
            HudLabel::Science => {
                **text = format!("{} (+{})", resources.science, resources.science_per_turn);
            }
            HudLabel::Food => {
                **text = format!("{} (+{})", resources.food, resources.food_per_turn);
            }
        }
    }
}
