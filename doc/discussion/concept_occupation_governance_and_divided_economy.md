# 占領地統治システムと軍政・経済力学設計書

(Occupation Governance, Military Administration, Remnant Subjugation & Divided Economics)

本ドキュメントは、軍事制圧後の被占領地・敗戦地域における**「軍政（占領軍・憲兵）と自治民政の緊張関係」**、**「対立（抵抗・サボタージュ）と取り込み（懐柔・傀儡化・協力者利権）」**、および**「分断経済・自国依存と自立経済のジレンマ」**をゲーム内データ構造・メカニクス・動的シミュレーションとして体系化した設計仕様書である。

---

## 1. コア思想：軍事制圧の完了は「統治戦争」の開始に過ぎない

従来の4X・大戦略ゲームにおける「占領すれば自動的に自国の領土となり、ターン経過で反乱度が下がるだけ」という受動的モデルを排する。

軍事侵攻で敵主力を壊滅させても、**「そこに数千万〜数十億の旧敵国民・官僚・警察・産業資本が残されている」**。
占領地統治の本質は、以下の三つ巴の摩擦をいかに制御・搾取・共存させるかの動的バランスゲームである：

1. **治安・治安維持の物理的コスト**:
   - 占領軍（正規師団）と憲兵・治安警察（MP/Gendarmerie）の配備負荷。
   - 兵力を貼り付ければ前線戦力が削られ、引き抜けば地下ゲリラと闇市場が急拡大する。
2. **取り込み（Co-optation）と対立（Insurgency）の二極化**:
   - 旧体制の官僚・富裕層・地域エリートを取り込み「自治民政」を機能させるか、それらを一掃して自国直轄の強権軍政を布くか。
   - 協力者は「裏切り者」として民衆の標的となり、弾圧を強めれば中間層全体が反乱分子へ転落する。
3. **分断経済の罠（依存 vs 自立）**:
   - 旧本国サプライチェーンから切り離された占領地は、飢餓と産業停止（ハイパーインフレ・失業）に直面する。
   - 自国通貨と本土補給に全面依存させれば「占領国の巨大な財政出血（ブラックホール）」となり、勝手に自立経済を回復させれば「再独立・反乱の物的基盤」を与える。

```text
                      ┌───────────────────────────────┐
                      │    軍事制圧完了・被占領地域   │
                      └──────────────┬────────────────┘
                                     │ 統治政策の選択
        ┌────────────────────────────┴────────────────────────────┐
        ▼                                                         ▼
  【直接軍政・強圧統治】                                       【間接統治・傀儡自治民政】
   ・憲兵・駐留軍による強権治安維持                              ・旧官僚・親占領派エリートの擁立
   ・全面的な資産接収・自国規格強制                              ・地域自治と旧通貨・法体系の一定温存
   ・反乱リスク極大化、治安維持費高騰                            ・サボタージュ低減、だが腐敗と裏切りリスク
        │                                                         │
        └────────────────────────────┬────────────────────────────┘
                                     ▼
                      ┌───────────────────────────────┐
                      │   経済分断と生存サプライチェーン   │
                      │  自国補給依存（莫大な財政負担）   │
                      │             vs                │
                      │  自立経済再建（潜在的反乱能力）   │
                      └───────────────────────────────┘
```

---

## 2. 占領統治の重層アクター構造

占領地タイル／地域は、互いに異なる動機と権限を持つアクターの力学で駆動される。

