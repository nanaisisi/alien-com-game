use bevy::prelude::*;
use std::collections::HashMap;

use super::hex::HexCoord;
use super::terrain::TerrainType;

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
