use bevy::prelude::*;

pub mod settings;
pub mod title;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((title::TitleUiPlugin, settings::SettingsUiPlugin));
    }
}

