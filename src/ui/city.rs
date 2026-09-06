use bevy::prelude::*;

use crate::faction::types::{FactionResources, PlayerFaction};
use crate::faction::FactionOutpost;
use crate::map::interaction::SelectedTile;
use crate::map::MapGrid;
use crate::state::AppState;
use crate::ui::theme::UiTheme;
use crate::ui::UiBlockMapInteraction;
use crate::unit::mesh::spawn_unit_model;
use crate::unit::types::CombatGroupType;
use crate::unit::Unit;

pub struct CityUiPlugin;

impl Plugin for CityUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CityModalState>()
            .add_systems(
                Update,
                (
                    toggle_city_modal_system,
                    city_modal_button_interaction_system,
                    city_production_action_system,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), close_city_modal);
    }
}

/// 都市管理画面のモーダル状態
#[derive(Resource, Default, Debug)]
pub struct CityModalState {
    pub is_open: bool,
    pub target_outpost_entity: Option<Entity>,
}

#[derive(Component)]
pub struct CityModalRoot;

#[derive(Component)]
pub enum CityButtonAction {
    ProduceUnit(CombatGroupType),
    UpgradeCity,
    Close,
}

#[derive(Component)]
pub struct CityNoticeText;

/// [C]キー または [Escape]キー、およびHUDボタン等による都市モーダルの開閉
#[allow(clippy::too_many_arguments)]
pub fn toggle_city_modal_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut modal_state: ResMut<CityModalState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected_tile: Res<SelectedTile>,
    player_faction: Res<PlayerFaction>,
    outposts_query: Query<(Entity, &FactionOutpost)>,
    faction_res: Res<FactionResources>,
    existing: Query<Entity, With<CityModalRoot>>,
) {
    let mut should_toggle = false;

    if keyboard.just_pressed(KeyCode::KeyC) {
        should_toggle = true;
    } else if keyboard.just_pressed(KeyCode::Escape) && modal_state.is_open {
        modal_state.is_open = false;
        for entity in &existing {
            commands.entity(entity).despawn();
        }
        return;
    }

    if should_toggle {
        modal_state.is_open = !modal_state.is_open;

        if modal_state.is_open {
            // 選択中のタイルにプレイヤー派閥の都市があるか確認
            let mut target_entity = None;
            if let Some(coord) = selected_tile.0
                && let Some((e, _outpost)) = outposts_query
                    .iter()
                    .find(|(_, o)| o.coord == coord && o.faction == player_faction.0)
                {
                    target_entity = Some(e);
                }

            // なければプレイヤー派閥の最初の都市を選択
            if target_entity.is_none()
                && let Some((e, _)) = outposts_query
                    .iter()
                    .find(|(_, o)| o.faction == player_faction.0)
                {
                    target_entity = Some(e);
                }

            modal_state.target_outpost_entity = target_entity;

            if let Some(target_e) = target_entity
                && let Ok((_, outpost)) = outposts_query.get(target_e)
            {
                spawn_city_modal(
                    &mut commands,
                    &asset_server,
                    outpost,
                    &faction_res,
                );
            } else {
                modal_state.is_open = false;
            }
        } else {
            for entity in &existing {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn close_city_modal(
    mut commands: Commands,
    mut modal_state: ResMut<CityModalState>,
    existing: Query<Entity, With<CityModalRoot>>,
) {
    modal_state.is_open = false;
    for entity in &existing {
        commands.entity(entity).despawn();
    }
}

/// 都市管理画面の構築
pub fn spawn_city_modal(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    outpost: &FactionOutpost,
    faction_res: &FactionResources,
) {
    let font_regular = asset_server.load(UiTheme::fonts().regular());
    let font_bold = asset_server.load(UiTheme::fonts().bold());
    let surfaces = UiTheme::surfaces();
    let text_col = UiTheme::text();
    let faction = outpost.faction;
    let p_col = faction.primary_color();
    let a_col = faction.accent_color();

    commands
        .spawn((
            CityModalRoot,
            UiBlockMapInteraction,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(940.0),
                        height: Val::Px(580.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(24.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(10.0)),
                        row_gap: Val::Px(16.0),
                        ..default()
                    },
                    BorderColor::all(a_col),
                    BackgroundColor(surfaces.overlay()),
                ))
                .with_children(|window| {
                    // --- 1. ヘッダー ---
                    window
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::bottom(Val::Px(12.0)),
                            border: UiRect::bottom(Val::Px(1.5)),
                            ..default()
                        })
                        .insert(BorderColor::all(surfaces.border()))
                        .with_children(|header| {
                            header
                                .spawn(Node {
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(4.0),
                                    ..default()
                                })
                                .with_children(|title_box| {
                                    title_box.spawn((
                                        Text::new(format!("⬢ {} // 基地司令部", outpost.name)),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(20.0),
                                            ..default()
                                        },
                                        TextColor(p_col),
                                    ));

                                    title_box.spawn((
                                        Text::new(format!(
                                            "所属: 国{}【{}】  都市ランク: Lv.{}  防衛値: 100/100",
                                            faction.code(),
                                            faction.name_ja(),
                                            outpost.level
                                        )),
                                        TextFont {
                                            font: font_regular.clone().into(),
                                            font_size: FontSize::Px(12.0),
                                            ..default()
                                        },
                                        TextColor(text_col.muted()),
                                    ));
                                });

                            // 閉じるボタン
                            header
                                .spawn((
                                    Button,
                                    CityButtonAction::Close,
                                    Node {
                                        padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                                        border: UiRect::all(Val::Px(1.0)),
                                        border_radius: BorderRadius::all(Val::Px(6.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BorderColor::all(surfaces.border()),
                                    BackgroundColor(surfaces.card()),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("閉じる [Esc]"),
                                        TextFont {
                                            font: font_regular.clone().into(),
                                            font_size: FontSize::Px(13.0),
                                            ..default()
                                        },
                                        TextColor(text_col.main()),
                                    ));
                                });
                        });

                    // --- 2. メインコンテンツ（左: 都市産出・状況 / 右: 生産キュー・部隊配備） ---
                    window
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            flex_grow: 1.0,
                            column_gap: Val::Px(20.0),
                            ..default()
                        })
                        .with_children(|content| {
                            // 左ペイン: 都市の産出・施設
                            content
                                .spawn(Node {
                                    width: Val::Px(340.0),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(14.0),
                                    padding: UiRect::all(Val::Px(16.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                })
                                .insert(BorderColor::all(surfaces.border()))
                                .insert(BackgroundColor(surfaces.card()))
                                .with_children(|left| {
                                    left.spawn((
                                        Text::new("■ 基地産出 & 保有資源"),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(14.0),
                                            ..default()
                                        },
                                        TextColor(a_col),
                                    ));

                                    // 資源グリッド
                                    left.spawn(Node {
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(8.0),
                                        ..default()
                                    })
                                    .with_children(|res_grid| {
                                        let stats = [
                                            ("⚡ エネルギー産出", format!("+{} /ターン (備蓄: {})", faction_res.energy_per_turn, faction_res.energy)),
                                            ("⚙ 工業生産力", format!("+{} /ターン (備蓄: {})", faction_res.production_per_turn, faction_res.production)),
                                            ("🔬 科学研究力", format!("+{} /ターン (備蓄: {})", faction_res.science_per_turn, faction_res.science)),
                                            ("🍎 食糧供給", format!("+{} /ターン (備蓄: {})", faction_res.food_per_turn, faction_res.food)),
                                        ];
                                        for (label, val) in stats {
                                            res_grid
                                                .spawn(Node {
                                                    flex_direction: FlexDirection::Row,
                                                    justify_content: JustifyContent::SpaceBetween,
                                                    padding: UiRect::axes(Val::Px(8.0), Val::Px(6.0)),
                                                    border_radius: BorderRadius::all(Val::Px(4.0)),
                                                    ..default()
                                                })
                                                .insert(BackgroundColor(Color::srgba(0.10, 0.14, 0.20, 0.6)))
                                                .with_children(|row| {
                                                    row.spawn((
                                                        Text::new(label),
                                                        TextFont {
                                                            font: font_regular.clone().into(),
                                                            font_size: FontSize::Px(12.0),
                                                            ..default()
                                                        },
                                                        TextColor(text_col.muted()),
                                                    ));
                                                    row.spawn((
                                                        Text::new(val),
                                                        TextFont {
                                                            font: font_bold.clone().into(),
                                                            font_size: FontSize::Px(12.0),
                                                            ..default()
                                                        },
                                                        TextColor(text_col.main()),
                                                    ));
                                                });
                                        }
                                    });

                                    // 基地設備・アップグレード
                                    left.spawn((
                                        Text::new("■ 基地拡張計画"),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(13.0),
                                            ..default()
                                        },
                                        TextColor(a_col),
                                    ));

                                    left.spawn((
                                        Button,
                                        CityButtonAction::UpgradeCity,
                                        Node {
                                            padding: UiRect::all(Val::Px(10.0)),
                                            border: UiRect::all(Val::Px(1.0)),
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            flex_direction: FlexDirection::Column,
                                            row_gap: Val::Px(4.0),
                                            ..default()
                                        },
                                        BorderColor::all(surfaces.accent()),
                                        BackgroundColor(surfaces.panel()),
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new(format!("基地モジュール改修 (Lv.{} → Lv.{})", outpost.level, outpost.level + 1)),
                                            TextFont {
                                                font: font_bold.clone().into(),
                                                font_size: FontSize::Px(12.0),
                                                ..default()
                                            },
                                            TextColor(text_col.main()),
                                        ));
                                        btn.spawn((
                                            Text::new("必要資源: 生産 60 / エネルギー 40 (ターン産出 +20%)"),
                                            TextFont {
                                                font: font_regular.clone().into(),
                                                font_size: FontSize::Px(11.0),
                                                ..default()
                                            },
                                            TextColor(text_col.muted()),
                                        ));
                                    });
                                });

                            // 右ペイン: 戦闘団編成・生産メニュー
                            content
                                .spawn(Node {
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(12.0),
                                    padding: UiRect::all(Val::Px(16.0)),
                                    border: UiRect::all(Val::Px(1.0)),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                })
                                .insert(BorderColor::all(surfaces.border()))
                                .insert(BackgroundColor(surfaces.card()))
                                .with_children(|right| {
                                    right.spawn((
                                        Text::new("■ 戦闘団・開拓部隊の編成 & 配備"),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(15.0),
                                            ..default()
                                        },
                                        TextColor(a_col),
                                    ));

                                    // 部隊生産ボタン一覧
                                    let units = [
                                        (
                                            CombatGroupType::Scout,
                                            "偵察戦闘団 (Scout Group)",
                                            "高速移動(移動力4)・広域視界。惑星探査・資源探索に最適。",
                                            "コスト: 生産 30 / エネルギー 15",
                                        ),
                                        (
                                            CombatGroupType::LightInfantry,
                                            "機動歩兵中隊 (Light Infantry)",
                                            "主力戦闘部隊。高耐久(HP100)・攻撃力25。都市防衛および制圧。",
                                            "コスト: 生産 45 / エネルギー 25",
                                        ),
                                        (
                                            CombatGroupType::Colonist,
                                            "惑星開拓団 (Colonist Expedition)",
                                            "新たな前哨基地を建設し領土を拡大する専門工兵ユニット。",
                                            "コスト: 生産 75 / 食糧 40",
                                        ),
                                    ];

                                    for (unit_type, name, desc, cost) in units {
                                        right
                                            .spawn((
                                                Button,
                                                CityButtonAction::ProduceUnit(unit_type),
                                                Node {
                                                    flex_direction: FlexDirection::Row,
                                                    justify_content: JustifyContent::SpaceBetween,
                                                    align_items: AlignItems::Center,
                                                    padding: UiRect::all(Val::Px(12.0)),
                                                    border: UiRect::all(Val::Px(1.0)),
                                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                                    ..default()
                                                },
                                                BorderColor::all(surfaces.border()),
                                                BackgroundColor(surfaces.panel()),
                                            ))
                                            .with_children(|btn| {
                                                btn.spawn(Node {
                                                    flex_direction: FlexDirection::Column,
                                                    row_gap: Val::Px(4.0),
                                                    ..default()
                                                })
                                                .with_children(|info| {
                                                    info.spawn((
                                                        Text::new(name),
                                                        TextFont {
                                                            font: font_bold.clone().into(),
                                                            font_size: FontSize::Px(13.0),
                                                            ..default()
                                                        },
                                                        TextColor(text_col.main()),
                                                    ));
                                                    info.spawn((
                                                        Text::new(desc),
                                                        TextFont {
                                                            font: font_regular.clone().into(),
                                                            font_size: FontSize::Px(11.0),
                                                            ..default()
                                                        },
                                                        TextColor(text_col.muted()),
                                                    ));
                                                });

                                                btn.spawn(Node {
                                                    flex_direction: FlexDirection::Column,
                                                    align_items: AlignItems::FlexEnd,
                                                    row_gap: Val::Px(4.0),
                                                    ..default()
                                                })
                                                .with_children(|action_side| {
                                                    action_side.spawn((
                                                        Text::new(cost),
                                                        TextFont {
                                                            font: font_regular.clone().into(),
                                                            font_size: FontSize::Px(11.0),
                                                            ..default()
                                                        },
                                                        TextColor(a_col),
                                                    ));
                                                    action_side.spawn((
                                                        Text::new("【即時配備】"),
                                                        TextFont {
                                                            font: font_bold.clone().into(),
                                                            font_size: FontSize::Px(12.0),
                                                            ..default()
                                                        },
                                                        TextColor(p_col),
                                                    ));
                                                });
                                            });
                                    }

                                    // 通知エリア
                                    right.spawn((
                                        CityNoticeText,
                                        Text::new("配備したい部隊を選択してください。生産後、基地周辺の空きタイルへ自動配備されます。"),
                                        TextFont {
                                            font: font_regular.clone().into(),
                                            font_size: FontSize::Px(12.0),
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.5, 0.8, 1.0)),
                                    ));
                                });
                        });
                });
        });
}

type CityButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
    ),
    (Changed<Interaction>, With<CityButtonAction>),
>;

/// ボタンのホバーエフェクト
pub fn city_modal_button_interaction_system(
    mut interaction_query: CityButtonInteractionQuery,
) {
    let surfaces = UiTheme::surfaces();
    let btn_theme = UiTheme::button(false);
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(btn_theme.hovered());
                *border_color = BorderColor::all(surfaces.accent());
            }
            Interaction::Pressed => {
                *bg_color = BackgroundColor(btn_theme.pressed());
            }
            Interaction::None => {
                *bg_color = BackgroundColor(surfaces.panel());
                *border_color = BorderColor::all(surfaces.border());
            }
        }
    }
}

/// 部隊生産・都市アップグレード実行システム
#[allow(clippy::too_many_arguments)]
pub fn city_production_action_system(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &CityButtonAction), Changed<Interaction>>,
    mut modal_state: ResMut<CityModalState>,
    mut faction_res: ResMut<FactionResources>,
    mut outposts_query: Query<(Entity, &mut FactionOutpost)>,
    map_grid: Res<MapGrid>,
    units_query: Query<&Unit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut notice_query: Query<&mut Text, With<CityNoticeText>>,
    modal_roots: Query<Entity, With<CityModalRoot>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            CityButtonAction::Close => {
                modal_state.is_open = false;
                for entity in &modal_roots {
                    commands.entity(entity).despawn();
                }
                return;
            }
            CityButtonAction::UpgradeCity => {
                if let Some(outpost_e) = modal_state.target_outpost_entity
                    && let Ok((_, mut outpost)) = outposts_query.get_mut(outpost_e) {
                        let prod_cost = 60;
                        let energy_cost = 40;
                        if faction_res.production >= prod_cost && faction_res.energy >= energy_cost {
                            faction_res.production -= prod_cost;
                            faction_res.energy -= energy_cost;
                            outpost.level += 1;
                            faction_res.production_per_turn += 3;
                            faction_res.energy_per_turn += 3;

                            if let Ok(mut notice) = notice_query.single_mut() {
                                **notice = format!(
                                    "✔ 基地拡張完了！ Lv.{} に昇格しました。（生産/エネルギー +3/ターン）",
                                    outpost.level
                                );
                            }
                        } else {
                            if let Ok(mut notice) = notice_query.single_mut() {
                                **notice = "❌ 資源が不足しています（必要: 生産 60 / エネルギー 40）".to_string();
                            }
                        }
                    }
            }
            CityButtonAction::ProduceUnit(unit_type) => {
                let (prod_cost, energy_cost, food_cost) = match unit_type {
                    CombatGroupType::Scout => (30, 15, 0),
                    CombatGroupType::LightInfantry => (45, 25, 0),
                    CombatGroupType::Colonist => (75, 0, 40),
                };

                if faction_res.production < prod_cost
                    || faction_res.energy < energy_cost
                    || faction_res.food < food_cost
                {
                    if let Ok(mut notice) = notice_query.single_mut() {
                        **notice = format!(
                            "❌ 資源が不足しています（必要: 生産 {} / エネルギー {} / 食糧 {}）",
                            prod_cost, energy_cost, food_cost
                        );
                    }
                    continue;
                }

                let Some(outpost_e) = modal_state.target_outpost_entity else {
                    continue;
                };
                let Ok((_, outpost)) = outposts_query.get(outpost_e) else {
                    continue;
                };

                let map_w = map_grid.width.max(1);
                let base_coord = outpost.coord;
                let neighbors = base_coord.neighbors_with_width(map_w);

                // 空いている隣接タイルを探す
                let mut target_coord = None;
                for candidate in neighbors {
                    if let Some(&terrain) = map_grid.terrain_data.get(&candidate)
                        && terrain.is_passable_ground()
                            && !units_query.iter().any(|u| u.coord == candidate)
                        {
                            target_coord = Some(candidate);
                            break;
                        }
                }

                // なければ前哨基地タイル
                let spawn_coord = target_coord.unwrap_or(base_coord);

                // 資源消費
                faction_res.production -= prod_cost;
                faction_res.energy -= energy_cost;
                faction_res.food -= food_cost;

                // ユニットのスポーン
                let terrain_height = map_grid
                    .terrain_data
                    .get(&spawn_coord)
                    .map(|t| t.height())
                    .unwrap_or(0.0);
                let world_pos = spawn_coord.to_world_pos(crate::map::HEX_RADIUS);

                let unit_entity = commands
                    .spawn((
                        Unit::new(outpost.faction, *unit_type, spawn_coord),
                        Transform::from_xyz(world_pos.x, terrain_height, world_pos.z),
                        Visibility::default(),
                    ))
                    .id();

                spawn_unit_model(
                    &mut commands,
                    unit_entity,
                    &mut meshes,
                    &mut materials,
                    outpost.faction,
                    *unit_type,
                );

                if let Ok(mut notice) = notice_query.single_mut() {
                    **notice = format!(
                        "✔ {} を配備しました！（座標: col:{}, row:{}）",
                        unit_type.display_name(),
                        spawn_coord.to_col_row_with_width(map_w).0,
                        spawn_coord.to_col_row_with_width(map_w).1
                    );
                }
            }
        }
    }
}