```text

┌────────────────────────────────────────────────────────────────────────┐
│                        ① 占領軍司令部 (HQ)                           │
│  ・最高責任者（方面軍司令官/軍政長官/総督）                             │
│  ・統治方針決定（略奪・同化・自治育成・治安最優先）                    │
│  ・本土政府（プレイヤー）からの予算・食糧・弾薬配分                    │
└───────────────────┬────────────────────────────────┬───────────────────┘
                    │ 指揮命令・査察                 │ 監督・権限委譲
                    ▼                                ▼
┌──────────────────────────────────────┐  ┌──────────────────────────────┐
│  ② 治安・法執行・哨戒機関群           │  │     ③ 自治民政 (Civil Admin)   │
│  【軍事・準軍事組織】                 │  │  ・旧体制官僚・傀儡首班      │
│  ・方面隊/正規軍 (Field Army)        │  │  ・地方評議会・経済界・ギルド│
│  ・最低限守備隊 (Garrison Troops)    │  │  ・食糧配給、徴税、基礎インフラ│
│  ・憲兵隊 (Military Police)          │  │  ・対立：二重帳簿、横領、通敵│
│  ・国境警備隊 (Border Guards)        │  └──────────────────────────────┘
│  ・沿岸警備隊 (Coast Guard)          │                 │
│  【本土派遣・文民法執行組織】         │                 ▼
│  ・国家警察 (National Police)        │  ┌──────────────────────────────┐
│  ・自治体警察 (Municipal Police)     │  │   現地補助警察 (Auxiliary)   │
│  ・税関・国境徴税庁 (Customs Agency) │  │   Schutzmannschaft           │
│  ・麻薬・禁制品取締 (DEA/Narcotics)  │  │  ・安価だが内通・暗殺の標的  │
│  ・独立特務捜査 (Anti-Corruption CID)│  └──────────────────────────────┘
│  ・戒厳令執行、密輸遮断、暗殺阻止     │
└──────────────────────────────────────┘  └──────────────────────────────┘
                    │                                │
                    └────────────────┬───────────────┘
                                     ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        ④ 現地住民・社会階層                           │
│  ・親占領派（協力者利権） / 日和見中間層（生活困窮） / 地下抵抗組織   │
└────────────────────────────────────────────────────────────────────────┘
```

---

### 2.1 自国国境から進出する法執行・治安機関の役割分担

自国領土から前線が進撃・拡大するにつれ、**「正規戦闘部隊」「軍事守備隊」「国境・水域管理」「文民警察・捜査機関」**が重層的に関与する。単一の「占領軍」ではなく、機関ごとの任務・権限・コストが異なる。

```text
  [自国本土] ───(旧国境)───► [後方兵站線・占領地境] ───(新境界/前線)───► [交戦前線]
      │                           │                                    │
  自治体警察                 国境警備隊 / 沿岸警備隊               方面軍正規部隊
  国家警察                   最低限守備隊 / 憲兵隊                 (機動打撃・前線突破)
  独立捜査機関               (密輸遮断・ゲリラ封じ込め)
```

1. **方面軍・正規軍（Frontline Combat Field Army）**:
   - **主任務**: 敵正規軍の撃滅、要衝突破。
   - **占領地での挙動**: 圧倒的火力による大規模蜂起の物理粉砕。
   - **限界**: 駐留維持費（弾薬・食糧）が極めて重く、前線から戦力を引き抜くことになる。パトロールや民衆尋問には不向きで、過剰暴力による巻き添え（Collateral Damage）で中間層を抵抗派に変えてしまう。
2. **最低限守備隊（Minimum Garrison Troops / Rear Area Security）**:
   - **主任務**: 重要補給ハブ、飛行場、弾薬庫、鉄道結節点の拠点防衛。
   - **挙動**: 低コストで配備可能な二線級・予備役部隊。拠点「防衛」には強いが、街区全体の治安巡回や地下ゲリラの捜索能力はない。
3. **憲兵隊（Military Police / Gendarmerie）**:
   - **主任務**: 軍紀維持、軍用道路の交通統制、占領地軍政令の執行。
   - **挙動**: 捜査値・摘発値が高く、後述の独立捜査機関と連携してサボタージュや反乱計画を早期摘発する。
4. **国境警備隊（Border Guards）**:
   - **主任務**: 旧国境・新占領境界線の検問・封鎖、越境密輸・スパイ潜入の阻止。
   - **挙動**: 自国本土と占領地、または分割占領された他国占領区との境界に展開。武器・禁制品・闇市場物資の流入を遮断。未配備だと本土の銃器規制が崩壊し、自国側にまで治安悪化が逆流する。
5. **沿岸警備隊・水上哨戒隊（Coast Guard / Riverine Patrol）**:
   - **主任務**: 港湾・沿岸部・河川輸送路の航行警備、海上密輸・機雷ゲリラの掃討。
   - **挙動**: 海洋・河川に面した都市のサプライチェーンを防衛。物資横流し用の密輸小型艇や、水路を利用した残党の浸透を阻止。
