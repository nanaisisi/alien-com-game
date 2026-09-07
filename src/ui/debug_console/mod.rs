use bevy::prelude::*;

use crate::state::AppState;

pub mod command;
pub mod interaction;
pub mod types;
pub mod view;

pub use types::DebugConsoleState;

pub struct DebugConsolePlugin;

impl Plugin for DebugConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugConsoleState>()
            .add_systems(
                Update,
                (
                    interaction::handle_secret_trigger_click,
                    interaction::handle_toggle_debug_console,
                    interaction::handle_warning_modal_interaction,
                    interaction::handle_console_close_button,
                    interaction::handle_console_keyboard_input,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame).or_else(in_state(AppState::Title))),
            );
    }
}
