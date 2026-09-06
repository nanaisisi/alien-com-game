use bevy::prelude::*;

use crate::state::AppState;

pub mod territory;
pub mod types;

#[allow(unused_imports)]
pub use territory::{
    cleanup_territories, setup_initial_faction_territories, update_territory_overlays,
    FactionOutpost, TerritoryMap, TerritoryOverlay, TileTerritory,
};
pub use types::{DiplomaticRelation, FactionId, FactionManager, PlayerFaction};

pub struct FactionPlugin;

impl Plugin for FactionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerFaction>()
            .init_resource::<FactionManager>()
            .init_resource::<TerritoryMap>()
            .register_type::<FactionId>()
            .register_type::<DiplomaticRelation>()
            .register_type::<TileTerritory>()
            .add_systems(
                OnEnter(AppState::InGame),
                setup_initial_faction_territories.after(crate::map::generate_hex_map),
            )
            .add_systems(
                Update,
                update_territory_overlays.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_territories);
    }
}