6. **国家警察・派遣機動隊（National Police / Riot Constabulary）**:
   - **主任務**: 都市部の暴動鎮圧、略奪防止、夜間外出禁止令の巡回執行。
   - **挙動**: 自国本土から派遣される治安専門の文民部隊。軍靴で踏みにじる印象を和らげ、過剰火力を抑えつつデモや暴動をコントロール可能。ただし自国本土の治安リソースを削るトレードオフが発生。
7. **自治体警察（Municipal / Local Police）**:
   - **主任務**: 現地都市の日常犯罪（窃盗・殺人・小規模闇市）の処理、交通整理。
   - **挙動**: 旧敵国の警察組織を審査・スクリーニングして再雇用、または傀儡自治政府が新設。最も安価で土地勘があるが、レジスタンスの内通者が混入するリスク（浸透度）が最も高い。
8. **税関・国境徴税庁（Customs & Border Revenue Agency）**:
   - **主任務**: 物資流入・流出の臨検、密貿易の摘発、関税・通行税の徴収、外貨・金塊流出阻止。
   - **挙動**: 占領地と本土、または周辺第三国との交易路を押さえ、自国の財政収入を直接確保する。未配備だと、占領地の希少資源や戦略物資が第三国へ不法流出し、敵対残党の資金源となる。
9. **麻薬・禁制品取締機関（DEA / Narcotics & Contraband Enforcement）**:
   - **主任務**: 麻薬・オピオイド・向精神薬、違法軍用薬品、兵器素材の密造・密売ネットワーク解体。
   - **挙動**: 経済が崩壊した占領地では、**民衆や軍閥が生存のために「麻薬・密造薬品・化学物質」を換金作物として栽培・精製**し、それが闇市場の基軸通貨と化す。取締機関はこれを破壊して地下資金源を枯渇させるが、現地の貧困農民・労働者との直接衝突を引き起こす。
10. **独立捜査機関・特務汚職監察（Independent Special Investigation / Anti-Corruption CID）**:

- **主任務**: 巨大カルテル摘発、二重帳簿・汚職官僚の摘発、自国軍幹部による物資横流しスキャンダルの捜査。
- **挙動**: 軍の指揮系統から独立した権限を持ち、傀儡政府の不正蓄財や、軍需物資（燃料・食糧）を闇市場へ横流しする腐敗将校を摘発・粛清する。治安指数ではなく「腐敗抑止値」「要人暗殺阻止率」を直接ブーストする。

---

### 2.2 国家体制（政体）による治安・法執行アーキテクチャの違い

プレイヤー（またはAI国家）の**国家体制（State Ideology / Regime Type）**によって、どの機関が存在し、どの省庁が占領地治安を牛耳るのかが根本から異なる。

```text
┌──────────────────────┬────────────────────────────────┬──────────────────────────────────────────┐
│ 国家体制モデル       │ 治安の主軸となる組織構造       │ 占領地統治における特性・得失             │
├──────────────────────┼────────────────────────────────┼──────────────────────────────────────────┤
│ ① 権威主義・東側型   │ 内務省国内軍 (Internal Troops) │ ・軍と同等の装甲車・重火器を持つ準軍事隊 │
│    (内務省・保安優位)│ 国家保安委員会 (Secret Police) │ ・治安鎮圧・反乱掃討は極めて強力だが、   │
│                      │                                │   恐怖統治により民心融和は絶望的         │
├──────────────────────┼────────────────────────────────┼──────────────────────────────────────────┤
│ ② 民主・連邦・法治型 │ 文民法執行 (Federal / Local)   │ ・軍の直接介入を制限（Posse Comitatus）  │
│    (分権・法手続き)  │ 専門機関 (FBI/DEA/Customs)     │ ・人権配慮で民心離反は防ぎやすいが、     │
│                      │ 司法審査・州兵 (National Guard)│   捜査・摘発手続きが遅く迅速さに欠ける   │
├──────────────────────┼────────────────────────────────┼──────────────────────────────────────────┤
│ ③ 純軍政・国家総動員 │ 憲兵司令部 (Kempeitai/Feldg.)  │ ・軍が行政・司法・警察の全権を直轄掌握   │
│    (国防軍独占型)    │ 要塞守備隊・防衛管区           │ ・即断即決、軍票の強制執行に長けるが、   │
│                      │                                │   経済再建や文民行政のノウハウが皆無     │
├──────────────────────┼────────────────────────────────┼──────────────────────────────────────────┤
│ ④ メガコーポ・企業型 │ 私設軍事警備会社 (PMC)         │ ・コスト採算性（ROI）で治安レベルを決定  │
│    (新自由主義・民営)│ アセット回収請負人             │ ・利益が出る鉱山・ドック周辺のみ重警備し │
│                      │                                │   採算の合わないスラム・市街地は完全放置 │
└──────────────────────┴────────────────────────────────┴──────────────────────────────────────────┘
```

