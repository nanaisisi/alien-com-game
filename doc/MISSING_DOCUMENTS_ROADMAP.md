# 未作成・不足ドキュメント体系と開発ロードマップ (Missing Documents & Expansion Roadmap)

本ドキュメントは、本作（4X大戦略＋サブタイル戦術・戦闘団制）における**「現在何が不足しており、今後どのドキュメントを段階的に作成・発展させていくべきか」**を、最新のディレクトリ階層および設計進捗を踏まえて体系的にまとめた設計ロードマップ管理書です。

---

## 1. ドキュメント整備状況の俯瞰（成熟度マトリクス）

現在、本プロジェクトは「軍事・戦術・兵器設計」「マップ・タイル進展」「統治・戦後処理・方面軍」といったハイレベルな構造設計が大幅に深化・ディレクトリ分類（7分野）されました。
一方で、**「ゲーム内で実際に扱う資源・部品・弾薬の実体定義」「Civ:BE型技術ウェブ・アフィニティツリー」「偶発的な外交危機イベント」「エイリアン巣窟の詳細運用」「操作系UI（ステアリングUI）」が明確な空白・不足**として残されています。

```text
【領域別のドキュメント充実度（最新点検）】
・軍事・兵器・戦術ルール : ★★★★★ (完了：モジュール共通化、不便さを伴う多脚、無人機、残骸、降伏)
・マップ・領土・タイル   : ★★★★★ (完了：シームレス接続、施設破壊、脱都市拡張、Tier進展、戦場過密)
・産業・サプライチェーン : ★★★★☆ (充実：歴史的進化、新規性と継続性、ブートローダー産業、輸送ウインドウ)
・外交・統治・方面軍     : ★★★★☆ (充実：方面軍統治、合意吸収、降伏処理、二重経済、失踪者・自然国境)
・世界観・人口・進行骨格 : ★★★★☆ (充実：人工子宮・種分化、開拓生態、動的情勢推移、勝利条件骨格)
──────────────────────────────────────────────────────────────────────────
・イベント・ナラティブ   : ★★★☆☆ (枠組み・カタログ連鎖完了。外交危機・偶発エスカレーションが未作成)
・基本経済・リソース品目 : ★★★★★ (完了：一次/二次/消費材、弾薬規格の分化再統合、陸海工業食料)
・研究開発・技術ツリー   : ★☆☆☆☆ (ウェブ構造、アフィニティ分岐、リバース工学ブーストが未作成)
・エイリアン生態・巣窟   : ★☆☆☆☆ (世界観はあるが、巣窟の繁殖・ミズマ拡散/浄化・誘導捕獲が未作成)
・操作系UI・ステアリング : ★☆☆☆☆ (Wiki/Viewerはあるが、戦闘団・自動産業への政策ステアリングUIが未作成)
```

---

## 2. 不足ドキュメント一覧（作成すべき仕様書群）

今後、段階的に策定・保存していくべきドキュメントを以下の4領域・計5件として定義します。

### ① 経済・資源品目・物流循環領域（策定済）

- **ドキュメント名**: [`./discussion/industry_logistics/concept_resource_economy_and_logistics.md`](./discussion/industry_logistics/concept_resource_economy_and_logistics.md)（資源カテゴリ・物資格納・食料と弾薬規格の進化流通仕様書）
- **ステータス**: **初版策定済**
- **策定内容**:
  - **弾薬規格ダイナミクス**: 冷戦直後型地球共通規格から、同時並行開発による分化、後発追従・同盟協定・事実上の標準化による再統合。
  - **食料システム（3大アプローチ）**: 陸（土壌・温室・畜産）／海（閉鎖養殖・海洋資源）／工業（細胞培養リアクター）。気候タイルの自活投資と、異星現地生物採取（あるものを食うリスク）。
  - **3層品目体系**: 一次資源（エネルギー、構造鉱物、機能鉱物、バイオ原質）、二次中間財（基礎部品、論理コア、装甲板、レーション）、消費財（弾薬、燃料、交換部品）。

