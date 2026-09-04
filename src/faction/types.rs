use bevy::prelude::*;
use std::collections::HashMap;

/// 惑星入植に関わる主要派閥ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub enum FactionId {
    /// 国A: 帝国 (赤) - 旧宗主国、貴族制・資本主義、軍事・拡張志向
    #[default]
    Empire,
    /// 国B: 大公国 (水色) - 北国、立憲君主制・実質共和制、文化・技術
    GrandDuchy,
    /// 国C: 連邦 (茶色) - 南部連邦、アメリカ/インド/アフリカ的、生産・多様性
    Federation,
    /// 国D: 共和国 (黄色) - 西部国家、中国風、集権的動員・工業
    Republic,
    /// 国E: 共同体 (緑) - 企業体と労働組合連合、東ロシア/ASEAN/オセアニア的
    Commonwealth,
    /// 国F: 連合 (青) - バシレス大陸覇権国家、世界最高水準の総合力
    Union,
}

impl FactionId {
    pub const ALL: [FactionId; 6] = [
        FactionId::Empire,
        FactionId::GrandDuchy,
        FactionId::Federation,
        FactionId::Republic,
        FactionId::Commonwealth,
        FactionId::Union,
    ];

    pub fn code(&self) -> &'static str {
        match self {
            FactionId::Empire => "A",
            FactionId::GrandDuchy => "B",
            FactionId::Federation => "C",
            FactionId::Republic => "D",
            FactionId::Commonwealth => "E",
            FactionId::Union => "F",
        }
    }

    pub fn name_ja(&self) -> &'static str {
        match self {
            FactionId::Empire => "帝国",
            FactionId::GrandDuchy => "大公国",
            FactionId::Federation => "連邦",
            FactionId::Republic => "共和国",
            FactionId::Commonwealth => "共同体",
            FactionId::Union => "連合",
        }
    }

    pub fn name_en(&self) -> &'static str {
        match self {
            FactionId::Empire => "Empire",
            FactionId::GrandDuchy => "Grand Duchy",
            FactionId::Federation => "Federation",
            FactionId::Republic => "Republic",
            FactionId::Commonwealth => "Commonwealth",
            FactionId::Union => "Union",
        }
    }

    pub fn formal_title(&self) -> &'static str {
        match self {
            FactionId::Empire => "アシリア帝国入植遠征軍",
            FactionId::GrandDuchy => "北方大公国開拓使団",
            FactionId::Federation => "アシリア南部自由連邦",
            FactionId::Republic => "アシリア人民統一共和国",
            FactionId::Commonwealth => "環洋共同体企業連合",
            FactionId::Union => "バシレス統合覇権連合",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            FactionId::Empire => {
                "アシリア大陸東部を領有する超大国。皇帝不在の中、皇太子が主導。貴族制と資本家が共存し、強大な軍事力と拡張志向を持つ。"
            }
            FactionId::GrandDuchy => {
                "アシリア大陸北東部から中央北部に広がる立憲君主制の北国。建前上は帝国の従属国だが実質的な対抗馬。高い科学技術と外交力を誇る。"
            }
            FactionId::Federation => {
                "アシリア大陸南部を領有する多民族連邦国家。豊かな資源と高い生産基盤を持ち、実利的な開拓・通商を重んじる。"
            }
            FactionId::Republic => {
                "アシリア大陸西部を領有する巨大集権国家。国家主導の計画経済と大規模な動員力を生かした急速なインフラ構築が得意。"
            }
            FactionId::Commonwealth => {
                "大陸北西部・南西部の島嶼部を拠点とする企業体と労働組合による連合体。通商ネットワークと生態系バイオテクノロジーに強み。"
            }
            FactionId::Union => {
                "バシレス大陸を統一する世界最高峰の覇権国家。高度な制度的統合と最先端のサイバネティクス・航空技術を誇る。"
            }
        }
    }

    pub fn primary_color(&self) -> Color {
        match self {
            FactionId::Empire => Color::srgb(0.85, 0.20, 0.20),       // 赤 (Red)
            FactionId::GrandDuchy => Color::srgb(0.15, 0.75, 0.85),   // 水色 (Cyan/Light Blue)
            FactionId::Federation => Color::srgb(0.65, 0.45, 0.32),   // 茶色 (Brown)
            FactionId::Republic => Color::srgb(0.95, 0.75, 0.15),     // 黄色 (Yellow)
            FactionId::Commonwealth => Color::srgb(0.25, 0.72, 0.35), // 緑 (Green)
            FactionId::Union => Color::srgb(0.18, 0.45, 0.90),        // 青 (Blue)
        }
    }

    pub fn accent_color(&self) -> Color {
        match self {
            FactionId::Empire => Color::srgb(1.0, 0.4, 0.4),
            FactionId::GrandDuchy => Color::srgb(0.45, 0.90, 1.0),
            FactionId::Federation => Color::srgb(0.85, 0.65, 0.50),
            FactionId::Republic => Color::srgb(1.0, 0.90, 0.40),
            FactionId::Commonwealth => Color::srgb(0.45, 0.90, 0.55),
            FactionId::Union => Color::srgb(0.45, 0.70, 1.0),
        }
    }
}

