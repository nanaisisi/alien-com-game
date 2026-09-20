# ドキュメント構成・目録 (Documentation Index)

本プロジェクト（Alien Com Game）における各種仕様書・検討資料の構成および索引です。

---

## 📁 全体ディレクトリ構造

```text
doc/
├── INDEX.md                        # 本ファイル（ドキュメント目録）
├── GAME_DESIGN.md                  # ゲーム企画・コア仕様設計書
├── Faction.md                      # 勢力一覧・固有ボーナス/特性定義
└── discussion/                     # 詳細設計・発展仕様・アイデア検討ドキュメント群（計19件）
    ├── sub_tile_seamless_connection.md
    ├── future_concept_subtile_industry.md
    ├── concept_weapon_module_commonality.md
    ├── example_weapon_modules_cases.md
    ├── concept_future_legged_unmanned_rules.md
    ├── concept_ethics_autonomous_and_surrender_dynamics.md
    ├── concept_salvage_scuttling_and_compatibility.md
    ├── concept_dynamic_arsenal_geospatial.md
    ├── concept_industry_structure.md
    ├── concept_industry_module_mapping.md
    ├── concept_automation_systems.md
    ├── concept_diplomacy_relations.md
    ├── concept_occupation_governance_and_divided_economy.md
    ├── concept_capitulation_remnants_and_supply_chain.md
    ├── concept_frontier_civilization_and_ecosystem_dynamics.md
    ├── concept_event_situational.md
    ├── concept_event_stochastic.md
    ├── concept_in_game_wiki.md
    └── concept_viewer_separation_wiki_and_live.md
```

---

## 📑 1. ルート基本ドキュメント

* **[GAME_DESIGN.md](./GAME_DESIGN.md)**：ゲーム企画・基本仕様設計書（マスタードキュメント）
  * ゲームコンセプト（4X SFターン制戦略、Civ:BEライク）
  * 全体マップ（大戦略層）と単一タイルマップ（戦術層）の二重スケール構造
  * 固定編成・戦闘団制（増強師団相当）による小部隊運用
  * 原住エイリアンとの共存／支配／排除の関係性システム
  * 技術ツリー、内政・資源、外交の基本方針
* **[Faction.md](./Faction.md)**：登場勢力設定仕様書
  * 母星の歴史・大戦停戦を経て入植した架空の6大国家概念（帝国A、大公国B、連邦C、共和国D、共同体E、連合F）
  * 各国の体制・文化圏背景、母星と入植惑星での外交関係（具体的な国名は未確定・継続検討事項）
  * アフィニティ（思想・適応路線）や経済/軍事ボーナスの基本方向性

---

## 💡 2. 詳細検討・発展仕様（`doc/discussion/`）

各システム・メカニクスごとに詳細な検討と仕様アイデアがまとめられています。

### ① マップ構造・サブタイル運用（スケール連携）

全体マップ（大戦略）とサブタイル（戦術マップ）の連携に関する仕様検討。

* **[sub_tile_seamless_connection.md](./discussion/sub_tile_seamless_connection.md)**
  * 全体マップとサブタイル（戦術マップ）のシームレスな接続性。
  * タイル境界（エッジ）での部隊進入・離脱・追撃、隣接タイルからの増援・支援砲撃の物理的結合ルール。
* **[future_concept_subtile_industry.md](./discussion/future_concept_subtile_industry.md)**
  * サブタイル上に存在する小規模施設・インフラ（パイプライン、送電線、観測所、防衛タワー等）の配置と、戦術戦での破壊・防衛インタラクション。

### ② 軍事・兵器モジュール・戦術運用

ユニットの設計、兵器モジュール化、無人・自律兵器、戦場ルール。

* **[concept_weapon_module_commonality.md](./discussion/concept_weapon_module_commonality.md)**
  * 車体／シャシーと兵装モジュールの共通化設計（モジュラー兵器システム）。生産効率と改修自由度の両立。
* **[example_weapon_modules_cases.md](./discussion/example_weapon_modules_cases.md)**
  * 兵装モジュール・シャシー組み合わせの実装具体例（対空砲架、レールガン、ミサイルポッド、電子戦装備等）。
