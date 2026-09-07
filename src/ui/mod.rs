use bevy::prelude::*;

pub mod city;
pub mod debug_console;
pub mod diplomacy;
pub mod faction_select;
pub mod hud;
pub mod info_panel;
pub mod minimap;
pub mod pause;
pub mod settings;
pub mod theme;
pub mod title;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            title::TitleUiPlugin,
            faction_select::FactionSelectPlugin,
            settings::SettingsUiPlugin,
            hud::InGameHudPlugin,
            info_panel::InfoPanelPlugin,
            city::CityUiPlugin,
            diplomacy::DiplomacyUiPlugin,
            pause::PauseMenuPlugin,
            minimap::MinimapPlugin,
            debug_console::DebugConsolePlugin,
        ));
    }
}

/// マーカーコンポーネント: このUI要素の上にマウスカーソルがある場合、
/// タイルのホバー・選択やドラッグ操作などマップインタラクションをブロックする。
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UiBlockMapInteraction;



