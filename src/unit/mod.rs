use bevy::prelude::*;

use crate::state::AppState;

pub mod interaction;
pub mod mesh;
pub mod spawn;
pub mod types;

#[allow(unused_imports)]
pub use interaction::{ReachableTiles, UnitInteractionPlugin};
#[allow(unused_imports)]
pub use spawn::{cleanup_units, spawn_initial_units};
#[allow(unused_imports)]
pub use types::{CombatGroupType, MoveTargetMarker, SelectedUnit, Unit, UnitSelectionRing};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Unit>()
            .register_type::<CombatGroupType>()
            .add_plugins(UnitInteractionPlugin)
            .add_systems(
                OnEnter(AppState::InGame),
                spawn_initial_units.after(crate::faction::setup_initial_faction_territories),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_units);
    }
}
