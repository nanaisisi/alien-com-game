use bevy::prelude::*;

use super::types::*;
use crate::faction::types::PlayerFaction;
use crate::map::settings::MapConfig;
use crate::state::AppState;
use crate::ui::theme::UiTheme;

pub type FactionSelectInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static Interaction,
        &'static SelectAction,
        &'static mut BorderColor,
        &'static mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>),
>;

pub type FactionSelectActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static SelectAction),
    (Changed<Interaction>, With<Button>),
>;

pub type DetailTitleTextQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static mut Text, &'static mut TextColor),
    (With<DetailTitleText>, Without<DetailDescText>),
>;

pub type DetailDescTextQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut Text,
    (With<DetailDescText>, Without<DetailTitleText>),
>;

pub fn faction_select_button_system(
    mut interaction_query: FactionSelectInteractionQuery,
    selected_menu: Res<SelectedFactionMenu>,
    map_config: Res<MapConfig>,
) {
    let surfaces = UiTheme::surfaces();

    for (interaction, action, mut border_color, mut bg_color) in &mut interaction_query {
        match action {
            SelectAction::Choose(f) => {
                let is_active = *f == selected_menu.faction;
                match *interaction {
                    Interaction::Pressed | Interaction::Hovered => {
                        *border_color = BorderColor::all(f.accent_color());
                        *bg_color = BackgroundColor(Color::srgba(0.20, 0.30, 0.42, 0.9));
                    }
                    Interaction::None => {
                        if is_active {
                            *border_color = BorderColor::all(f.primary_color());
                            *bg_color = BackgroundColor(Color::srgba(0.15, 0.22, 0.32, 0.9));
                        } else {
                            *border_color = BorderColor::all(surfaces.border());
                            *bg_color = BackgroundColor(surfaces.card());
                        }
                    }
                }
            }
            SelectAction::ChooseEnvironment(env) => {
                let is_active = *env == map_config.environment;
                match *interaction {
                    Interaction::Pressed | Interaction::Hovered => {
                        *border_color = BorderColor::all(env.theme_color());
                        *bg_color = BackgroundColor(Color::srgba(0.22, 0.32, 0.45, 0.9));
                    }
                    Interaction::None => {
                        if is_active {
                            *border_color = BorderColor::all(env.theme_color());
                            *bg_color = BackgroundColor(Color::srgba(0.18, 0.26, 0.38, 0.9));
                        } else {
                            *border_color = BorderColor::all(surfaces.border());
                            *bg_color = BackgroundColor(surfaces.card());
                        }
                    }
                }
            }
            SelectAction::ChooseSize(sz) => {
                let is_active = *sz == map_config.size;
                match *interaction {
                    Interaction::Pressed | Interaction::Hovered => {
                        *border_color = BorderColor::all(surfaces.accent());
                        *bg_color = BackgroundColor(Color::srgba(0.25, 0.40, 0.50, 0.9));
                    }
                    Interaction::None => {
                        if is_active {
                            *border_color = BorderColor::all(surfaces.accent());
                            *bg_color = BackgroundColor(Color::srgba(0.20, 0.35, 0.45, 0.9));
                        } else {
                            *border_color = BorderColor::all(surfaces.border());
                            *bg_color = BackgroundColor(surfaces.card());
                        }
                    }
                }
            }
            SelectAction::RerollSeed => match *interaction {
                Interaction::Pressed | Interaction::Hovered => {
                    *border_color = BorderColor::all(surfaces.accent());
                    *bg_color = BackgroundColor(Color::srgba(0.18, 0.28, 0.38, 0.9));
                }
                Interaction::None => {
                    *border_color = BorderColor::all(surfaces.border());
                    *bg_color = BackgroundColor(surfaces.card());
                }
            },
            SelectAction::Confirm => match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(Color::srgb(0.25, 0.65, 0.70));
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(Color::srgb(0.18, 0.48, 0.55));
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(Color::srgb(0.12, 0.35, 0.40));
                }
            },
            SelectAction::Back => match *interaction {
                Interaction::Pressed | Interaction::Hovered => {
                    *border_color = BorderColor::all(surfaces.accent());
                    *bg_color = BackgroundColor(Color::srgb(0.15, 0.24, 0.35));
                }
                Interaction::None => {
                    *border_color = BorderColor::all(surfaces.border());
                    *bg_color = BackgroundColor(surfaces.card());
                }
            },
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn faction_select_action_system(
    query: FactionSelectActionQuery,
    mut selected_menu: ResMut<SelectedFactionMenu>,
    mut player_faction: ResMut<PlayerFaction>,
    mut map_config: ResMut<MapConfig>,
    mut next_state: ResMut<NextState<AppState>>,
    mut title_query: DetailTitleTextQuery,
    mut desc_query: DetailDescTextQuery,
    mut env_desc_query: Query<&mut Text, (With<EnvDescText>, Without<DetailTitleText>, Without<DetailDescText>, Without<SeedDisplayText>)>,
    mut seed_text_query: Query<&mut Text, (With<SeedDisplayText>, Without<DetailTitleText>, Without<DetailDescText>, Without<EnvDescText>)>,
    mut cards_query: Query<(&FactionCard, &mut BorderColor, &mut BackgroundColor), (Without<EnvButton>, Without<SizeButton>)>,
    mut env_btn_query: Query<(&EnvButton, &mut BorderColor, &mut BackgroundColor), (Without<FactionCard>, Without<SizeButton>)>,
    mut size_btn_query: Query<(&SizeButton, &mut BorderColor, &mut BackgroundColor), (Without<FactionCard>, Without<EnvButton>)>,
    mut panel_query: Query<&mut BorderColor, (With<DetailPanelRoot>, Without<FactionCard>, Without<EnvButton>, Without<SizeButton>)>,
) {
    for (interaction, action) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            SelectAction::Choose(faction) => {
                selected_menu.faction = *faction;

                if let Ok((mut text, mut text_color)) = title_query.single_mut() {
                    **text = format!("{} ({})", faction.formal_title(), faction.name_ja());
                    *text_color = TextColor(faction.accent_color());
                }
                if let Ok(mut text) = desc_query.single_mut() {
                    **text = faction.description().to_string();
                }

                if let Ok(mut panel_border) = panel_query.single_mut() {
                    *panel_border = BorderColor::all(faction.primary_color());
                }

                let surfaces = UiTheme::surfaces();

                // カード枠のスタイル更新
                for (card, mut border_color, mut bg_color) in &mut cards_query {
                    let is_active = card.0 == *faction;
                    if is_active {
                        *border_color = BorderColor::all(card.0.primary_color());
                        *bg_color = BackgroundColor(Color::srgba(0.15, 0.22, 0.32, 0.9));
                    } else {
                        *border_color = BorderColor::all(surfaces.border());
                        *bg_color = BackgroundColor(surfaces.card());
                    }
                }
            }
            SelectAction::ChooseEnvironment(env) => {
                map_config.environment = *env;

                if let Ok(mut text) = env_desc_query.single_mut() {
                    **text = env.description().to_string();
                }

                let surfaces = UiTheme::surfaces();
                for (btn, mut border_color, mut bg_color) in &mut env_btn_query {
                    let is_active = btn.0 == *env;
                    if is_active {
                        *border_color = BorderColor::all(btn.0.theme_color());
                        *bg_color = BackgroundColor(Color::srgba(0.18, 0.26, 0.38, 0.9));
                    } else {
                        *border_color = BorderColor::all(surfaces.border());
                        *bg_color = BackgroundColor(surfaces.card());
                    }
                }
            }
            SelectAction::ChooseSize(sz) => {
                map_config.size = *sz;

                let surfaces = UiTheme::surfaces();
                for (btn, mut border_color, mut bg_color) in &mut size_btn_query {
                    let is_active = btn.0 == *sz;
                    if is_active {
                        *border_color = BorderColor::all(surfaces.accent());
                        *bg_color = BackgroundColor(Color::srgba(0.20, 0.35, 0.45, 0.9));
                    } else {
                        *border_color = BorderColor::all(surfaces.border());
                        *bg_color = BackgroundColor(surfaces.card());
                    }
                }
            }
            SelectAction::RerollSeed => {
                let display_seed = map_config.reroll_seed();

                if let Ok(mut text) = seed_text_query.single_mut() {
                    **text = format!("🎲 SEED: {}", display_seed);
                }
            }
            SelectAction::Confirm => {
                player_faction.0 = selected_menu.faction;
                next_state.set(AppState::InGame);
            }
            SelectAction::Back => {
                next_state.set(AppState::Title);
            }
        }
    }
}
