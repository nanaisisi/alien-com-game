use bevy::prelude::*;

use crate::faction::types::FactionId;
use crate::map::hex::HexCoord;

/// 戦闘団（Combat Group）のカテゴリー種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum CombatGroupType {
    /// 偵察戦闘団: 高機動・視界確保特化
    Scout,
    /// 開拓/民間人戦闘団: 新たな拠点の設営能力
    Colonist,
    /// 軽歩兵戦闘団: 基本的な陸上戦闘部隊
    LightInfantry,
}

impl CombatGroupType {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Scout => "偵察戦闘団 (Scout)",
            Self::Colonist => "開拓戦闘団 (Colonizer)",
            Self::LightInfantry => "軽歩兵戦闘団 (Light Infantry)",
        }
    }

    pub fn base_movement(&self) -> u32 {
        match self {
            Self::Scout => 3,
            Self::Colonist => 2,
            Self::LightInfantry => 2,
        }
    }

    pub fn base_max_hp(&self) -> u32 {
        match self {
            Self::Scout => 80,
            Self::Colonist => 50,
            Self::LightInfantry => 100,
        }
    }

    pub fn attack_power(&self) -> u32 {
        match self {
            Self::Scout => 15,
            Self::Colonist => 0,
            Self::LightInfantry => 30,
        }
    }
}

/// 全体マップ上で活動する戦闘団ユニット
#[derive(Component, Debug, Clone, Reflect)]
pub struct Unit {
    pub faction: FactionId,
    pub group_type: CombatGroupType,
    pub coord: HexCoord,
    pub max_movement: u32,
    pub current_movement: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub is_exhausted: bool,
}

impl Unit {
    pub fn new(faction: FactionId, group_type: CombatGroupType, coord: HexCoord) -> Self {
        let max_movement = group_type.base_movement();
        let max_hp = group_type.base_max_hp();
        Self {
            faction,
            group_type,
            coord,
            max_movement,
            current_movement: max_movement,
            hp: max_hp,
            max_hp,
            is_exhausted: false,
        }
    }

    #[allow(dead_code)]
    pub fn reset_turn(&mut self) {
        self.current_movement = self.max_movement;
        self.is_exhausted = false;
    }
}

/// 現在選択されているユニットのエンティティ
#[derive(Resource, Default, Debug)]
pub struct SelectedUnit(pub Option<Entity>);

/// ユニット移動可能タイルの表示用マーカー
#[derive(Component)]
pub struct MoveTargetMarker {
    #[allow(dead_code)]
    pub target_coord: HexCoord,
}

/// ユニット選択リング表示用マーカー
#[derive(Component)]
pub struct UnitSelectionRing;
