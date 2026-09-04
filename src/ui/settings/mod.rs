use bevy::prelude::*;

use crate::state::AppState;

pub mod graphics;
pub mod interaction;
pub mod types;
pub mod view;

pub use graphics::{
    apply_graphics_settings_system, enforce_fps_limit_system, setup_environment_settings_system,
    FpsLimiterState,
};
pub use interaction::{
    settings_button_action_system, settings_button_interaction_system,
    settings_keyboard_navigation_system, update_settings_display_system,
    update_settings_focus_highlight_system,
};
#[allow(unused_imports)]
pub use types::{
    reset_settings_focus, AntiAliasingMode, FpsLimitMode, GameSettings, ModalFocusItem,
    SettingsFocusItem, SettingsNavFocus, RESOLUTION_PRESETS,
};
pub use view::{cleanup_settings_ui, setup_settings_ui};

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>()
            .init_resource::<SettingsNavFocus>()
            .init_resource::<FpsLimiterState>()
            .add_systems(Startup, setup_environment_settings_system)
            .add_systems(PostStartup, apply_graphics_settings_system)
            .add_systems(Update, apply_graphics_settings_system)
            .add_systems(Last, enforce_fps_limit_system)
            .add_systems(OnEnter(AppState::Settings), (setup_settings_ui, reset_settings_focus))
            .add_systems(
                Update,
                (
                    settings_keyboard_navigation_system,
                    settings_button_interaction_system,
                    settings_button_action_system,
                    update_settings_display_system,
                    update_settings_focus_highlight_system,
                )
                    .run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), cleanup_settings_ui);
    }
}
