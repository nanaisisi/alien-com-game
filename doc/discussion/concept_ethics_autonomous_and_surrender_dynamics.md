# 倫理観・自律兵器・戦況包囲下の戦争疲労度設計仕様書 (Concept Ethics, Autonomous Kill-Chain & Surrender Dynamics)

本ドキュメントは、本作（4X大戦略＋サブタイル戦術・戦闘団制）における**「現実国家ドクトリンの下書きと倫理的昇華」「自律兵器による致死性（キルチェーンと責任の所在）」「包囲・孤立下での降伏裁量と戦争疲弊度（War Weariness）」**を、ゲーム内データ構造およびアルゴリズム・内部処理として体系化した設計仕様書である。

---

## 1. 倫理観の二大柱と基本方針

### 1.1 現実国家を下書きにする際の倫理的昇華

1. **抽象化と系譜化（特定の国名・民族の直接模倣の回避）**:
   - 現実の国家をそのまま出すのではなく、「地球時代の特定地政学・ドクトリンを引き継いだ架空の星間開拓勢力」として抽象化する。
2. **多面的な生存合理性（ステレオタイプ悪役の排除）**:
   - 損耗を許容する重工集団主義国家も、人命第一を掲げるハイテク連邦国家も、それぞれが「極限環境下で入植民を生き残らせるための独自の合理性と倫理基準」を持つものとして描く。
   - 数値パラメータ（`DoctrineProfile`）として表現し、思想の優劣ではなく「トレードオフ」として機能させる。

### 1.2 自律兵器（AI）が死因になることの倫理的ジレンマ

1. **人間性の希釈とキルチェーン**:
   - アルゴリズムによる即時排除は軍事的に圧倒的な反応速度を誇るが、「誰がその死に責任を負うのか」「降伏や非戦闘員の識別を誤った場合の責任」という法的・倫理的リスクを内包する。
2. **システムとしての具現化**:
   - 単なるテキスト演出にとどまらず、**「交戦規則（ROE）のスライダー」「IFF誤認インシデント」「国内世論・国際外交ペナルティ」**として内部処理化する。

---

## 2. 4X戦略レイヤー：国家ドクトリンと戦況感応モデル

### 2.1 国家ドクトリン構造体 (`DoctrineProfile`)

```rust
/// 国家ドクトリン・倫理プロファイル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctrineProfile {
    /// 勢力識別子
    pub faction_id: String,

    /// 人命重視係数 (0.0: 完全消耗許容 ～ 2.0: 極度の人命至上主義)
    /// - 高: 有人兵の戦死や包囲見殺しで世論が激しく悪化
    /// - 低: 損耗に寛容だが、兵士の士気や脱走率に別の影響
    pub casualty_aversion: f32,

    /// 自律兵器倫理規制 (0.0: 完全自律認可 ～ 1.0: 厳格な国際人道法遵守)
    /// - 高: 完全自律射撃(Free Fire)を禁止、軍事裁判・外交批判を恐れる
    /// - 低: アルゴリズムによる即時排除を容認
    pub autonomous_ethics_standard: f32,

    /// 外交世論感応度 (0.0: 孤立主義 ～ 1.0: 国際協調・同盟重視)
    pub diplomatic_sensitivity: f32,

    /// メディア統制率 (0.0: 報道完全自由 ～ 1.0: 報道管制・情報隠蔽)
    /// - インシデント発生時の国内世論炎上を一時的に遅延・減衰させる
    pub media_censorship_ratio: f32,
}
```

---

## 3. 包囲下での降伏裁量と戦争疲労度（War Weariness）の動的スケーリング

### 3.1 本質メカニクス：包囲されている側の立場と戦況コンテキスト

自軍部隊が敵に包囲・孤立（Encircled / Cut-Off）された際、司令部（プレイヤー）が**「降伏を許可するか、玉砕（死守）を強要するか」**を選択する。
この際、国民・社会が受ける戦争疲労度は、**「戦況の切迫度と大義の有無」**によって全く異なる。

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                 自軍部隊が包囲された時の「降伏許可 vs 玉砕命令」             │
├──────────────────────┬──────────────────────┬───────────────────────────────┤
│ ① 均衡・局所衝突     │ ② 絶望的劣勢 (滅亡寸前)│ ③ 圧倒的優勢 (大国の局所包囲) │
│ 「なぜ見殺しにする？」│ 「1分でも稼いでくれ」 │ 「救援が来る/軍の威信」       │
│  → 拒絶で世論【大炎上】│  → 降伏不許可も受容   │  → 組織・ドクトリンで許容     │
└──────────────────────┴──────────────────────┴───────────────────────────────┘
```

1. **均衡状態・必要性の薄い衝突（不毛な消耗戦）**:
   - 領土拡張や局所的な資源摩擦など、国家存亡に関わらない戦況。
   - ここで包囲部隊の降伏を許さず玉砕・見殺しにした場合、国民・遺族から「なぜ無駄な面子や無能な作戦で兵士を犬死にさせたのか！」と激しい批判が集中し、**戦争疲労度が激増（大炎上）**する。
2. **負けている極限状態（絶望的劣勢・国家存亡の危機）**:
   - 首都や主要植民地が迫られている祖国防衛戦。
   - 「包囲部隊が命を賭して時間を稼がなければ国全体が滅びる」という極限状態のため、玉砕死守命令が「崇高な自己犠牲」として社会的に受容され、**降伏不許可による戦争疲労度ペナルティは極小化**する。
3. **相手より数割段上の国力・戦力（圧倒的強者）**:
   - 国力に圧倒的な余裕があり、国民生活に戦争の影が落ちていないため、前線の局所的損害や強硬姿勢が国内世論の動揺（厭戦気分）に直結しにくい。

### 3.2 内部処理の実装仕様

```rust
/// 戦況バランスと切迫度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictContext {
    /// 絶望的劣勢 / 国家存亡の危機 (自国総合戦力比 < 0.5)
    DesperateSurvival,
    /// 均衡・膠着状態 / 不毛な消耗戦 (自国総合戦力比 0.7 ～ 1.4)
    GrindingStalemate,
    /// 圧倒的優勢 / 治安平定・掃討 (自国総合戦力比 > 2.0)
    OverwhelmingSuperiority,
    /// 通常
    Standard,
}

