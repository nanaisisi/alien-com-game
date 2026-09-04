use bevy::prelude::*;

use crate::faction::types::{FactionId, PlayerFaction};
use crate::map::settings::{MapConfig, MapSize, PlanetEnvironment};
use crate::state::AppState;

pub struct FactionSelectPlugin;

impl Plugin for FactionSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedFactionMenu>()
            .add_systems(OnEnter(AppState::FactionSelect), setup_faction_select_ui)
            .add_systems(
                Update,
                (
                    faction_select_button_system,
                    faction_select_action_system,
                )
                    .run_if(in_state(AppState::FactionSelect)),
            )
            .add_systems(OnExit(AppState::FactionSelect), cleanup_faction_select_ui);
    }
}

#[derive(Resource, Default)]
struct SelectedFactionMenu {
    pub faction: FactionId,
}

#[derive(Component)]
struct FactionSelectRoot;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum SelectAction {
    Choose(FactionId),
    ChooseEnvironment(PlanetEnvironment),
    ChooseSize(MapSize),
    RerollSeed,
    Confirm,
    Back,
}

#[derive(Component)]
struct DetailTitleText;

#[derive(Component)]
struct DetailDescText;

#[derive(Component)]
struct DetailPanelRoot;

#[derive(Component)]
struct FactionCard(FactionId);

#[derive(Component)]
struct EnvButton(PlanetEnvironment);

#[derive(Component)]
struct SizeButton(MapSize);

#[derive(Component)]
struct SeedDisplayText;

#[derive(Component)]
struct EnvDescText;

// カラー
const BG_COLOR: Color = Color::srgba(0.04, 0.07, 0.12, 0.95);
const PANEL_BG: Color = Color::srgba(0.08, 0.14, 0.22, 0.85);
const BORDER_COLOR: Color = Color::srgb(0.20, 0.35, 0.50);
const TEXT_MAIN: Color = Color::srgb(0.92, 0.96, 0.98);
const TEXT_MUTED: Color = Color::srgb(0.60, 0.72, 0.82);
const ACCENT_CYAN: Color = Color::srgb(0.25, 0.85, 0.75);

