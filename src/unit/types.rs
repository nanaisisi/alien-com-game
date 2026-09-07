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

    pub fn attack_range(&self) -> u32 {
        match self {
            Self::Scout => 1,
            Self::Colonist => 0,
            Self::LightInfantry => 1,
        }
    }
}

/// ユニットの現在の行動・待機状態
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum UnitActionState {
    /// 通常（待機または行動中）
    #[default]
    Idle,
    /// 防御態勢（継続ターン数に応じて防御力+25%〜+50%）
    Fortified { turns: u32 },
    /// 警戒監視中（敵が接近・視界内に入ると自動で目覚める）
    Alert,
    /// 休眠中（手動選択または敵接近まで自動巡回スキップ）
    Sleeping,
    /// 回復・修理中（毎ターンHP回復、全快でIdleへ自動復帰）
    Healing,
}

impl UnitActionState {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Idle => "待機/行動可能",
            Self::Fortified { turns } => {
                if *turns >= 2 {
                    "防御陣地展開 (強固 +50%)"
                } else {
                    "防御態勢 (+25%)"
                }
            }
            Self::Alert => "警戒監視 (Alert)",
            Self::Sleeping => "休眠待機 (Sleep)",
            Self::Healing => "回復・修理中 (Healing)",
        }
    }

    pub fn defense_multiplier(&self) -> f32 {
        match self {
            Self::Fortified { turns } => {
                if *turns >= 2 {
                    1.50
                } else {
                    1.25
                }
            }
            _ => 1.0,
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
    pub action_state: UnitActionState,
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
            action_state: UnitActionState::Idle,
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

/// ユニット操作モード
#[allow(dead_code)]
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitCommandMode {
    #[default]
    None,
    /// Mキー: 移動先指定モード
    Move,
    /// Aキー: 突撃・近接攻撃指定モード
    Charge,
    /// Rキー: 遠隔射撃指定モード
    RangedAttack,
}

/// Mキーによるユニット移動指示モードの状態（後方互換）
#[derive(Resource, Default, Debug)]
pub struct MoveModeState(pub bool);

/// ユニット操作コマンドモード
#[allow(dead_code)]
#[derive(Resource, Default, Debug)]
pub struct ActiveCommandMode(pub UnitCommandMode);

/// ユニット移動可能タイルの表示用マーカー
#[derive(Component)]
pub struct MoveTargetMarker {
    #[allow(dead_code)]
    pub target_coord: HexCoord,
}

/// 攻撃可能ターゲットタイルの表示用マーカー
#[allow(dead_code)]
#[derive(Component)]
pub struct AttackTargetMarker {
    pub target_entity: Entity,
    pub target_coord: HexCoord,
}

/// ユニット選択リング表示用マーカー
#[derive(Component)]
pub struct UnitSelectionRing;
