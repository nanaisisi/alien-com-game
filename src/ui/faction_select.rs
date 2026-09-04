use bevy::prelude::*;

use crate::faction::types::{FactionId, PlayerFaction};
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

#[derive(Component)]
enum SelectAction {
    Choose(FactionId),
    Confirm,
    Back,
}

#[derive(Component)]
struct DetailTitleText;

#[derive(Component)]
struct DetailDescText;

#[derive(Component)]
struct FactionCard(FactionId);

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
                padding: UiRect::axes(Val::Px(40.0), Val::Px(30.0)),
                ..default()
            },
            BackgroundColor(BG_COLOR),
        ))
        .with_children(|root| {
            // 1. ヘッダー
            root.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(6.0),
                ..default()
            })
            .with_children(|header| {
                header.spawn((
                    Text::new("FACTION SELECTION // 入植勢力選択"),
                    TextFont {
                        font: font_bold.clone().into(),
                        font_size: FontSize::Px(32.0),
                        ..default()
                    },
                    TextColor(ACCENT_CYAN),
                ));

                header.spawn((
                    Text::new("未知の惑星へ進出する母星の6大国家から指揮する勢力を選択してください"),
                    TextFont {
                        font: font_regular.clone().into(),
                        font_size: FontSize::Px(15.0),
                        ..default()
                    },
                    TextColor(TEXT_MUTED),
                ));
            });

            // 2. メインコンテンツ（左: 派閥グリッド, 右: 選択中の詳細）
            root.spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Px(420.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(28.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Stretch,
                ..default()
            })
            .with_children(|content| {
                // 左: 6派閥カード一覧
                content
                    .spawn(Node {
                        width: Val::Px(480.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(10.0),
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
                                    padding: UiRect::axes(Val::Px(16.0), Val::Px(12.0)),
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
                                    column_gap: Val::Px(12.0),
                                    ..default()
                                })
                                .with_children(|left| {
                                    // カラー識別マーカー
                                    left.spawn((
                                        Node {
                                            width: Val::Px(12.0),
                                            height: Val::Px(24.0),
                                            border_radius: BorderRadius::all(Val::Px(3.0)),
                                            ..default()
                                        },
                                        BackgroundColor(f_col),
                                    ));

                                    left.spawn((
                                        Text::new(format!("国{}【{}】", faction.code(), faction.name_ja())),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(16.0),
                                            ..default()
                                        },
                                        TextColor(TEXT_MAIN),
                                    ));
                                });

                                card.spawn((
                                    Text::new(faction.name_en()),
                                    TextFont {
                                        font: font_regular.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(TEXT_MUTED),
                                ));
                            });
                        }
                    });

                // 右: 選択中派閥の詳細パネル
                content
                    .spawn((
                        Node {
                            width: Val::Px(520.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(16.0),
                            padding: UiRect::all(Val::Px(24.0)),
                            border: UiRect::all(Val::Px(1.5)),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        BorderColor::all(selected_menu.faction.primary_color()),
                        BackgroundColor(Color::srgba(0.06, 0.10, 0.16, 0.95)),
                    ))
                    .with_children(|panel| {
                        // 派閥正式名称
                        panel.spawn((
                            DetailTitleText,
                            Text::new(format!(
                                "{} ({})",
                                selected_menu.faction.formal_title(),
                                selected_menu.faction.name_ja()
                            )),
                            TextFont {
                                font: font_bold.clone().into(),
                                font_size: FontSize::Px(20.0),
                                ..default()
                            },
                            TextColor(selected_menu.faction.accent_color()),
                        ));

                        // 派閥説明
                        panel.spawn((
                            DetailDescText,
                            Text::new(selected_menu.faction.description()),
                            TextFont {
                                font: font_regular.clone().into(),
                                font_size: FontSize::Px(15.0),
                                ..default()
                            },
                            TextColor(TEXT_MAIN),
                        ));

                        panel.spawn((
                            Text::new(
                                "【勢力特性】\n\
                                 ・入植初期ボーナス: 指揮戦闘団 + 基礎資材\n\
                                 ・母星文化圏に基づく独自の外交関係と戦略アフィニティ",
                            ),
                            TextFont {
                                font: font_regular.clone().into(),
                                font_size: FontSize::Px(13.0),
                                ..default()
                            },
                            TextColor(TEXT_MUTED),
                        ));
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
                            height: Val::Px(46.0),
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
                            height: Val::Px(46.0),
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

fn faction_select_button_system(
    mut interaction_query: FactionSelectInteractionQuery,
    selected_menu: Res<SelectedFactionMenu>,
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

fn faction_select_action_system(
    query: FactionSelectActionQuery,
    mut selected_menu: ResMut<SelectedFactionMenu>,
    mut player_faction: ResMut<PlayerFaction>,
    mut next_state: ResMut<NextState<AppState>>,
    mut title_query: Query<&mut Text, (With<DetailTitleText>, Without<DetailDescText>)>,
    mut desc_query: Query<&mut Text, (With<DetailDescText>, Without<DetailTitleText>)>,
    mut cards_query: Query<(&FactionCard, &mut BorderColor, &mut BackgroundColor)>,
) {
    for (interaction, action) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            SelectAction::Choose(faction) => {
                selected_menu.faction = *faction;

                if let Ok(mut text) = title_query.single_mut() {
                    **text = format!("{} ({})", faction.formal_title(), faction.name_ja());
                }
                if let Ok(mut text) = desc_query.single_mut() {
                    **text = faction.description().to_string();
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