1. **「内務省国内軍（Internal Troops / Rosgvardia / 武装警察）」の正体**:
   - **国防軍（外敵相手）**と**警察（一般犯罪相手）**の中間に位置する「体制保全のための巨大な第2軍隊」。
   - 正規軍と同等の歩兵戦闘車（IFV）や迫撃砲を装備し、防諜・国境警備・反乱鎮圧・要人警護・強制収容所警備までを一手に担う。
   - **ゲーム内挙動**: 占領地において「正規軍を前線から引き抜く必要がない」「憲兵よりも遥かに重武装で暴動を瞬殺できる」という絶大なメリットを持つが、維持費が国防予算と内務予算の二重払いとなり国家財政を激しく圧迫する。
2. **文民分権型（司法・専門捜査官型）**:
   - 税関、麻薬取締局、連邦捜査局、地方保安官のように組織が専門分化。
   - **ゲーム内挙動**: 汚職摘発や密輸阻止、違法薬物の資金源遮断には極めて高いボーナスを発揮するが、機関同士の縄張り争い（セクショナリズム）が発生し、統合作戦にタイムラグが生じる。

---

## 3. 「取り込み」と「対立」のダイナミクス

占領地域における人口は単一の「不穏度（Unrest）」ではなく、3つの派閥勢力比率としてシミュレートされる。

```text
  [ 協力・親占領派 ] ◄───(利権/治安保証)─── [ 日和見中間層 ] ───(貧困/過剰弾圧)───► [ 地下抵抗・反抗派 ]
       (Collaborators)                          (Pragmatic Civilians)                      (Insurgents)
```

### 3.1 取り込み（Co-optation）と「分割統治（Divide and Rule）」のメカニズム

占領側が現地エリート（特定部族・階級・宗教・旧冷遇マイノリティ）を懐柔する手段：

- **食糧・エネルギー・医薬品の優先配分**: 親占領派や指定協力コミュニティの居住区に生活物資を傾斜配分。
- **自治・警察権限の付与**: 特定の現地勢力に「補助警察（Schutzmannschaft）」の権限と火器を与え、旧支配層や対立コミュニティの監視・徴税を代行させる。
- **没収資産・利権の払い下げ**: 敵性エリート（逃亡・処刑された指導層）の工廠・農地・商業権を協力派に経営させ、既得権益で縛る。

### 3.2 分割統治が誘発する「内部対立の激化と反作用（Sectarian Fragmentation & Blowback）」

優先配分や特定グループの重用は、単なる「統治の円滑化」にとどまらず、**現地住民同士の潜在的亀裂を爆発させ、修復不能な内部抗争（内戦状態）へと波及する**。

```text
                            ┌─────────────────────────────────┐
                            │ 占領軍による特定勢力への優先配分│
                            └────────────────┬────────────────┘
                                             │
                      ┌──────────────────────┴──────────────────────┐
                      ▼                                             ▼
        【特権を与えられた協力派】                     【排除・冷遇された一般住民/旧主流派】
         ・生活保証と武装警察権を獲得                   ・飢餓・物資枯渇・屈辱感の蓄積
         ・私的報復や利権独占へ暴走                     ・「占領軍」と同時に「協力派」を激しく憎悪
                      │                                             │
                      └──────────────────────┬──────────────────────┘
                                             ▼
                              ┌─────────────────────────────┐
                              │  共同体間の報復リンチ・暗闘  │
                              │  （協力派集落への焼き討ち等）│
                              └──────────────┬──────────────┘
                                             │
                      ┌──────────────────────┴──────────────────────┐
                      ▼                                             ▼
        【占領軍への致命的跳ね返り】                   【対立の制御不能・泥沼化】
         ・占領軍が「部族抗争の審判・標的」に巻き込まれる   ・反乱が愛国抵抗から民族・宗派間虐殺へ変質
         ・協力派が敵を作りすぎて占領軍なしでは自立不能     ・兵站線が全方位の憎悪に晒され治安費が爆発
```

