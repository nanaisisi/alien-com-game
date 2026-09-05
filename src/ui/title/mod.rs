use bevy::prelude::*;

use crate::state::AppState;

pub mod interaction;
pub mod types;
pub mod view;

#[allow(unused_imports)]
pub use interaction::{
    button_action_system, button_interaction_system, execute_title_action,
    title_keyboard_navigation_system,
};
#[allow(unused_imports)]
pub use types::{
    reset_title_focus, MenuButtonAction, TitleMenuButton, TitleMenuFocus, TitleRootUi, MENU_ACTIONS,
};
pub use view::{cleanup_title_ui, setup_title_ui};

pub struct TitleUiPlugin;

impl Plugin for TitleUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TitleMenuFocus>()
            .add_systems(OnEnter(AppState::Title), (setup_title_ui, reset_title_focus))
            .add_systems(
                Update,
                (
                    title_keyboard_navigation_system,
                    button_interaction_system,
                    button_action_system,
                )
                    .run_if(in_state(AppState::Title)),
            )
            .add_systems(OnExit(AppState::Title), cleanup_title_ui);
    }
}
