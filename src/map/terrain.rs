use bevy::prelude::*;

/// タイルの地形種別
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub enum TerrainType {
    /// 平原: 標準的な地形、視界良好、移動コスト低
    Plains,
    /// 丘陵: 視界ボーナス、防御有利
    Hills,
    /// 森林/原生林: 遮蔽あり、機甲部隊は進行しづらい
    Forest,
    /// 山岳: 通行不能または極めて高コスト、航空機のみ通過可
    Mountains,
    /// 海洋/水域: 陸上部隊進入不可（艦船・航空機のみ）
    Ocean,
    /// 毒性湿地/瘴気帯: エイリアン活動度高、人類部隊に継続ダメージやデバフ
    ToxicSwamp,
    /// 結晶・鉱物地帯: 資源採掘適地、特異な生態系
    CrystalFields,
}

impl TerrainType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Plains => "平原 (Plains)",
            Self::Hills => "丘陵 (Hills)",
            Self::Forest => "異星原生林 (Alien Forest)",
            Self::Mountains => "険峻な山岳 (Mountains)",
            Self::Ocean => "海洋 (Ocean)",
            Self::ToxicSwamp => "瘴気湿地 (Toxic Swamp)",
            Self::CrystalFields => "共鳴結晶地帯 (Crystal Fields)",
        }
    }

    /// 地形の基礎色 (SF感のあるスタイリッシュなパレット)
    pub fn base_color(&self) -> Color {
        match self {
            Self::Plains => Color::srgb(0.24, 0.42, 0.28),        // 深い緑
            Self::Hills => Color::srgb(0.48, 0.45, 0.35),         // 黄土・岩色
            Self::Forest => Color::srgb(0.12, 0.30, 0.22),        // 濃緑
            Self::Mountains => Color::srgb(0.38, 0.40, 0.46),     // 暗めの岩石色
            Self::Ocean => Color::srgb(0.10, 0.25, 0.48),         // ディープブルー
            Self::ToxicSwamp => Color::srgb(0.35, 0.18, 0.45),    // 瘴気パープル
            Self::CrystalFields => Color::srgb(0.20, 0.60, 0.65), // エメラルドシアン
        }
    }

    /// ホバー時の強調色
    pub fn hovered_color(&self) -> Color {
        match self {
            Self::Plains => Color::srgb(0.38, 0.60, 0.42),
            Self::Hills => Color::srgb(0.65, 0.62, 0.50),
            Self::Forest => Color::srgb(0.24, 0.48, 0.35),
            Self::Mountains => Color::srgb(0.55, 0.58, 0.65),
            Self::Ocean => Color::srgb(0.22, 0.42, 0.70),
            Self::ToxicSwamp => Color::srgb(0.52, 0.30, 0.65),
            Self::CrystalFields => Color::srgb(0.35, 0.80, 0.85),
        }
    }

    /// 3D描画時の高さオフセット（起伏表現）
    pub fn height(&self) -> f32 {
        match self {
            Self::Ocean => 0.1,
            Self::ToxicSwamp => 0.15,
            Self::Plains => 0.25,
            Self::Forest => 0.35,
            Self::CrystalFields => 0.38,
            Self::Hills => 0.55,
            Self::Mountains => 0.95,
        }
    }

    /// 移動コスト（1.0 = 標準）
    pub fn movement_cost(&self) -> f32 {
        match self {
            Self::Plains => 1.0,
            Self::Hills => 1.5,
            Self::Forest => 1.8,
            Self::ToxicSwamp => 2.0,
            Self::CrystalFields => 1.4,
            Self::Mountains | Self::Ocean => 999.0, // 通行不可
        }
    }

    /// 進入可能かどうか（一般地上部隊）
    pub fn is_passable_ground(&self) -> bool {
        !matches!(self, Self::Mountains | Self::Ocean)
    }
}