1. **水平的不平等の爆発（Horizontal Inequality）**:
   - 物資不足の極限下で「隣の地区（または対立部族・特定宗教集団）だけが占領軍から食糧や電気をもらっている」という状況は、占領軍への憎悪以上に**「同胞・隣人への強烈なルサンチマンと私怨」**を生む。
2. **補助警察の私兵化と私的報復（Settling Old Scores）**:
   - 占領軍から武器と権限を与えられた現地補助警察は、占領軍の治安維持のためではなく**「戦前の個人的な恨み、土地争い、部族間の抗争」の清算に武力を濫用**し始める。
   - これにより、無関係だった日和見中間層までが「自衛のために」地下抵抗組織や武装自衛団へ身を投じる。
3. **報復の連鎖と「泥沼の巻き添え（Blowback）」**:
   - 抵抗組織は占領軍の正規基地よりも、**警備の薄い「協力者の家族・村落・商店」を優先標的（見せしめ・焼き討ち・リンチ）**にする。
   - 協力派は占領軍に対して「あいつらを全員皆殺しにしてくれ」と過激な弾圧を要求し、占領軍がこれに乗って報復掃討を行うと、一般住民全体が完全に敵に回るという悪循環（Toxic Dependency）に陥る。

---

### 3.3 対立（Insurgency & Sabotage）の階層的発展

住民が抵抗勢力に流れるトリガーと、対抗形態の段階発展：

- **反乱への転落トリガー**:
  - **強制徴発・工場解体**: 食糧や機械設備を自国本土へ持ち去る（略奪）。
  - **不公平な配分と連座弾圧**: レジスタンス1人を捕らえるために街区全体を兵糧攻めにする。
  - **文化・イデオロギー弾圧**: 旧国の象徴破壊、強制同化教育。
- **対抗形態の4段階発展**:
  1. **消極的サボタージュ（Passive Resistance）**:
     - 工場での不良品混入、送電ケーブルの夜間切断、税・食糧の隠匿。
  2. **協力者狩り・非対称テロ（Sectarian Retaliation & Terror）**:
     - 傀儡官僚・補助警察官の暗殺、配給拠点や密告者の爆破。
  3. **社会的分断内戦（Communal Civil War under Occupation）**:
     - 親占領派民兵と反占領地下ゲリラが、占領軍の目の前で市街戦・虐殺を展開。占領軍は双方が衝突する緩衝地帯に釘付けにされる。
  4. **全面武力蜂起（General Insurrection）**:
     - 外部残党軍や第三国の工作員と呼応し、重火器を持ち出して都市部の一斉奪還を作戦展開。

---

## 4. 経済システムの分断：自国依存 vs 自立経済のジレンマ

占領地が直面する最大の問題は**「旧本国の経済圏から切り離されたことによる経済機能不全」**である。

```text
                               ┌───────────────────────────┐
                               │ 旧中央サプライチェーン喪失│
                               └─────────────┬─────────────┘
                                             │
                      ┌──────────────────────┴──────────────────────┐
                      ▼                                             ▼
        【ルートA：完全自国依存モデル】               【ルートB：自立経済再建モデル】
        (Dependent Imperial Colony)                   (Autonomous Regional Revival)
         ・自国通貨（軍票）の強制導入                   ・旧通貨・局所バーター経済容認
         ・食糧・燃料・原料を自国本土から補給           ・地域内サプライチェーンの自律再建
         ・製品はすべて本国規格へ換装                   ・現地企業・小規模工廠の稼働承認
         ─────────────────────────────                  ─────────────────────────────
         [利点]                                         [利点]
         ・反乱能力を物資的に骨抜きにできる             ・自国の補給・財政負担がゼロ
         ・本土経済への直接的貢献                       ・現地の失業率・不満が急速に鎮静化
         [欠点]                                         [欠点]
         ・莫大な補給スループットを占拠                 ・密造兵器・残党への横流し温床
         ・自国が兵站難に陥ると一気に飢餓暴動           ・将来の「再独立・反乱」の物的基盤になる
```

### 4.1 通貨と金融の分断

