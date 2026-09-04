use bevy::prelude::*;

pub mod diplomacy;
pub mod faction_select;
pub mod hud;
pub mod minimap;
pub mod pause;
pub mod settings;
pub mod title;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            title::TitleUiPlugin,
            faction_select::FactionSelectPlugin,
            settings::SettingsUiPlugin,
            hud::InGameHudPlugin,
            diplomacy::DiplomacyUiPlugin,
            pause::PauseMenuPlugin,
            minimap::MinimapPlugin,
        ));
    }
}



