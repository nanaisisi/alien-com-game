use bevy::prelude::*;

use crate::faction::{FactionResources, PlayerFaction};
use crate::ui::theme::UiTheme;

use super::types::{HudAction, HudLabel, HudRoot, END_TURN_NORMAL};

pub fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resources: Res<FactionResources>,
    player_faction: Res<PlayerFaction>,
    existing_hud: Query<Entity, With<HudRoot>>,
) {
    if !existing_hud.is_empty() {
        return;
    }

    let font_regular = asset_server.load(UiTheme::fonts().regular());
    let font_bold = asset_server.load(UiTheme::fonts().bold());
    let surfaces = UiTheme::surfaces();
    let text = UiTheme::text();

    commands
        .spawn((
            HudRoot,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|root| {
            // ==========================================
            // 1. トップバー（リソース & ターン & メニュー）
            // ==========================================
            root.spawn((
                crate::ui::UiBlockMapInteraction,
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    width: Val::Percent(100.0),
                    height: Val::Px(52.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(6.0)),
                    border: UiRect::bottom(Val::Px(1.5)),
                    ..default()
                },
                BackgroundColor(surfaces.panel()),
                BorderColor::all(surfaces.border()),
            ))
            .with_children(|top_bar| {
                // 左側: ターン表示 & 勢力名
                top_bar
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(16.0),
                        ..default()
                    })
                    .with_children(|left| {
                        let f = player_faction.0;
                        let f_badge_text = format!("国{}【{}】// {}", f.code(), f.name_ja(), f.name_en());
                        // 勢力名バッジ
                        left.spawn((
                            Node {
                                padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                                border: UiRect::all(Val::Px(1.5)),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.12, 0.18, 0.26, 0.85)),
                            BorderColor::all(f.primary_color()),
                        ))
                        .with_children(|badge| {
                            badge.spawn((
                                Text::new(f_badge_text),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(13.0),
                                    ..default()
                                },
                                TextColor(f.accent_color()),
                            ));
                        });

                        // ターン数
                        left.spawn((
                            HudLabel::Turn,
                            Text::new(format!("TURN {:02}", resources.turn)),
                            TextFont {
                                font: font_bold.clone().into(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                            TextColor(text.main()),
                        ));
                    });

                // 中央: リソースリスト
                top_bar
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(28.0),
                        ..default()
                    })
                    .with_children(|center| {
                        // エネルギー
                        spawn_resource_item(
                            center,
                            HudLabel::Energy,
                            "\u{f0e7} エネルギー",
                            &format!(
                                "{} (+{})",
                                resources.energy, resources.energy_per_turn
                            ),
                            Color::srgb(0.95, 0.75, 0.20),
                            &font_regular,
                            &font_bold,
                        );

                        // 生産力
                        spawn_resource_item(
                            center,
                            HudLabel::Production,
                            "\u{f0ad} 生産力",
                            &format!(
                                "{} (+{})",
                                resources.production, resources.production_per_turn
                            ),
                            Color::srgb(0.95, 0.45, 0.25),
                            &font_regular,
                            &font_bold,
                        );

                        // 科学力
                        spawn_resource_item(
                            center,
                            HudLabel::Science,
                            "\u{f0c3} 科学力",
                            &format!(
                                "{} (+{})",
                                resources.science, resources.science_per_turn
                            ),
                            Color::srgb(0.35, 0.75, 0.95),
                            &font_regular,
                            &font_bold,
                        );

                        // 食料
                        spawn_resource_item(
                            center,
                            HudLabel::Food,
                            "\u{f023d} 食料",
                            &format!(
                                "{} (+{})",
                                resources.food, resources.food_per_turn
                            ),
                            Color::srgb(0.45, 0.85, 0.40),
                            &font_regular,
                            &font_bold,
                        );
                    });

                // 右側: 外交・メニューボタン
                top_bar
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(12.0),
                        ..default()
                    })
                    .with_children(|right| {
                        // 都市管理ボタン
                        right
                            .spawn((
                                Button,
                                HudAction::OpenCity,
                                Node {
                                    padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                                    border: UiRect::all(Val::Px(1.5)),
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BorderColor::all(UiTheme::SURFACES.accent),
                                BackgroundColor(Color::srgba(0.12, 0.28, 0.38, 0.85)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("⬢ 基地司令部 (C)"),
                                    TextFont {
                                        font: font_bold.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(UiTheme::SURFACES.accent),
                                ));
                            });

                        // 外交ボタン
                        right
                            .spawn((
                                Button,
                                HudAction::OpenDiplomacy,
                                Node {
                                    padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                                    border: UiRect::all(Val::Px(1.5)),
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BorderColor::all(UiTheme::SURFACES.accent),
                                BackgroundColor(Color::srgba(0.10, 0.25, 0.32, 0.85)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("惑星外交 (F)"),
                                    TextFont {
                                        font: font_bold.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(UiTheme::SURFACES.accent),
                            ));
                        });

                        // 設定・メニューボタン
                        right
                            .spawn((
                                Button,
                                HudAction::OpenMenu,
                                Node {
                                    padding: UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                                    border: UiRect::all(Val::Px(1.5)),
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                BorderColor::all(surfaces.border()),
                                BackgroundColor(UiTheme::button(false).normal()),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("\u{f013} メニュー (ESC)"),
                                    TextFont {
                                        font: font_bold.clone().into(),
                                        font_size: FontSize::Px(13.0),
                                        ..default()
                                    },
                                    TextColor(text.main()),
                                ));
                            });

                        // 右端からある程度幅を空けた、広めの透明デバッグトリガーエリア
                        right.spawn((
                            Button,
                            crate::ui::debug_console::types::DebugSecretTriggerArea,
                            Node {
                                width: Val::Px(60.0),
                                height: Val::Px(36.0),
                                margin: UiRect::left(Val::Px(12.0)),
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                        ));
                    });
            });

            // ==========================================
            // 2. 右下: ターン終了ボタン
            // ==========================================
            root.spawn((
                crate::ui::UiBlockMapInteraction,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(24.0),
                    right: Val::Px(24.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexEnd,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
            ))
            .with_children(|action_panel| {
                // ターン終了ボタン
                action_panel
                    .spawn((
                        Button,
                        HudAction::EndTurn,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(56.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(2.0),
                            ..default()
                        },
                        BorderColor::all(surfaces.accent()),
                        BackgroundColor(END_TURN_NORMAL),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("NEXT TURN"),
                            TextFont {
                                font: font_bold.clone().into(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                            TextColor(surfaces.accent()),
                        ));

                        btn.spawn((
                            Text::new("ターン終了 [Space]"),
                            TextFont {
                                font: font_regular.clone().into(),
                                font_size: FontSize::Px(11.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.70, 0.85, 0.90)),
                        ));
                    });
            });
        });
}

fn spawn_resource_item(
    parent: &mut ChildSpawnerCommands,
    label_type: HudLabel,
    name: &str,
    initial_val: &str,
    color: Color,
    font_regular: &Handle<Font>,
    font_bold: &Handle<Font>,
) {
    let text = UiTheme::text();

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(6.0),
            padding: UiRect::axes(Val::Px(8.0), Val::Px(3.0)),
            border: UiRect::all(Val::Px(1.0)),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        })
        .insert(BackgroundColor(Color::srgba(0.08, 0.12, 0.18, 0.6)))
        .insert(BorderColor::all(Color::srgba(0.2, 0.3, 0.4, 0.5)))
        .with_children(|item| {
            // アイコン・リソース名
            item.spawn((
                Text::new(name),
                TextFont {
                    font: font_regular.clone().into(),
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(color),
            ));

            // 数値
            item.spawn((
                label_type,
                Text::new(initial_val),
                TextFont {
                    font: font_bold.clone().into(),
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(text.main()),
            ));
        });
}

pub fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