fn setup_faction_select_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_faction: Res<PlayerFaction>,
    map_config: Res<MapConfig>,
    mut selected_menu: ResMut<SelectedFactionMenu>,
) {
    selected_menu.faction = player_faction.0;

    let font_regular = asset_server.load("fonts/UDEVGothicNF-Regular.ttf");
    let font_bold = asset_server.load("fonts/UDEVGothicNF-Bold.ttf");

    commands
        .spawn((
            FactionSelectRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(32.0), Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(BG_COLOR),
        ))
        .with_children(|root| {
            // 1. ヘッダー
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(4.0),
                ..default()
            })
            .with_children(|header| {
                header.spawn((
                    Text::new("EXPEDITION CONFIG // 出撃勢力 & 探査環境設定"),
                    TextFont {
                        font: font_bold.clone().into(),
                        font_size: FontSize::Px(26.0),
                        ..default()
                    },
                    TextColor(ACCENT_CYAN),
                ));

                header.spawn((
                    Text::new("未知の惑星へ進出する指揮勢力と、開拓対象となる惑星環境を選択してください"),
                    TextFont {
                        font: font_regular.clone().into(),
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(TEXT_MUTED),
                ));
            });

            // 2. メインコンテンツ（左: 派閥リスト, 右: 勢力詳細 + 惑星環境・マップ設定）
            root.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(510.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                ..default()
            })
            .with_children(|content| {
                // 左: 6派閥カード一覧
                content
                    .spawn(Node {
                        width: Val::Px(450.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(8.0),
                        justify_content: JustifyContent::Center,
                        ..default()
                    })
                    .with_children(|list| {
                        for faction in FactionId::ALL {
                            let is_active = faction == selected_menu.faction;
                            let f_col = faction.primary_color();

                            list.spawn((
                                Button,
                                SelectAction::Choose(faction),
                                FactionCard(faction),
                                Node {
                                    width: Val::Percent(100.0),
                                    padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceBetween,
                                    border: UiRect::all(Val::Px(if is_active { 2.5 } else { 1.0 })),
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    ..default()
                                },
                                BorderColor::all(if is_active { f_col } else { BORDER_COLOR }),
                                BackgroundColor(if is_active {
                                    Color::srgba(0.15, 0.22, 0.32, 0.9)
                                } else {
                                    PANEL_BG
                                }),
                            ))
                            .with_children(|card| {
                                card.spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    column_gap: Val::Px(10.0),
                                    ..default()
                                })
                                .with_children(|left| {
                                    // カラー識別マーカー
                                    left.spawn((
                                        Node {
                                            width: Val::Px(10.0),
                                            height: Val::Px(22.0),
                                            border_radius: BorderRadius::all(Val::Px(3.0)),
                                            ..default()
                                        },
                                        BackgroundColor(f_col),
                                    ));

                                    left.spawn((
                                        Text::new(format!("国{}【{}】", faction.code(), faction.name_ja())),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(15.0),
                                            ..default()
                                        },
                                        TextColor(TEXT_MAIN),
                                    ));
                                });

                                card.spawn((
                                    Text::new(faction.name_en()),
                                    TextFont {
                                        font: font_regular.clone().into(),
                                        font_size: FontSize::Px(12.0),
                                        ..default()
                                    },
                                    TextColor(TEXT_MUTED),
                                ));
                            });
                        }
                    });

                // 右: 勢力詳細 + 惑星環境・マップ設定パネル
                content
                    .spawn((
                        DetailPanelRoot,
                        Node {
                            width: Val::Px(590.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(10.0),
                            padding: UiRect::all(Val::Px(18.0)),
                            border: UiRect::all(Val::Px(1.5)),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        BorderColor::all(selected_menu.faction.primary_color()),
                        BackgroundColor(Color::srgba(0.06, 0.10, 0.16, 0.95)),
                    ))
                    .with_children(|panel| {
                        // 上部: 派閥情報
                        panel.spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.0),
                            ..default()
                        })
                        .with_children(|faction_info| {
                            // 派閥正式名称
                            faction_info.spawn((
                                DetailTitleText,
                                Text::new(format!(
                                    "{} ({})",
                                    selected_menu.faction.formal_title(),
                                    selected_menu.faction.name_ja()
                                )),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(18.0),
                                    ..default()
                                },
                                TextColor(selected_menu.faction.accent_color()),
                            ));

                            // 派閥説明
                            faction_info.spawn((
                                DetailDescText,
                                Text::new(selected_menu.faction.description()),
                                TextFont {
                                    font: font_regular.clone().into(),
                                    font_size: FontSize::Px(12.5),
                                    ..default()
                                },
                                TextColor(TEXT_MAIN),
                            ));
                        });

                        // 区切り線
                        panel.spawn((
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(1.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.25, 0.85, 0.75, 0.3)),
                        ));

                        // 下部: 惑星環境・マップ設定セクション
                        panel.spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|settings_sec| {
                            settings_sec.spawn((
                                Text::new("PLANET ENVIRONMENT // 探査環境 & マップ構成"),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(13.5),
                                    ..default()
                                },
                                TextColor(ACCENT_CYAN),
                            ));

                            // 1. 環境タイプボタン行
                            settings_sec.spawn(Node {
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::Wrap,
                                row_gap: Val::Px(6.0),
                                column_gap: Val::Px(6.0),
                                ..default()
                            })
                            .with_children(|env_row| {
                                for env in PlanetEnvironment::ALL {
                                    let is_active = env == map_config.environment;
                                    env_row.spawn((
                                        Button,
                                        SelectAction::ChooseEnvironment(env),
                                        EnvButton(env),
                                        Node {
                                            padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                                            border: UiRect::all(Val::Px(if is_active { 2.0 } else { 1.0 })),
                                            border_radius: BorderRadius::all(Val::Px(4.0)),
                                            ..default()
                                        },
                                        BorderColor::all(if is_active { env.theme_color() } else { BORDER_COLOR }),
                                        BackgroundColor(if is_active {
                                            Color::srgba(0.18, 0.26, 0.38, 0.9)
                                        } else {
                                            PANEL_BG
                                        }),
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new(env.name_ja()),
                                            TextFont {
                                                font: font_bold.clone().into(),
                                                font_size: FontSize::Px(11.5),
                                                ..default()
                                            },
                                            TextColor(if is_active { env.theme_color() } else { TEXT_MAIN }),
                                        ));
                                    });
                                }
                            });

                            // 環境説明テキスト
                            settings_sec.spawn((
                                EnvDescText,
                                Text::new(map_config.environment.description()),
                                TextFont {
                                    font: font_regular.clone().into(),
                                    font_size: FontSize::Px(12.0),
                                    ..default()
                                },
                                TextColor(TEXT_MUTED),
                            ));

                            // 2. マップサイズ選択 & シード再生成ボタン行
                            settings_sec.spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(6.0),
                                margin: UiRect::top(Val::Px(2.0)),
                                ..default()
                            })
                            .with_children(|size_sec| {
                                size_sec.spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::SpaceBetween,
                                    align_items: AlignItems::Center,
                                    ..default()
                                })
                                .with_children(|header_row| {
                                    header_row.spawn((
                                        Text::new("マップサイズ:"),
                                        TextFont {
                                            font: font_regular.clone().into(),
                                            font_size: FontSize::Px(12.0),
                                            ..default()
                                        },
                                        TextColor(TEXT_MUTED),
                                    ));

                                    // シード再抽選ボタン
                                    header_row.spawn((
                                        Button,
                                        SelectAction::RerollSeed,
                                        Node {
                                            padding: UiRect::axes(Val::Px(10.0), Val::Px(3.0)),
                                            border: UiRect::all(Val::Px(1.0)),
                                            border_radius: BorderRadius::all(Val::Px(4.0)),
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BorderColor::all(BORDER_COLOR),
                                        BackgroundColor(PANEL_BG),
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            SeedDisplayText,
                                            Text::new(format!("🎲 SEED: {}", map_config.seed)),
                                            TextFont {
                                                font: font_regular.clone().into(),
                                                font_size: FontSize::Px(11.0),
                                                ..default()
                                            },
                                            TextColor(TEXT_MAIN),
                                        ));
                                    });
                                });

                                // サイズ選択ボタン一覧 (7種、折り返し可能)
                                size_sec.spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    flex_wrap: FlexWrap::Wrap,
                                    row_gap: Val::Px(5.0),
                                    column_gap: Val::Px(5.0),
                                    align_items: AlignItems::Center,
                                    ..default()
                                })
                                .with_children(|size_group| {
                                    for sz in MapSize::ALL {
                                        let is_active = sz == map_config.size;
                                        size_group.spawn((
                                            Button,
                                            SelectAction::ChooseSize(sz),
                                            SizeButton(sz),
                                            Node {
                                                padding: UiRect::axes(Val::Px(7.0), Val::Px(4.0)),
                                                border: UiRect::all(Val::Px(if is_active { 1.5 } else { 1.0 })),
                                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                                ..default()
                                            },
                                            BorderColor::all(if is_active { ACCENT_CYAN } else { BORDER_COLOR }),
                                            BackgroundColor(if is_active {
                                                Color::srgba(0.20, 0.35, 0.45, 0.9)
                                            } else {
                                                PANEL_BG
                                            }),
                                        ))
                                        .with_children(|btn| {
                                            btn.spawn((
                                                Text::new(sz.name_ja()),
                                                TextFont {
                                                    font: font_regular.clone().into(),
                                                    font_size: FontSize::Px(10.5),
                                                    ..default()
                                                },
                                                TextColor(if is_active { Color::WHITE } else { TEXT_MAIN }),
                                            ));
                                        });
                                    }
                                });
                            });
                        });
                    });
            });

            // 3. フッター（戻る / 決定ボタン）
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                ..default()
            })
            .with_children(|footer| {
                // 戻るボタン
                footer
                    .spawn((
                        Button,
                        SelectAction::Back,
                        Node {
                            width: Val::Px(160.0),
                            height: Val::Px(44.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.5)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BorderColor::all(BORDER_COLOR),
                        BackgroundColor(PANEL_BG),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("戻る (BACK)"),
                            TextFont {
                                font: font_bold.clone().into(),
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextColor(TEXT_MUTED),
                        ));
                    });

                // 決定して開拓開始ボタン
                footer
                    .spawn((
                        Button,
                        SelectAction::Confirm,
                        Node {
                            width: Val::Px(240.0),
                            height: Val::Px(44.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        BorderColor::all(ACCENT_CYAN),
                        BackgroundColor(Color::srgb(0.12, 0.35, 0.40)),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("開拓着任 (START GAME)"),
                            TextFont {
                                font: font_bold.clone().into(),
                                font_size: FontSize::Px(16.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            });
        });
}

type FactionSelectInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static Interaction,
        &'static SelectAction,
        &'static mut BorderColor,
        &'static mut BackgroundColor,
    ),
    (Changed<Interaction>, With<Button>),
>;

type FactionSelectActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static SelectAction),
    (Changed<Interaction>, With<Button>),
>;

type DetailTitleTextQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static mut Text, &'static mut TextColor),
    (With<DetailTitleText>, Without<DetailDescText>),
>;

type DetailDescTextQuery<'world, 'state> = Query<
    'world,
    'state,
    &'static mut Text,
    (With<DetailDescText>, Without<DetailTitleText>),
>;

fn faction_select_button_system(
    mut interaction_query: FactionSelectInteractionQuery,
    selected_menu: Res<SelectedFactionMenu>,
    map_config: Res<MapConfig>,
) {
    for (interaction, action, mut border_color, mut bg_color) in &mut interaction_query {
        match action {
            SelectAction::Choose(f) => {
                let is_active = *f == selected_menu.faction;
                match *interaction {
                    Interaction::Pressed | Interaction::Hovered => {
                        *border_color = BorderColor::all(f.accent_color());
                        *bg_color = BackgroundColor(Color::srgba(0.20, 0.30, 0.42, 0.9));
                    }
                    Interaction::None => {
                        if is_active {
                            *border_color = BorderColor::all(f.primary_color());
                            *bg_color = BackgroundColor(Color::srgba(0.15, 0.22, 0.32, 0.9));
                        } else {
                            *border_color = BorderColor::all(BORDER_COLOR);
                            *bg_color = BackgroundColor(PANEL_BG);
                        }
                    }
                }
            }
            SelectAction::ChooseEnvironment(env) => {
                let is_active = *env == map_config.environment;
                match *interaction {
                    Interaction::Pressed | Interaction::Hovered => {
                        *border_color = BorderColor::all(env.theme_color());
                        *bg_color = BackgroundColor(Color::srgba(0.22, 0.32, 0.45, 0.9));
                    }
                    Interaction::None => {
                        if is_active {
                            *border_color = BorderColor::all(env.theme_color());
                            *bg_color = BackgroundColor(Color::srgba(0.18, 0.26, 0.38, 0.9));
                        } else {
                            *border_color = BorderColor::all(BORDER_COLOR);
                            *bg_color = BackgroundColor(PANEL_BG);
                        }
                    }
                }
            }
            SelectAction::ChooseSize(sz) => {
                let is_active = *sz == map_config.size;
                match *interaction {
                    Interaction::Pressed | Interaction::Hovered => {
                        *border_color = BorderColor::all(ACCENT_CYAN);
                        *bg_color = BackgroundColor(Color::srgba(0.25, 0.40, 0.50, 0.9));
                    }
                    Interaction::None => {
                        if is_active {
                            *border_color = BorderColor::all(ACCENT_CYAN);
                            *bg_color = BackgroundColor(Color::srgba(0.20, 0.35, 0.45, 0.9));
                        } else {
                            *border_color = BorderColor::all(BORDER_COLOR);
                            *bg_color = BackgroundColor(PANEL_BG);
                        }
                    }
                }
            }
            SelectAction::RerollSeed => match *interaction {
                Interaction::Pressed | Interaction::Hovered => {
                    *border_color = BorderColor::all(ACCENT_CYAN);
                    *bg_color = BackgroundColor(Color::srgba(0.18, 0.28, 0.38, 0.9));
                }
                Interaction::None => {
                    *border_color = BorderColor::all(BORDER_COLOR);
                    *bg_color = BackgroundColor(PANEL_BG);
                }
            },
            SelectAction::Confirm => match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(Color::srgb(0.25, 0.65, 0.70));
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(Color::srgb(0.18, 0.48, 0.55));
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(Color::srgb(0.12, 0.35, 0.40));
                }
            },
            SelectAction::Back => match *interaction {
                Interaction::Pressed | Interaction::Hovered => {
                    *border_color = BorderColor::all(ACCENT_CYAN);
                    *bg_color = BackgroundColor(Color::srgb(0.15, 0.24, 0.35));
                }
                Interaction::None => {
                    *border_color = BorderColor::all(BORDER_COLOR);
                    *bg_color = BackgroundColor(PANEL_BG);
                }
            },
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn faction_select_action_system(
    query: FactionSelectActionQuery,
    mut selected_menu: ResMut<SelectedFactionMenu>,
    mut player_faction: ResMut<PlayerFaction>,
    mut map_config: ResMut<MapConfig>,
    mut next_state: ResMut<NextState<AppState>>,
    mut title_query: DetailTitleTextQuery,
    mut desc_query: DetailDescTextQuery,
    mut env_desc_query: Query<&mut Text, (With<EnvDescText>, Without<DetailTitleText>, Without<DetailDescText>, Without<SeedDisplayText>)>,
    mut seed_text_query: Query<&mut Text, (With<SeedDisplayText>, Without<DetailTitleText>, Without<DetailDescText>, Without<EnvDescText>)>,
    mut cards_query: Query<(&FactionCard, &mut BorderColor, &mut BackgroundColor), (Without<EnvButton>, Without<SizeButton>)>,
    mut env_btn_query: Query<(&EnvButton, &mut BorderColor, &mut BackgroundColor), (Without<FactionCard>, Without<SizeButton>)>,
    mut size_btn_query: Query<(&SizeButton, &mut BorderColor, &mut BackgroundColor), (Without<FactionCard>, Without<EnvButton>)>,
    mut panel_query: Query<&mut BorderColor, (With<DetailPanelRoot>, Without<FactionCard>, Without<EnvButton>, Without<SizeButton>)>,
) {
    for (interaction, action) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            SelectAction::Choose(faction) => {
                selected_menu.faction = *faction;

                if let Ok((mut text, mut text_color)) = title_query.single_mut() {
                    **text = format!("{} ({})", faction.formal_title(), faction.name_ja());
                    *text_color = TextColor(faction.accent_color());
                }
                if let Ok(mut text) = desc_query.single_mut() {
                    **text = faction.description().to_string();
                }

                if let Ok(mut panel_border) = panel_query.single_mut() {
                    *panel_border = BorderColor::all(faction.primary_color());
                }

                // カード枠のスタイル更新
                for (card, mut border_color, mut bg_color) in &mut cards_query {
                    let is_active = card.0 == *faction;
                    if is_active {
                        *border_color = BorderColor::all(card.0.primary_color());
                        *bg_color = BackgroundColor(Color::srgba(0.15, 0.22, 0.32, 0.9));
                    } else {
                        *border_color = BorderColor::all(BORDER_COLOR);
                        *bg_color = BackgroundColor(PANEL_BG);
                    }
                }
            }
            SelectAction::ChooseEnvironment(env) => {
                map_config.environment = *env;

                if let Ok(mut text) = env_desc_query.single_mut() {
                    **text = env.description().to_string();
                }

                for (btn, mut border_color, mut bg_color) in &mut env_btn_query {
                    let is_active = btn.0 == *env;
                    if is_active {
                        *border_color = BorderColor::all(btn.0.theme_color());
                        *bg_color = BackgroundColor(Color::srgba(0.18, 0.26, 0.38, 0.9));
                    } else {
                        *border_color = BorderColor::all(BORDER_COLOR);
                        *bg_color = BackgroundColor(PANEL_BG);
                    }
                }
            }
            SelectAction::ChooseSize(sz) => {
                map_config.size = *sz;

                for (btn, mut border_color, mut bg_color) in &mut size_btn_query {
                    let is_active = btn.0 == *sz;
                    if is_active {
                        *border_color = BorderColor::all(ACCENT_CYAN);
                        *bg_color = BackgroundColor(Color::srgba(0.20, 0.35, 0.45, 0.9));
                    } else {
                        *border_color = BorderColor::all(BORDER_COLOR);
                        *bg_color = BackgroundColor(PANEL_BG);
                    }
                }
            }
            SelectAction::RerollSeed => {
                // シンプルなLCGでシード値を変更
                let new_seed = map_config.seed.wrapping_mul(1664525).wrapping_add(1013904223);
                let display_seed = (new_seed % 90000) + 10000;
                map_config.seed = display_seed;

                if let Ok(mut text) = seed_text_query.single_mut() {
                    **text = format!("🎲 SEED: {}", display_seed);
                }
            }
            SelectAction::Confirm => {
                player_faction.0 = selected_menu.faction;
                next_state.set(AppState::InGame);
            }
            SelectAction::Back => {
                next_state.set(AppState::Title);
            }
        }
    }
}

fn cleanup_faction_select_ui(
    mut commands: Commands,
    query: Query<Entity, With<FactionSelectRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