* **[concept_future_legged_unmanned_rules.md](./discussion/concept_future_legged_unmanned_rules.md)**
  * 多脚・二脚歩行兵器の特性検討（万能ではなく構造の複雑さ・整備負荷・脆弱性等の「不便さ」を伴うロマン枠としての議論深化）、無人ドローン・自律機械ユニットの階層（Tier）と通信途絶時の行動ルーチン。
* **[concept_ethics_autonomous_and_surrender_dynamics.md](./discussion/concept_ethics_autonomous_and_surrender_dynamics.md)**
  * 自律兵器運用における倫理的課題・国際条約、敵部隊の降伏判定と捕虜・無力化ユニットの処理。
* **[concept_salvage_scuttling_and_compatibility.md](./discussion/concept_salvage_scuttling_and_compatibility.md)**
  * 戦場での残骸回収（サルベージ）、鹵獲阻止のための自爆・自沈（スカットリング）、異文明・敵性技術モジュールの規格互換性とリバースエンジニアリング。
* **[concept_dynamic_arsenal_geospatial.md](./discussion/concept_dynamic_arsenal_geospatial.md)**
  * 地理・地形特性に応じた戦力展開、補給工廠（アーセナル）の空間的配置と動的兵站ラインの防衛。

### ③ 産業・サプライチェーン・自動化

生産ライン、モジュール製造、煩雑さを軽減する自動化システム。

* **[concept_industry_structure.md](./discussion/concept_industry_structure.md)**
  * 素材採掘・精錬・中間部品加工・最終製品組立に至る多段階産業ツリーの基本構造。
* **[concept_industry_module_mapping.md](./discussion/concept_industry_module_mapping.md)**
  * 工業施設と生産モジュールのマッピング、工場ラインの構成と兵器生産の接続。
* **[concept_automation_systems.md](./discussion/concept_automation_systems.md)**
  * 自動輸送ルート設定、定期哨戒（パトロール）、前線への自動補給ロジック。

### ④ 外交・占領統治・戦後処理

勢力間の関係性、戦争終了後の国家解体と統治メカニクス。

* **[concept_diplomacy_relations.md](./discussion/concept_diplomacy_relations.md)**
  * 国家間外交、通商協定、防衛協定、エイリアンに対する共同方針。
* **[concept_occupation_governance_and_divided_economy.md](./discussion/concept_occupation_governance_and_divided_economy.md)**
  * 敵都市・領土の占領統治方式（軍政・傀儡政権・直接併合）、治安度維持、分断された経済圏の管理。
* **[concept_capitulation_remnants_and_supply_chain.md](./discussion/concept_capitulation_remnants_and_supply_chain.md)**
  * 敗戦国の無条件/条件付き降伏、残存武装勢力（レジスタンス/軍閥）のゲリラ化とサプライチェーンの寸断・再編。

### ⑤ 世界観・生態系・惑星開拓

舞台となる惑星環境、初期開拓フェーズの設定。

* **[concept_frontier_civilization_and_ecosystem_dynamics.md](./discussion/concept_frontier_civilization_and_ecosystem_dynamics.md)**
  * 惑星開拓初期の低文明・現地適応設定、原住エイリアンや現地植物の生態サイクル・環境変化に対する動的反応。

### ⑥ イベントシステム

ゲーム進行中に発生する動的イベント。

* **[concept_event_situational.md](./discussion/concept_event_situational.md)**
  * プレイヤーの特定行動（過度な環境破壊、特定技術研究、特定外交方針等）にトリガーされて発生するシチュエーショナル・イベント。
* **[concept_event_stochastic.md](./discussion/concept_event_stochastic.md)**
  * 気象激変、地殻変動、エイリアンの異常活性化など、確率的・ランダムに発生する環境イベント。

### ⑦ UI・情報閲覧システム

情報表示とUX設計。

* **[concept_in_game_wiki.md](./discussion/concept_in_game_wiki.md)**
  * ゲーム内百科事典（Civopedia相当）の構造、調査・研究進捗に応じた動的記事アンロック。
* **[concept_viewer_separation_wiki_and_live.md](./discussion/concept_viewer_separation_wiki_and_live.md)**
  * 静的情報（Wiki）とリアルタイム戦況・ユニット状態（Live Viewer）のUI分離・統合アプローチ。
