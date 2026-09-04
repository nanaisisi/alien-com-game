use bevy::prelude::*;
use std::collections::HashMap;

use crate::state::AppState;
use self::hex::HexCoord;
use self::terrain::TerrainType;

pub mod generation;
pub mod hex;
pub mod interaction;
pub mod settings;
pub mod terrain;

pub use self::generation::{cleanup_hex_map, generate_hex_map};
#[allow(unused_imports)]
pub use self::generation::create_hex_mesh;
#[allow(unused_imports)]
pub use self::hex::{MAP_HEIGHT, MAP_WIDTH};
pub use self::hex::{MAP_HEIGHT as GRID_HEIGHT, MAP_WIDTH as GRID_WIDTH};
#[allow(unused_imports)]
pub use self::settings::{MapConfig, MapSize, PlanetEnvironment};

pub const HEX_RADIUS: f32 = 1.0;

/// マップ全体の状態を保持するリソース
#[derive(Resource, Default)]
pub struct MapGrid {
    pub tiles: HashMap<HexCoord, Entity>,
    pub terrain_data: HashMap<HexCoord, TerrainType>,
    pub width: i32,
    pub height: i32,
}

#[derive(Component)]
pub struct HexTile {
    pub coord: HexCoord,
    pub terrain: TerrainType,
}

#[derive(Component)]
pub struct MapRoot;

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