- **軍票（Military Scrip）の強制発行**:
  - 占領軍が物資調達のために刷る軍用通貨。自国本土では使えない。
  - 乱発すると信用崩壊を起こし、**闇市場（Black Market）**が誕生。旧敵国通貨や外貨、実物物資（缶詰・弾薬）が事実上の通貨となり、占領軍の経済統制から脱走する。

### 4.2 サプライチェーンの再編摩擦

- 局所の機械工場を動かそうにも、「特殊合金は惑星A、制御マイコンは星系B」から輸入していたため停止している。
- **選択肢**:
  1. **自国本土サプライチェーンへ接ぎ木（Splice into Empire Grid）**:
     - 高度な投資と規格換装が必要。完了すれば大工場として稼働。
  2. **粗悪現地自給（Downscale to Local Substitution）**:
     - 規格を下げ、現地で調達できる粗悪素材で低級品（旧式実包、粗悪トラック）を作らせる。

---

## 5. データ構造と内部処理仕様 (Rust)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type RegionId = u32;
pub type FactionId = String;
pub type UnitId = u64;

/// 占領地統治の全体状態
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupationGovernanceZone {
    pub region_id: RegionId,
    pub occupier_faction: FactionId,
    pub original_owner_faction: FactionId,

    /// 統治体制モード
    pub regime_type: OccupationRegimeType,

    /// 治安・警察力状態
    pub security_posture: SecurityPosture,

    /// 人口の政治的派閥比率 (合計 1.0)
    pub demographic_loyalty: DemographicLoyalty,

    /// 経済統制・分断モデル
    pub economic_policy: EconomicIntegrationPolicy,

    /// 経済健全度と依存度
    pub economy_status: OccupiedEconomyStatus,
}

/// 占領統治体制の類型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OccupationRegimeType {
    /// 直轄軍政（Military Governorate）: 最高司令官が直接全権を掌握。強圧的。
    DirectMilitaryRule { martial_law_strictness: f32 },

    /// 傀儡自治民政（Puppet Civil Administration）: 親占領派エリートを首班に据えた間接統治。
    PuppetCivilAdmin {
        collaborator_leader_loyalty: f32,
        civil_autonomy_level: f32, // 0.0=名ばかり, 1.0=完全内政自治
    },

    /// 資源・産業接収区（Extractive Concession）: 民生を放棄し、鉱山・ドックのみ武力死守。
    ResourceStrippingEnclave { defense_perimeter_size: u32 },
}

/// 治安・法執行機関の配備・稼働状況
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPosture {
    /// 方面隊・正規軍戦闘ユニット（前線突破・大蜂起撃滅用）
    pub frontline_combat_units: Vec<UnitId>,
    /// 最低限守備隊（拠点・兵站ハブ防衛用）
    pub rear_garrison_units: Vec<UnitId>,
    /// 憲兵隊（MP）大隊数（軍紀・交通線・軍政令執行）
    pub military_police_battalions: u32,
    /// 国境警備隊（境界検問・密輸密航遮断）
    pub border_guard_contingents: u32,
    /// 沿岸警備隊・水上哨戒隊（港湾・水路警備・海上ゲリラ阻止）
    pub coast_guard_cutters: u32,
    /// 本土派遣国家警察・機動隊（暴動鎮圧・文民統制）
    pub national_police_dispatched: u32,
    /// 現地自治体警察（旧警察再編・日常治安）
    pub municipal_police_strength: u32,
    /// 現地採用補助警察（Auxiliary）部隊数
    pub collaborator_auxiliaries: u32,
    /// 税関・国境徴税官数（密貿易阻止・関税確保）
    pub customs_officers: u32,
    /// 麻薬・禁制品取締捜査官数（地下資金源・換金薬物カルテル解体）
    pub narcotics_enforcement_agents: u32,
    /// 独立捜査機関（CID/CBI）特務班数（汚職・裏切り・テロ細胞摘発）
    pub independent_investigation_cells: u32,

    /// 総合治安指数 (0.0=無政府・ゲリラ天国, 1.0=完全制圧)
    pub pacification_index: f32,
    /// 地下組織の浸透度 (0.0=壊滅, 1.0=全土に地下網構築)
    pub insurgency_infiltration: f32,
    /// 越境密輸・外部兵器流入度 (0.0=完全封鎖, 1.0=フリーパス)
    pub cross_border_smuggling_rate: f32,
    /// 違法薬物・地下資金カルテル規模 (0.0=壊滅, 1.0=巨大資金源化)
    pub illicit_narcotics_index: f32,
    /// 官僚・軍幹部の腐敗癒着度 (0.0=清廉, 1.0=全面腐敗)
    pub corruption_level: f32,
}