### ② 技術ツリー・研究開発（R&D）領域（最優先・ゲーム骨格）

- **ドキュメント候補名**: [`./discussion/world_progression/concept_technology_tree_structure.md`](./discussion/world_progression/concept_technology_tree_structure.md)（研究開発・技術ウェブ構造設計書）
- **優先度**: **高（最優先）**
- **目的・策定すべき内容**:
  - **放射状Web構造**: Civ:BE型の中央から外縁へ広がる非線形ツリー（幹技術と枝技術）。
  - **アフィニティ（適応思想）分岐**: 地球純正（地球型重工業・装甲重視）、現地適応（生体工学・ミズマ耐性・エイリアン共生）、サイバネティック（自律機械・電子知性・無人化）。
  - **自律研究とステアリング**: 国内研究機関の自動進捗に対し、プレイヤーが予算・重点分野を介入（ステアリング）。
  - **実戦・残骸ブースト**: 敵性残骸の回収・解析による特定技術の飛躍的アンロック。

### ③ 外交危機・偶発紛争（エスカレーション）領域

- **ドキュメント候補名**: [`./discussion/events/concept_diplomatic_incidents_and_crises.md`](./discussion/events/concept_diplomatic_incidents_and_crises.md)（外交危機・偶発紛争イベント設計書）
- **優先度**: **中**
- **目的・策定すべき内容**:
  - **グレーゾーン事態**: 国境未定地での密輸・不法資源採掘、民間探査船の領犯・拿捕、通商破壊工作。
  - **エスカレーション・ラダー**: 警告・抗議 → 経済制裁・資産凍結 → 局地衝突・代理紛争 → 最後通牒・全面戦争に至る動的な緊張度メーター。
  - **第三極（失踪者・軍閥）の介在**: 国境地帯の自発的失踪者コミュニティを巡る情報戦と宣戦口実（開戦事由/Casus Belli）の生成。

### ④ エイリアン生態サイクル・巣窟（Nest）運用領域

- **ドキュメント候補名**: [`./discussion/world_progression/concept_alien_lifecycle_and_nest_mechanics.md`](./discussion/world_progression/concept_alien_lifecycle_and_nest_mechanics.md)（エイリアン生態・巣窟インタラクション詳細仕様書）
- **優先度**: **中**
- **目的・策定すべき内容**:
  - **巣窟（Nest）のライフサイクル**: 休眠、活性化、繁殖期、暴走（大侵入/Swarm）のトリガー。
  - **瘴気（ミズマ）ダイナミクス**: 巣窟からの自律拡散、地形侵食、人類部隊への継続腐食ダメージと現地適応部隊のステルス/回復バフ、浄化プラントによる抑制。
  - **生態兵站・外交的利用**: フェロモン誘引弾による敵対国タイルへの誘導、生体組織の定期採取（共生協定）と報復襲撃リスク。

### ⑤ プレイヤー操作・ステアリングUI/UX領域

- **ドキュメント候補名**: [`./discussion/ui_wiki/concept_doctrine_steering_ui.md`](./discussion/ui_wiki/concept_doctrine_steering_ui.md)（国家ドクトリン・自動化ステアリングUI仕様書）
- **優先度**: **中**
- **目的・策定すべき内容**:
  - **「指示せず誘導する」UI設計**: 戦闘団の固定編成や国内産業の自動更新に対し、プレイヤーがどのようなスライダー・ドクトリン方針カードで干渉するか。
  - **移行ラグの視覚化**: ドクトリン変更（例: 共通化率優先から新鋭テック導入へ）に伴う工廠ライン転換と、前線部隊の整備ペナルティ期間のダッシュボード表示。

---

## 3. 作成済みドキュメントの再整理・進捗反映一覧