/// 外交態度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum DiplomaticRelation {
    /// ◎: 良好・友好 (Good / Friendly)
    Friendly,
    /// ◯: 普通・中立 (Normal / Neutral)
    Normal,
    /// △: 微妙・警戒 (Cautious / Tense)
    Cautious,
    /// ✕: 敵対・交戦 (Hostile / War)
    Hostile,
}

impl DiplomaticRelation {
    pub fn symbol(&self) -> &'static str {
        match self {
            DiplomaticRelation::Friendly => "◎",
            DiplomaticRelation::Normal => "◯",
            DiplomaticRelation::Cautious => "△",
            DiplomaticRelation::Hostile => "✕",
        }
    }

    pub fn text_ja(&self) -> &'static str {
        match self {
            DiplomaticRelation::Friendly => "友好 (良好)",
            DiplomaticRelation::Normal => "中立 (平常)",
            DiplomaticRelation::Cautious => "警戒 (緊張)",
            DiplomaticRelation::Hostile => "敵対 (交戦)",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            DiplomaticRelation::Friendly => Color::srgb(0.25, 0.85, 0.45),
            DiplomaticRelation::Normal => Color::srgb(0.70, 0.80, 0.90),
            DiplomaticRelation::Cautious => Color::srgb(0.95, 0.75, 0.25),
            DiplomaticRelation::Hostile => Color::srgb(0.95, 0.25, 0.25),
        }
    }
}

/// プレイヤーが操作する派閥
#[derive(Resource, Debug, Clone, Copy, Reflect)]
pub struct PlayerFaction(pub FactionId);

impl Default for PlayerFaction {
    fn default() -> Self {
        Self(FactionId::Empire)
    }
}

/// 全派閥の外交状態およびステータス管理
#[derive(Resource, Debug, Clone)]
pub struct FactionManager {
    /// 2派閥間の友好度数値 (-100 〜 +100)
    relations: HashMap<(FactionId, FactionId), i32>,
    /// 派閥ごとの領有タイル数
    territory_count: HashMap<FactionId, usize>,
}

impl Default for FactionManager {
    fn default() -> Self {
        let mut mgr = Self {
            relations: HashMap::new(),
            territory_count: HashMap::new(),
        };
        mgr.init_default_diplomacy();
        mgr
    }
}

impl FactionManager {
    /// doc/Faction.md の初期外交関係マトリクスを反映
    /// 表:外交関係
    /// ◎:良好、◯:普通、△:微妙
    ///  ABCDEF
    /// A-△◯◎◯◯△
    /// B--◯◯◎◎
    /// C---◯◯◯
    /// D----△△
    /// E-----◯
    fn init_default_diplomacy(&mut self) {
        use FactionId::*;

        let pairs = [
            // A (Empire)
            (Empire, GrandDuchy, DiplomaticRelation::Cautious), // A - B: △
            (Empire, Federation, DiplomaticRelation::Normal),   // A - C: ◯
            (Empire, Republic, DiplomaticRelation::Friendly),   // A - D: ◎
            (Empire, Commonwealth, DiplomaticRelation::Normal), // A - E: ◯
            (Empire, Union, DiplomaticRelation::Cautious),      // A - F: △
            // B (Grand Duchy)
            (GrandDuchy, Federation, DiplomaticRelation::Normal), // B - C: ◯
            (GrandDuchy, Republic, DiplomaticRelation::Normal),   // B - D: ◯
            (GrandDuchy, Commonwealth, DiplomaticRelation::Friendly), // B - E: ◎
            (GrandDuchy, Union, DiplomaticRelation::Friendly),    // B - F: ◎
            // C (Federation)
            (Federation, Republic, DiplomaticRelation::Normal), // C - D: ◯
            (Federation, Commonwealth, DiplomaticRelation::Normal), // C - E: ◯
            (Federation, Union, DiplomaticRelation::Normal),    // C - F: ◯
            // D (Republic)
            (Republic, Commonwealth, DiplomaticRelation::Cautious), // D - E: △
            (Republic, Union, DiplomaticRelation::Cautious),        // D - F: △
            // E (Commonwealth)
            (Commonwealth, Union, DiplomaticRelation::Normal), // E - F: ◯
        ];

        for (f1, f2, rel) in pairs {
            let score = match rel {
                DiplomaticRelation::Friendly => 60,
                DiplomaticRelation::Normal => 0,
                DiplomaticRelation::Cautious => -30,
                DiplomaticRelation::Hostile => -80,
            };
            self.set_relation_score(f1, f2, score);
        }

        for f in FactionId::ALL {
            self.territory_count.insert(f, 0);
        }
    }

