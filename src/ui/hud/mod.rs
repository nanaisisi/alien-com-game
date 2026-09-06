use bevy::prelude::*;

use crate::state::AppState;

pub mod interaction;
pub mod types;
pub mod view;

pub use interaction::{
    handle_keyboard_shortcuts, hud_button_action_system, hud_button_interaction_system,
    update_hud_display_system,
};
#[allow(unused_imports)]
pub use types::{HudAction, HudLabel, HudRoot};
pub use view::{cleanup_hud, setup_hud};

pub struct InGameHudPlugin;

impl Plugin for InGameHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_hud)
            .add_systems(
                Update,
                (
                    hud_button_interaction_system,
                    hud_button_action_system,
                    update_hud_display_system,
                    handle_keyboard_shortcuts,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_hud);
    }
}