/// 住民の派閥構成比
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemographicLoyalty {
    /// 協力者・親占領派（利権受領、補助警察、傀儡官僚）
    pub collaborator_ratio: f32,
    /// 日和見中間層（生活・治安の安定を最優先）
    pub pragmatic_neutrals: f32,
    /// 頑強な反抗・地下抵抗派（レジスタンス支持母体）
    pub active_resisters: f32,
}

/// 経済統合・分断政策
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EconomicIntegrationPolicy {
    /// 本土完全依存モデル：本国通貨（または軍票）強制、物資は本土補給網頼み
    ImperialDependence {
        scrip_inflation_rate: f32,
        homeland_subsidy_tons: u64,
    },
    /// 自立経済育成モデル：地域内サプライチェーン再建容認、局所通貨・独自通商容認
    RegionalAutarky {
        allowed_industrial_tier: u8, // 上位兵器の製造は制限
        smuggling_risk_factor: f32,  // 兵器横流しリスク
    },
    /// 徹底略奪・解体モデル：工場設備・資源を本土へ搬出し放棄
    StripAndAbandon,
}

/// 占領地経済ステータス
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupiedEconomyStatus {
    /// 失業率 (0.0 ～ 1.0)
    pub unemployment_rate: f32,
    /// 食糧・民生品充足率 (1.0以上で正常、0.5未満で飢餓暴動)
    pub subsistence_fulfillment: f32,
    /// 闇市場規模 (0.0=皆無, 1.0=表の経済を凌駕)
    pub black_market_scale: f32,
    /// 自国本土からの兵站負担（毎ターン消費される補給ポイント）
    pub logistical_drain_on_homeland: u64,
    /// 自国本土への上納価値（税収・資源）
    pub extracted_revenue_to_homeland: u64,
}

