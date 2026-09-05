use bevy::prelude::*;

use crate::state::AppState;

pub mod interaction;
pub mod types;
pub mod view;

pub use interaction::{faction_select_action_system, faction_select_button_system};
#[allow(unused_imports)]
pub use types::{
    DetailDescText, DetailPanelRoot, DetailTitleText, EnvButton, EnvDescText, FactionCard,
    FactionSelectRoot, SeedDisplayText, SelectAction, SelectedFactionMenu, SizeButton,
};
pub use view::{cleanup_faction_select_ui, setup_faction_select_ui};

pub struct FactionSelectPlugin;

impl Plugin for FactionSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedFactionMenu>()
            .add_systems(OnEnter(AppState::FactionSelect), setup_faction_select_ui)
            .add_systems(
                Update,
                (
                    faction_select_button_system,
                    faction_select_action_system,
                )
                    .run_if(in_state(AppState::FactionSelect)),
            )
            .add_systems(OnExit(AppState::FactionSelect), cleanup_faction_select_ui);
    }
}
