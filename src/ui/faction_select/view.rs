use bevy::prelude::*;

use super::types::*;
use crate::faction::types::{FactionId, PlayerFaction};
use crate::map::settings::{MapConfig, MapSize, PlanetEnvironment};
use crate::ui::theme::UiTheme;

pub fn setup_faction_select_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_faction: Res<PlayerFaction>,
    mut map_config: ResMut<MapConfig>,
    mut selected_menu: ResMut<SelectedFactionMenu>,
) {
    map_config.reroll_seed();
    selected_menu.faction = player_faction.0;

    let font_regular = asset_server.load(UiTheme::fonts().regular());
    let font_bold = asset_server.load(UiTheme::fonts().bold());
    let surfaces = UiTheme::surfaces();
    let text = UiTheme::text();

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
            BackgroundColor(surfaces.overlay()),
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
                    TextColor(surfaces.accent()),
                ));

                header.spawn((
                    Text::new("未知の惑星へ進出する指揮勢力と、開拓対象となる惑星環境を選択してください"),
                    TextFont {
                        font: font_regular.clone().into(),
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(text.muted()),
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
                                BorderColor::all(if is_active { f_col } else { surfaces.border() }),
                                BackgroundColor(if is_active {
                                    Color::srgba(0.15, 0.22, 0.32, 0.9)
                                } else {
                                    surfaces.card()
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
                                        TextColor(text.main()),
                                    ));
                                });

                                card.spawn((
                                    Text::new(faction.name_en()),
                                    TextFont {
                                        font: font_regular.clone().into(),
                                        font_size: FontSize::Px(12.0),
                                        ..default()
                                    },
                                    TextColor(text.muted()),
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
                                TextColor(text.main()),
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
                                TextColor(surfaces.accent()),
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
                                        BorderColor::all(if is_active { env.theme_color() } else { surfaces.border() }),
                                        BackgroundColor(if is_active {
                                            Color::srgba(0.18, 0.26, 0.38, 0.9)
                                        } else {
                                            surfaces.card()
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
                                            TextColor(if is_active { env.theme_color() } else { text.main() }),
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
                                TextColor(text.muted()),
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
                                        TextColor(text.muted()),
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
                                        BorderColor::all(surfaces.border()),
                                        BackgroundColor(surfaces.card()),
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
                                            TextColor(text.main()),
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
                                            BorderColor::all(if is_active { surfaces.accent() } else { surfaces.border() }),
                                            BackgroundColor(if is_active {
                                                Color::srgba(0.20, 0.35, 0.45, 0.9)
                                            } else {
                                                surfaces.card()
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
                                                TextColor(if is_active { Color::WHITE } else { text.main() }),
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
                        BorderColor::all(surfaces.border()),
                        BackgroundColor(surfaces.card()),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("戻る (BACK)"),
                            TextFont {
                                font: font_bold.clone().into(),
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextColor(text.muted()),
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
                        BorderColor::all(surfaces.accent()),
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

pub fn cleanup_faction_select_ui(
    mut commands: Commands,
    query: Query<Entity, With<FactionSelectRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
