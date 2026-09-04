use bevy::prelude::*;

use self::territory::{
    cleanup_territories, setup_initial_faction_territories, TerritoryMap, TileTerritory,
};
use self::types::{DiplomaticRelation, FactionId, FactionManager, PlayerFaction};
use crate::state::AppState;

pub mod territory;
pub mod types;

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
            .add_systems(OnEnter(AppState::Title), cleanup_territories);
    }
}