これまでに作成され、検証が完了したドキュメントの現行ステータスです。

| ドキュメント名（相対パス） | 格納カテゴリ | 概要・達成状況 |
| :--- | :--- | :--- |
| **[`./discussion/events/concept_event_catalog_and_chains.md`](./discussion/events/concept_event_catalog_and_chains.md)** | `events/` | 8大ジャンルの具体イベントカタログと不可逆な連鎖イベント（初版完了） |
| **[`./discussion/world_progression/concept_game_progression_and_victory.md`](./discussion/world_progression/concept_game_progression_and_victory.md)** | `world_progression/` | 動的情勢推移と5大勝利条件、クリア後継続プレイ保証（初版完了） |
| **[`./discussion/map_tile/concept_tile_centric_expansion.md`](./discussion/map_tile/concept_tile_centric_expansion.md)** | `map_tile/` | 脱・都市中心主義のタイル拡張、物理インフラ延伸（初版完了） |
| **[`./discussion/map_tile/concept_tile_progression_system.md`](./discussion/map_tile/concept_tile_progression_system.md)** | `map_tile/` | 全タイル共通Tier 0〜4進展、1タイル1部隊制と戦場過密大混戦（初版完了） |
| **[`./discussion/industry_logistics/concept_bootloader_and_national_supply_chains.md`](./discussion/industry_logistics/concept_bootloader_and_national_supply_chains.md)** | `industry_logistics/` | 機械が機械を作るブートローダー産業、金型・歩留まりと国家サプライチェーン（初版完了） |
| **[`./discussion/world_progression/concept_demographics_ectogenesis_and_speciation.md`](./discussion/world_progression/concept_demographics_ectogenesis_and_speciation.md)** | `world_progression/` | 人工子宮（エクトジェネシス）による人口爆発と適応種分化（初版完了） |
| **[`./discussion/industry_logistics/concept_theater_operations_and_joint_sectors.md`](./discussion/industry_logistics/concept_theater_operations_and_joint_sectors.md)** | `industry_logistics/` | 方面軍（Theater Command）組織、専念領域と共同管理領域（初版完了） |
| **[`./discussion/events/concept_faction_consensus_and_standardization.md`](./discussion/events/concept_faction_consensus_and_standardization.md)** | `events/` | 空気感による完全合意吸収、相手国承認のタイル移管、通信規格調達3方式（初版完了） |
| **[`./discussion/map_tile/concept_maritime_governance_and_ocean_engineering.md`](./discussion/map_tile/concept_maritime_governance_and_ocean_engineering.md)** | `map_tile/` | 海上統治・海洋工学（埋め立て/固定式/半固定式/メガフロート、海上都市、シーレーン）（初版完了） |

---

## 4. 段階的作業ロードマップ（今後の実施順序）

不足ドキュメント間の依存関係（前提条件）に基づき、以下の順序で作成を進めます。

```text
【フェーズ1: 大戦略ゲームの数値・入力基盤（最優先）】
  ├─ ① concept_resource_economy_and_logistics.md
  │    └─ 採掘リソース、中間部品、弾薬消費、通貨の品目カタログを確定
  └─ ② concept_technology_tree_structure.md
       └─ Civ:BE型ウェブツリー、アフィニティ3大思想、残骸回収ブーストを確定

【フェーズ2: 外交緊張と生態系インタラクション】
  ├─ ③ concept_diplomatic_incidents_and_crises.md
  │    └─ 国境小競り合いから全面戦争へのエスカレーション・危機管理を確定
  └─ ④ concept_alien_lifecycle_and_nest_mechanics.md
       └─ 巣窟の活性・ミズマ拡散、誘導兵器、共生採取のルールを確定

【フェーズ3: プレイヤー操作・統率インターフェース】
  └─ ⑤ concept_doctrine_steering_ui.md
       └─ 固定編成・自律産業への大戦略ステアリングUI/UX仕様を確定
```