impl OccupationGovernanceZone {
    /// ターン毎の治安・法執行・民心・経済変動シミュレーション
    pub fn simulate_turn_cycle(&mut self) -> OccupationTurnEvents {
        let mut events = OccupationTurnEvents::default();

        // 1. 国境・水域・税関による外部浸透・密輸・関税徴収の計算
        let border_defense = (self.security_posture.border_guard_contingents as f32 * 0.25)
            + (self.security_posture.coast_guard_cutters as f32 * 0.2)
            + (self.security_posture.customs_officers as f32 * 0.15);
        self.security_posture.cross_border_smuggling_rate =
            (1.0 - border_defense).clamp(0.05, 1.0);

        // 税関による財政上納（密貿易阻止に比例して正規税収UP）
        let customs_revenue = (self.security_posture.customs_officers as u64) * 500;
        self.economy_status.extracted_revenue_to_homeland += customs_revenue;

        // 2. 麻薬取締による地下経済・反乱資金源の解体
        let drug_bust_power = self.security_posture.narcotics_enforcement_agents as f32 * 0.18;
        self.security_posture.illicit_narcotics_index =
            (self.security_posture.illicit_narcotics_index - drug_bust_power).clamp(0.0, 1.0);

        // 3. 独立捜査機関による腐敗・通敵・横流しの摘発
        let anti_corruption_sweep = self.security_posture.independent_investigation_cells as f32 * 0.15;
        self.security_posture.corruption_level =
            (self.security_posture.corruption_level - anti_corruption_sweep).clamp(0.0, 1.0);

        // 3. 複合治安抑止力と捜査摘発力の計算
        let civil_police_power = (self.security_posture.national_police_dispatched as f32 * 1.2)
            + (self.security_posture.municipal_police_strength as f32 * 0.6)
            + (self.security_posture.collaborator_auxiliaries as f32 * 0.5);
        let mp_power = self.security_posture.military_police_battalions as f32 * 1.4;
        let garrison_intimidation = (self.security_posture.rear_garrison_units.len() as f32 * 0.4)
            + (self.security_posture.frontline_combat_units.len() as f32 * 0.8);

        // 過剰軍事力による民心摩擦（正規軍が多すぎると巻き添えで中間層離反）
        if self.security_posture.frontline_combat_units.len() > 3 {
            let collateral_alienation = 0.02;
            self.demographic_loyalty.pragmatic_neutrals = (self.demographic_loyalty.pragmatic_neutrals - collateral_alienation).max(0.0);
            self.demographic_loyalty.active_resisters = (self.demographic_loyalty.active_resisters + collateral_alienation).min(1.0);
        }

        let total_pacification_power = civil_police_power + mp_power + garrison_intimidation;

        // 4. 経済状態が民心に与える影響
        if self.economy_status.subsistence_fulfillment < 0.6 {
            let radicalized = (0.6 - self.economy_status.subsistence_fulfillment) * 0.05;
            self.demographic_loyalty.pragmatic_neutrals = (self.demographic_loyalty.pragmatic_neutrals - radicalized).max(0.0);
            self.demographic_loyalty.active_resisters = (self.demographic_loyalty.active_resisters + radicalized).min(1.0);
            events.unrest_escalated = true;
        }

        // 5. サボタージュとテロの判定（密輸兵器と潜伏度に乗算）
        let effective_insurgency = self.demographic_loyalty.active_resisters
            * self.security_posture.insurgency_infiltration
            * (1.0 + self.security_posture.cross_border_smuggling_rate);

        if effective_insurgency > total_pacification_power * 0.015 {
            // 独立捜査機関や憲兵が暗殺を防ぎきれなかった場合
            if self.demographic_loyalty.collaborator_ratio > 0.1 && self.security_posture.independent_investigation_cells < 2 {
                self.demographic_loyalty.collaborator_ratio = (self.demographic_loyalty.collaborator_ratio - 0.02).max(0.0);
                events.collaborator_assassinated = true;
            }
            events.sabotage_on_industry = true;
        }

        // 6. 経済依存度の負担計算
        match &mut self.economic_policy {
            EconomicIntegrationPolicy::ImperialDependence { homeland_subsidy_tons, scrip_inflation_rate } => {
                self.economy_status.logistical_drain_on_homeland = *homeland_subsidy_tons;
                if *scrip_inflation_rate > 0.2 {
                    self.economy_status.black_market_scale = (self.economy_status.black_market_scale + 0.05).min(1.0);
                }
            }
            EconomicIntegrationPolicy::RegionalAutarky { smuggling_risk_factor, .. } => {
                self.economy_status.logistical_drain_on_homeland = 0;
                self.security_posture.insurgency_infiltration =
                    (self.security_posture.insurgency_infiltration + (*smuggling_risk_factor * 0.02)).min(1.0);
            }
            EconomicIntegrationPolicy::StripAndAbandon => {
                self.economy_status.subsistence_fulfillment = 0.2;
                self.economy_status.unemployment_rate = 0.85;
            }
        }

        events
    }
}

/// ターン処理結果イベント
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OccupationTurnEvents {
    pub collaborator_assassinated: bool,
    pub sabotage_on_industry: bool,
    pub unrest_escalated: bool,
    pub open_insurgency_sparked: bool,
}
```

---

## 6. プレイヤーの戦略的ジレンマとUI/ゲーム体験

プレイヤーは占領地管理画面において、常に**「痛みを伴うトレードオフ」**を突きつけられる：

1. **治安維持のコスト配分**:
   - 前線の正規軍を治安維持に縛り付けて攻勢を鈍らせるか？
   - 現地人を集めた「補助警察」を編成して安価に済ませるか（ただし内通・暗殺の危険）？
2. **協力者の保護と民心のバランス**:
   - 協力派官僚を守るために街区に厳しい夜間外出禁止令を布けば、中間層の反感を買いレジスタンスを増やす。
   - 融和政策をとれば、協力者が次々と暗殺され、行政組織が崩壊する。
3. **兵站のブラックホールか、再反乱の火種か**:
   - **完全依存ルート**: 本土から毎ターン膨大な食糧と補給物資を輸送船団で送り込み続ける。本土の生産が途絶えた瞬間、占領地全土が一斉に飢餓暴動を起こす。
   - **自立経済ルート**: 現地の工場や農地を再稼働させて自活させる。治安は安定し負担も減るが、数年後には「独自規格の兵器を密造できる独立軍閥」に化けるリスクを孕む。

このモデルにより、戦後の占領統治は**「軍事制圧後の消化不良・統治疲弊」**をリアルかつ劇的に描き出す。