    pub fn relation_between(&self, f1: FactionId, f2: FactionId) -> DiplomaticRelation {
        if f1 == f2 {
            return DiplomaticRelation::Friendly;
        }
        let score = self.get_relation_score(f1, f2);
        if score >= 40 {
            DiplomaticRelation::Friendly
        } else if score >= -15 {
            DiplomaticRelation::Normal
        } else if score >= -60 {
            DiplomaticRelation::Cautious
        } else {
            DiplomaticRelation::Hostile
        }
    }

    pub fn get_relation_score(&self, f1: FactionId, f2: FactionId) -> i32 {
        if f1 == f2 {
            return 100;
        }
        let (a, b) = Self::sort_pair(f1, f2);
        self.relations.get(&(a, b)).copied().unwrap_or(0)
    }

    pub fn set_relation_score(&mut self, f1: FactionId, f2: FactionId, score: i32) {
        if f1 == f2 {
            return;
        }
        let (a, b) = Self::sort_pair(f1, f2);
        self.relations.insert((a, b), score.clamp(-100, 100));
    }

    pub fn modify_relation(&mut self, f1: FactionId, f2: FactionId, delta: i32) {
        let current = self.get_relation_score(f1, f2);
        self.set_relation_score(f1, f2, current + delta);
    }

    pub fn territory_count(&self, faction: FactionId) -> usize {
        self.territory_count.get(&faction).copied().unwrap_or(0)
    }

    pub fn set_territory_count(&mut self, faction: FactionId, count: usize) {
        self.territory_count.insert(faction, count);
    }

    fn sort_pair(f1: FactionId, f2: FactionId) -> (FactionId, FactionId) {
        let idx1 = FactionId::ALL.iter().position(|&x| x == f1).unwrap_or(0);
        let idx2 = FactionId::ALL.iter().position(|&x| x == f2).unwrap_or(0);
        if idx1 < idx2 {
            (f1, f2)
        } else {
            (f2, f1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faction_default_diplomacy_matrix() {
        let mgr = FactionManager::default();

        // Faction.md 表:外交関係 の初期値検証
        // A - B: △ (Cautious)
        assert_eq!(
            mgr.relation_between(FactionId::Empire, FactionId::GrandDuchy),
            DiplomaticRelation::Cautious
        );
        // A - D: ◎ (Friendly)
        assert_eq!(
            mgr.relation_between(FactionId::Empire, FactionId::Republic),
            DiplomaticRelation::Friendly
        );
        // B - E: ◎ (Friendly)
        assert_eq!(
            mgr.relation_between(FactionId::GrandDuchy, FactionId::Commonwealth),
            DiplomaticRelation::Friendly
        );
        // B - F: ◎ (Friendly)
        assert_eq!(
            mgr.relation_between(FactionId::GrandDuchy, FactionId::Union),
            DiplomaticRelation::Friendly
        );
        // D - F: △ (Cautious)
        assert_eq!(
            mgr.relation_between(FactionId::Republic, FactionId::Union),
            DiplomaticRelation::Cautious
        );
    }

    #[test]
    fn test_relation_modification() {
        let mut mgr = FactionManager::default();
        let f1 = FactionId::Empire;
        let f2 = FactionId::Union; // 初期 △ (-30)

        assert_eq!(mgr.relation_between(f1, f2), DiplomaticRelation::Cautious);

        // 贈り物 (+80) で友好 (50) へ
        mgr.modify_relation(f1, f2, 80);
        assert_eq!(mgr.relation_between(f1, f2), DiplomaticRelation::Friendly);

        // -150 で敵対 (-100 clamp) へ
        mgr.modify_relation(f1, f2, -150);
        assert_eq!(mgr.relation_between(f1, f2), DiplomaticRelation::Hostile);
    }
}