/// 孤立部隊への指令方針
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncircledStandingOrder {
    /// 降伏許可: 弾薬枯渇・包囲時に白旗を上げ捕虜となることを認める (兵士生存、捕虜化)
    AuthorizeSurrender,
    /// 徹底抗戦・死守: 最後の一兵まで交戦を強要 (玉砕・全滅リスク)
    FightToTheLast,
}

impl DoctrineProfile {
    /// 包囲下で降伏を許可せず、自軍兵士が玉砕・戦死した際の世論ペナルティ計算
    pub fn calculate_encircled_denial_penalty(
        &self,
        manned_soldiers_perished: u32,
        context: ConflictContext,
        war_justification: f32, // 戦争の大義 (0.0: 不毛な領土紛争 ～ 1.0: 祖国防衛)
    ) -> f32 {
        if manned_soldiers_perished == 0 {
            return 0.0;
        }

        let base_penalty = (manned_soldiers_perished as f32) * 15.0;

        // 戦況と大義による国民の憤激倍率 (Outrage Multiplier)
        let outrage_multiplier = match context {
            // 均衡・必要性の薄い衝突:
            // 大義が薄いほど国民の怒りは激化 (最大3.0倍超)
            ConflictContext::GrindingStalemate => 2.5 * (1.5 - war_justification.clamp(0.0, 1.0)),

            // 絶望的劣勢 (滅亡寸前):
            // 祖国防衛の時間稼ぎとして受容されるため、反発は極小 (0.2倍)
            ConflictContext::DesperateSurvival => 0.2,

            // 圧倒的優勢 (大国の局所戦):
            // 国力に余裕があり、社会全体の厭戦気分には直結しにくい (0.4倍)
            ConflictContext::OverwhelmingSuperiority => 0.4,

            ConflictContext::Standard => 1.0,
        };

        // 国民の人命重視度およびメディア統制率を反映
        let effective_penalty = base_penalty * outrage_multiplier * self.casualty_aversion;
        effective_penalty * (1.0 - (self.media_censorship_ratio * 0.5))
    }
}
```

---

## 4. 戦術レイヤー：自律兵器キルチェーンと包囲時の相互作用

### 4.1 交戦規則（ROE）と無人機自律射撃

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngagementPolicy {
    /// 厳格統制 (Human-in-the-loop):
    /// 有人指揮ノードの認証がないと射撃不能。誤射・国際法違反ゼロだが、ECM妨害下で射撃不能/遅延。
    StrictHumanApproval,

    /// 条件付き自律 (Human-on-the-loop):
    /// 平時は自律射撃、高リスク時（降伏疑い等）のみ有人承認待ち。
    SupervisedAutonomous,

    /// 完全自律即時排除 (Full Autonomous Kill / Free Fire):
    /// 通信途絶下でも超高速迎撃。ただしエッジAIによる誤認・降伏無視インシデントのリスク最大。
    FullAutonomousKill,
}
```

### 4.2 無人機がもたらす「包囲下の悲劇（ゲーム的ジレンマ）」

包囲された戦闘団に**有人指揮官＋自律無人機群（Tier 1/2）**が混在している場合、以下の特殊インシデントが発生する：

1. **降伏阻害インシデント（Friendly Trapped by Autonomous Escorts）**:
   - 有人部隊は弾薬が尽き、司令部も「降伏許可」を出している。
   - しかし、部隊内に残存する無人護衛機が `FullAutonomousKill` 設定のままであった場合、接近する敵部隊に対して自動反撃を継続してしまう。
   - これにより敵側から「偽装降伏」「徹底抗戦」と判定され、**人間兵士ごと掃討砲撃を浴びて全滅**する。
   - **戦後スキャンダル**: 「無人機の停止失敗・暴走により我が兵士が降伏の機会を奪われて死亡」として、通常の戦死ペナルティを大幅に超える政治的スキャンダルが発生する。

```rust
/// 包囲下での無人機による降伏阻害判定
pub fn check_autonomous_surrender_interference(
    encircled_units: &[Unit],
    policy: EngagementPolicy,
) -> bool {
    let has_surrendering_humans = encircled_units.iter().any(|u| u.is_manned && u.wants_surrender);
    let has_active_autonomous_weapons = encircled_units.iter().any(|u| !u.is_manned && u.is_operational);

    // 有人兵士が降伏しようとしているのに、完全自律兵器がアクティブな場合は降伏阻害フラグが成立
    has_surrendering_humans && has_active_autonomous_weapons && policy == EngagementPolicy::FullAutonomousKill
}
```

---

## 5. まとめとゲームデザイン的価値

1. **「正義の押し付け」ではない冷徹なシミュレーション**:
   - プレイヤーは「勝つため」「効率のため」に無人兵器の自律化や徹底抗戦命令を使うが、**戦況（均衡なのか、圧倒しているのか、滅亡寸前なのか）と国家体制（ドクトリン）に応じて、社会・国民から跳ね返ってくるコストが激変**する。
2. **大戦略（4X）と戦術（Battlegroup）の有機的結合**:
   - 戦術マップでの「無人機のROE設定」や「包囲された一戦闘団の引き際」という局所判断が、そのまま4X国家画面の「内閣支持率」「反戦デモ」「同盟関係」に直結する。
