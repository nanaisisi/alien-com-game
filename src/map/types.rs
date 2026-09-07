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

/// マップ表示設定（Civ6スタイルのグリッド表示 G、産出アイコン表示 Y）
#[derive(Resource, Debug, Clone)]
pub struct MapDisplaySettings {
    pub show_grid: bool,
    pub show_yields: bool,
}

impl Default for MapDisplaySettings {
    fn default() -> Self {
        Self {
            show_grid: true,
            show_yields: false,
        }
    }
}

#[derive(Component)]
pub struct HexTile {
    pub coord: HexCoord,
    pub terrain: TerrainType,
}

#[derive(Component)]
pub struct MapRoot;

/// グリッド線オーバーレイマーカー
#[derive(Component)]
pub struct GridOverlayLine;

/// タイル産出（Yield）オーバーレイマーカー
#[derive(Component)]
pub struct YieldOverlayTag;
