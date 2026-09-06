pub mod input;
pub mod render_core;
pub mod render_integration;
pub mod view;

use bevy::prelude::*;

use crate::state::AppState;

#[allow(unused_imports)]
pub use view::{
    MinimapCameraBoxPart, MinimapCoordText, MinimapImageNode, MinimapRoot, MINIMAP_HEIGHT,
    MINIMAP_WIDTH,
};

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MinimapState>()
            .add_systems(OnEnter(AppState::InGame), view::setup_minimap_ui)
            .add_systems(
                Update,
                (
                    render_core::update_minimap_texture_system,
                    input::handle_minimap_interaction_system,
                    render_integration::update_minimap_viewport_system,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::Title), view::cleanup_minimap_ui);
    }
}

/// ミニマップの表示・制御状態
#[derive(Resource, Default)]
pub struct MinimapState {
    pub texture_handle: Option<Handle<Image>>,
    pub is_dragging: bool,
    pub needs_update: bool,
}
