use bevy::prelude::*;

/// 惑星の環境タイプ
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum PlanetEnvironment {
    /// テラ型バランス: 地球類似。平原、森林、海洋、山岳が適度に調和
    #[default]
    Terra,
    /// 乾燥砂漠惑星: 海洋が少なく、広大な平原と丘陵、乾燥地帯
    Arid,
    /// 海洋群島惑星: 70%以上が海で覆われ、島嶼部が点在
    Archipelago,
    /// 瘴気・原始沼沢地: 毒性湿地と濃密な原生林が広がる高危険度環境
    ToxicMarsh,
    /// 晶氷・結晶極地: 結晶鉱床が豊富に露出し、険しい山岳が連なる
    Crystalline,
}

impl PlanetEnvironment {
    pub const ALL: [PlanetEnvironment; 5] = [
        PlanetEnvironment::Terra,
        PlanetEnvironment::Arid,
        PlanetEnvironment::Archipelago,
        PlanetEnvironment::ToxicMarsh,
        PlanetEnvironment::Crystalline,
    ];

    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Terra => "テラ型標準 (Terra)",
            Self::Arid => "乾燥砂漠 (Arid Dunes)",
            Self::Archipelago => "海洋諸島 (Archipelago)",
            Self::ToxicMarsh => "瘴気沼沢 (Toxic Marsh)",
            Self::Crystalline => "結晶山脈 (Crystalline)",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Terra => "母星に近い気候。海洋と陸地、森林が程よく分布する探査安定惑星。",
            Self::Arid => "水資源が乏しく広大な乾燥大地が続く。陸上部隊の行軍は容易だが水域は極小。",
            Self::Archipelago => "地表の大半が海に没した水惑星。限られた群島をめぐる激しい領有争いが生じる。",
            Self::ToxicMarsh => "原生異星植物と毒性湿地帯が密集。未知の生態系が活発で過酷な進軍を強いられる。",
            Self::Crystalline => "共鳴結晶鉱床が露出した高標高帯。特殊資源に恵まれるが険峻な山脈が遮る。",
        }
    }

    /// 海洋判定となる標高閾値（これ未満が海洋）
    pub fn sea_level_threshold(&self) -> f32 {
        match self {
            Self::Terra => 0.42,
            Self::Arid => 0.22,        // 海が非常に少ない
            Self::Archipelago => 0.58, // 海が非常に多い
            Self::ToxicMarsh => 0.38,  // 湿地が増えるため海自体は標準やや低め
            Self::Crystalline => 0.35, // 陸地多め
        }
    }

    /// 山岳発生の標高閾値
    pub fn mountain_threshold(&self) -> f32 {
        match self {
            Self::Terra => 0.72,
            Self::Arid => 0.75,
            Self::Archipelago => 0.78,
            Self::ToxicMarsh => 0.80, // 山岳は少なく湿地が多い
            Self::Crystalline => 0.65, // 山岳が頻出
        }
    }

    /// 丘陵発生の標高閾値
    pub fn hill_threshold(&self) -> f32 {
        match self {
            Self::Terra => 0.60,
            Self::Arid => 0.52,
            Self::Archipelago => 0.68,
            Self::ToxicMarsh => 0.65,
            Self::Crystalline => 0.50,
        }
    }

    /// 特性テーマカラー（UI用）
    pub fn theme_color(&self) -> Color {
        match self {
            Self::Terra => Color::srgb(0.25, 0.75, 0.50),
            Self::Arid => Color::srgb(0.85, 0.65, 0.30),
            Self::Archipelago => Color::srgb(0.20, 0.60, 0.90),
            Self::ToxicMarsh => Color::srgb(0.70, 0.35, 0.80),
            Self::Crystalline => Color::srgb(0.30, 0.85, 0.85),
        }
    }
}

/// マップサイズ定義
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum MapSize {
    /// 小型 (20x12)
    Small,
    /// 標準 (28x16)
    #[default]
    Standard,
    /// 大型 (36x20)
    Large,
}

impl MapSize {
    pub const ALL: [MapSize; 3] = [MapSize::Small, MapSize::Standard, MapSize::Large];

    pub fn dimensions(&self) -> (i32, i32) {
        match self {
            Self::Small => (20, 12),
            Self::Standard => (28, 16),
            Self::Large => (36, 20),
        }
    }

    pub fn width(&self) -> i32 {
        self.dimensions().0
    }

    pub fn height(&self) -> i32 {
        self.dimensions().1
    }

    pub fn name_ja(&self) -> &'static str {
        match self {
            Self::Small => "小型 (Small: 20x12)",
            Self::Standard => "標準 (Standard: 28x16)",
            Self::Large => "大型 (Large: 36x20)",
        }
    }
}

/// ゲーム中のマップ生成パラメータリソース
#[derive(Resource, Debug, Clone, Reflect)]
pub struct MapConfig {
    pub environment: PlanetEnvironment,
    pub size: MapSize,
    pub seed: u32,
}

impl Default for MapConfig {
    fn default() -> Self {
        Self {
            environment: PlanetEnvironment::Terra,
            size: MapSize::Standard,
            seed: 1337,
        }
    }
}

impl MapConfig {
    pub fn width(&self) -> i32 {
        self.size.width()
    }

    pub fn height(&self) -> i32 {
        self.size.height()
    }
}
