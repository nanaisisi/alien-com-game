use bevy::prelude::*;

use crate::state::AppState;

pub mod generation;
pub mod hex;
pub mod interaction;
pub mod settings;
pub mod terrain;
pub mod types;

pub use self::generation::{cleanup_hex_map, generate_hex_map};
#[allow(unused_imports)]
pub use self::generation::create_hex_mesh;
#[allow(unused_imports)]
pub use self::hex::{DEFAULT_MAP_HEIGHT, DEFAULT_MAP_WIDTH, MAP_HEIGHT, MAP_WIDTH};
pub use self::hex::{MAP_HEIGHT as GRID_HEIGHT, MAP_WIDTH as GRID_WIDTH};
#[allow(unused_imports)]
pub use self::settings::{MapConfig, MapSize, PlanetEnvironment};
#[allow(unused_imports)]
pub use self::terrain::TerrainType;
pub use self::types::{HexTile, MapGrid, MapRoot, HEX_RADIUS};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapConfig>()
            .init_resource::<MapGrid>()
            .add_plugins(interaction::MapInteractionPlugin)
            .add_systems(OnEnter(AppState::InGame), generate_hex_map)
            .add_systems(OnEnter(AppState::Title), cleanup_hex_map);
    }
}
