use bevy::prelude::*;

use crate::faction::types::{FactionId, FactionManager, PlayerFaction};
use crate::state::AppState;

pub struct DiplomacyUiPlugin;

impl Plugin for DiplomacyUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiplomacyModalState>()
            .add_systems(
                Update,
                (
                    toggle_diplomacy_modal_system,
                    diplomacy_button_system,
                    diplomacy_action_system,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), close_diplomacy_modal);
    }
}

#[derive(Resource, Default)]
pub struct DiplomacyModalState {
    pub is_open: bool,
    pub selected_target: Option<FactionId>,
}

#[derive(Component)]
struct DiplomacyModalRoot;

#[derive(Component)]
enum DiplomacyButtonAction {
    SelectFaction(FactionId),
    SendGift,
    ProposeNonAggression,
    Close,
}

#[derive(Component)]
struct TargetFactionTitle;

#[derive(Component)]
struct TargetFactionDesc;

#[derive(Component)]
struct TargetRelationText;

#[derive(Component)]
struct DiplomacyNoticeText;

use super::theme::{
    self, ACCENT_COLOR as ACCENT_CYAN, BORDER_COLOR, CARD_BG, OVERLAY_BG as MODAL_BG, TEXT_MAIN,
    TEXT_MUTED,
};

fn toggle_diplomacy_modal_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut modal_state: ResMut<DiplomacyModalState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_faction: Res<PlayerFaction>,
    faction_mgr: Res<FactionManager>,
    existing: Query<Entity, With<DiplomacyModalRoot>>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        modal_state.is_open = !modal_state.is_open;

        if modal_state.is_open {
            if modal_state.selected_target.is_none() {
                // デフォルトは自派閥以外の最初の派閥を選択
                modal_state.selected_target = FactionId::ALL
                    .iter()
                    .copied()
                    .find(|&f| f != player_faction.0);
            }
            spawn_diplomacy_modal(
                &mut commands,
                &asset_server,
                player_faction.0,
                &modal_state,
                &faction_mgr,
            );
        } else {
            for entity in &existing {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn close_diplomacy_modal(
    mut commands: Commands,
    mut modal_state: ResMut<DiplomacyModalState>,
    existing: Query<Entity, With<DiplomacyModalRoot>>,
) {
    modal_state.is_open = false;
    for entity in &existing {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_diplomacy_modal(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_faction: FactionId,
    modal_state: &DiplomacyModalState,
    faction_mgr: &FactionManager,
) {
    let font_regular = asset_server.load(theme::FONT_REGULAR);
    let font_bold = asset_server.load(theme::FONT_BOLD);

    let target_f = modal_state
        .selected_target
        .unwrap_or(FactionId::GrandDuchy);

    let rel = faction_mgr.relation_between(player_faction, target_f);
    let rel_score = faction_mgr.get_relation_score(player_faction, target_f);

    commands
        .spawn((
            DiplomacyModalRoot,
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
                        width: Val::Px(920.0),
                        height: Val::Px(560.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(24.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(10.0)),
                        row_gap: Val::Px(16.0),
                        ..default()
                    },
                    BorderColor::all(ACCENT_CYAN),
                    BackgroundColor(MODAL_BG),
                ))
                .with_children(|window| {
                    // 1. ダイアログヘッダー
                    window
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            padding: UiRect::bottom(Val::Px(12.0)),
                            border: UiRect::bottom(Val::Px(1.5)),
                            ..default()
                        })
                        .insert(BorderColor::all(BORDER_COLOR))
                        .with_children(|header| {
                            header.spawn((
                                Text::new("PLANETARY DIPLOMACY // 惑星外交・各派閥関係"),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(22.0),
                                    ..default()
                                },
                                TextColor(ACCENT_CYAN),
                            ));

                            header
                                .spawn((
                                    Button,
                                    DiplomacyButtonAction::Close,
                                    Node {
                                        padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                                        border: UiRect::all(Val::Px(1.0)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        ..default()
                                    },
                                    BorderColor::all(BORDER_COLOR),
                                    BackgroundColor(CARD_BG),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("✕ 閉じる (Esc/F)"),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(13.0),
                                            ..default()
                                        },
                                        TextColor(TEXT_MAIN),
                                    ));
                                });
                        });

                    // 2. ダイアログボディ（左: 派閥リスト, 右: 選択先との詳細・外交コマンド）
                    window
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            flex_grow: 1.0,
                            ..default()
                        })
                        .with_children(|body| {
                            // 左側: 全派閥リスト
                            body.spawn(Node {
                                width: Val::Px(360.0),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
                                ..default()
                            })
                            .with_children(|list| {
                                for faction in FactionId::ALL {
                                    let is_player = faction == player_faction;
                                    let is_selected = faction == target_f;
                                    let relation =
                                        faction_mgr.relation_between(player_faction, faction);
                                    let color = faction.primary_color();

                                    list.spawn((
                                        Button,
                                        DiplomacyButtonAction::SelectFaction(faction),
                                        Node {
                                            padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                                            border: UiRect::all(Val::Px(if is_selected {
                                                2.0
                                            } else {
                                                1.0
                                            })),
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::SpaceBetween,
                                            ..default()
                                        },
                                        BorderColor::all(if is_selected {
                                            color
                                        } else {
                                            BORDER_COLOR
                                        }),
                                        BackgroundColor(if is_selected {
                                            Color::srgba(0.14, 0.22, 0.34, 0.9)
                                        } else {
                                            CARD_BG
                                        }),
                                    ))
                                    .with_children(|item| {
                                        item.spawn(Node {
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            column_gap: Val::Px(8.0),
                                            ..default()
                                        })
                                        .with_children(|info| {
                                            info.spawn((
                                                Node {
                                                    width: Val::Px(8.0),
                                                    height: Val::Px(16.0),
                                                    border_radius: BorderRadius::all(Val::Px(2.0)),
                                                    ..default()
                                                },
                                                BackgroundColor(color),
                                            ));

                                            let name_label = if is_player {
                                                format!("国{}【{}】(自国)", faction.code(), faction.name_ja())
                                            } else {
                                                format!("国{}【{}】", faction.code(), faction.name_ja())
                                            };

                                            info.spawn((
                                                Text::new(name_label),
                                                TextFont {
                                                    font: font_bold.clone().into(),
                                                    font_size: FontSize::Px(14.0),
                                                    ..default()
                                                },
                                                TextColor(TEXT_MAIN),
                                            ));
                                        });

                                        if !is_player {
                                            item.spawn((
                                                Text::new(format!(
                                                    "{} {}",
                                                    relation.symbol(),
                                                    relation.text_ja()
                                                )),
                                                TextFont {
                                                    font: font_regular.clone().into(),
                                                    font_size: FontSize::Px(12.0),
                                                    ..default()
                                                },
                                                TextColor(relation.color()),
                                            ));
                                        }
                                    });
                                }
                            });

                            // 右側: 選択派閥との詳細・外交
                            body.spawn((
                                Node {
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(14.0),
                                    padding: UiRect::all(Val::Px(16.0)),
                                    border: UiRect::all(Val::Px(1.5)),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BorderColor::all(target_f.primary_color()),
                                BackgroundColor(Color::srgba(0.06, 0.10, 0.16, 0.90)),
                            ))
                            .with_children(|right| {
                                right.spawn((
                                    TargetFactionTitle,
                                    Text::new(format!(
                                        "{} // {}",
                                        target_f.formal_title(),
                                        target_f.name_en()
                                    )),
                                    TextFont {
                                        font: font_bold.clone().into(),
                                        font_size: FontSize::Px(18.0),
                                        ..default()
                                    },
                                    TextColor(target_f.accent_color()),
                                ));

                                right.spawn((
                                    TargetFactionDesc,
                                    Text::new(target_f.description()),
                                    TextFont {
                                        font: font_regular.clone().into(),
                                        font_size: FontSize::Px(14.0),
                                        ..default()
                                    },
                                    TextColor(TEXT_MAIN),
                                ));

                                right.spawn((
                                    TargetRelationText,
                                    Text::new(format!(
                                        "【対自勢力関係】: {} (好感度指数: {})\n【領有タイル数】: 約 {} 区画",
                                        rel.text_ja(),
                                        rel_score,
                                        faction_mgr.territory_count(target_f)
                                    )),
                                    TextFont {
                                        font: font_bold.clone().into(),
                                        font_size: FontSize::Px(14.0),
                                        ..default()
                                    },
                                    TextColor(rel.color()),
                                ));

                                // 外交通知メッセージ
                                right.spawn((
                                    DiplomacyNoticeText,
                                    Text::new("親善使節の派遣や協定締結が可能です"),
                                    TextFont {
                                        font: font_regular.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(TEXT_MUTED),
                                ));

                                // 外交アクションボタン
                                right.spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(12.0),
                                    margin: UiRect::top(Val::Px(8.0)),
                                    ..default()
                                })
                                .with_children(|actions| {
                                    actions.spawn((
                                        Button,
                                        DiplomacyButtonAction::SendGift,
                                        Node {
                                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                                            border: UiRect::all(Val::Px(1.0)),
                                            border_radius: BorderRadius::all(Val::Px(4.0)),
                                            ..default()
                                        },
                                        BorderColor::all(ACCENT_CYAN),
                                        BackgroundColor(Color::srgb(0.12, 0.25, 0.32)),
                                    ))
                                    .with_children(|b| {
                                        b.spawn((
                                            Text::new("親善使節・物資供与 (+15)"),
                                            TextFont {
                                                font: font_bold.clone().into(),
                                                font_size: FontSize::Px(13.0),
                                                ..default()
                                            },
                                            TextColor(ACCENT_CYAN),
                                        ));
                                    });

                                    actions.spawn((
                                        Button,
                                        DiplomacyButtonAction::ProposeNonAggression,
                                        Node {
                                            padding: UiRect::axes(Val::Px(14.0), Val::Px(8.0)),
                                            border: UiRect::all(Val::Px(1.0)),
                                            border_radius: BorderRadius::all(Val::Px(4.0)),
                                            ..default()
                                        },
                                        BorderColor::all(BORDER_COLOR),
                                        BackgroundColor(CARD_BG),
                                    ))
                                    .with_children(|b| {
                                        b.spawn((
                                            Text::new("不可侵協定打診"),
                                            TextFont {
                                                font: font_bold.clone().into(),
                                                font_size: FontSize::Px(13.0),
                                                ..default()
                                            },
                                            TextColor(TEXT_MAIN),
                                        ));
                                    });
                                });
                            });
                        });
                });
        });
}

type DiplomacyButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static Interaction,
        &'static DiplomacyButtonAction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
    ),
    (Changed<Interaction>, With<Button>),
>;

type DiplomacyActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static DiplomacyButtonAction),
    (Changed<Interaction>, With<Button>),
>;

fn diplomacy_button_system(mut query: DiplomacyButtonInteractionQuery) {
    for (interaction, action, mut bg_color, mut border_color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(Color::srgb(0.20, 0.38, 0.50));
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(Color::srgb(0.16, 0.28, 0.38));
                *border_color = BorderColor::all(ACCENT_CYAN);
            }
            Interaction::None => {
                match action {
                    DiplomacyButtonAction::SendGift => {
                        *bg_color = BackgroundColor(Color::srgb(0.12, 0.25, 0.32));
                        *border_color = BorderColor::all(ACCENT_CYAN);
                    }
                    _ => {
                        *bg_color = BackgroundColor(CARD_BG);
                        *border_color = BorderColor::all(BORDER_COLOR);
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn diplomacy_action_system(
    query: DiplomacyActionQuery,
    mut commands: Commands,
    mut modal_state: ResMut<DiplomacyModalState>,
    player_faction: Res<PlayerFaction>,
    mut faction_mgr: ResMut<FactionManager>,
    existing: Query<Entity, With<DiplomacyModalRoot>>,
    asset_server: Res<AssetServer>,
    mut notice_query: Query<&mut Text, (With<DiplomacyNoticeText>, Without<TargetRelationText>)>,
    mut rel_query: Query<&mut Text, (With<TargetRelationText>, Without<DiplomacyNoticeText>)>,
) {
    for (interaction, action) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            DiplomacyButtonAction::Close => {
                modal_state.is_open = false;
                for entity in &existing {
                    commands.entity(entity).despawn();
                }
            }
            DiplomacyButtonAction::SelectFaction(faction) => {
                modal_state.selected_target = Some(*faction);
                // 再描画
                for entity in &existing {
                    commands.entity(entity).despawn();
                }
                spawn_diplomacy_modal(
                    &mut commands,
                    &asset_server,
                    player_faction.0,
                    &modal_state,
                    &faction_mgr,
                );
            }
            DiplomacyButtonAction::SendGift => {
                if let Some(target) = modal_state.selected_target {
                    if target == player_faction.0 {
                        if let Ok(mut text) = notice_query.single_mut() {
                            **text = "自勢力への供与はできません。".to_string();
                        }
                        continue;
                    }

                    faction_mgr.modify_relation(player_faction.0, target, 15);
                    let new_rel = faction_mgr.relation_between(player_faction.0, target);
                    let score = faction_mgr.get_relation_score(player_faction.0, target);

                    if let Ok(mut text) = notice_query.single_mut() {
                        **text = format!("{}へ親善使節を派遣しました。好感度 +15", target.name_ja());
                    }
                    if let Ok(mut text) = rel_query.single_mut() {
                        **text = format!(
                            "【対自勢力関係】: {} (好感度指数: {})\n【領有タイル数】: 約 {} 区画",
                            new_rel.text_ja(),
                            score,
                            faction_mgr.territory_count(target)
                        );
                    }
                }
            }
            DiplomacyButtonAction::ProposeNonAggression => {
                if let Some(target) = modal_state.selected_target {
                    if target == player_faction.0 {
                        continue;
                    }
                    let score = faction_mgr.get_relation_score(player_faction.0, target);
                    if let Ok(mut text) = notice_query.single_mut() {
                        if score >= 20 {
                            **text = format!("{}との間で不可侵条約が仮締結されました！", target.name_ja());
                        } else {
                            **text = format!("{}は警戒を崩さず、条約提案を保留しました。", target.name_ja());
                        }
                    }
                }
            }
        }
    }
}
