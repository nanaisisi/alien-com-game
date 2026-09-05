use bevy::prelude::*;

use super::types::*;
use crate::ui::theme::UiTheme;

pub fn setup_title_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_regular = asset_server.load(UiTheme::fonts().regular());
    let font_bold = asset_server.load(UiTheme::fonts().bold());
    let surfaces = UiTheme::surfaces();
    let text = UiTheme::text();
    let standard_btn = UiTheme::button(false);

    // タイトルルートコンテナ（全画面フルスクリーン）
    commands
        .spawn((
            TitleRootUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.06, 0.10, 0.88)),
        ))
        .with_children(|parent| {
            // タイトルヘッダー領域
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|header| {
                    // メインタイトル
                    header.spawn((
                        Text::new("ALIEN COM GAME"),
                        TextFont {
                            font: font_bold.clone().into(),
                            font_size: FontSize::Px(52.0),
                            ..default()
                        },
                        TextColor(surfaces.accent()),
                    ));

                    // サブタイトル
                    header.spawn((
                        Text::new("— Planetary Colonization & Tactical Command —"),
                        TextFont {
                            font: font_regular.clone().into(),
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.70, 0.78, 0.85)),
                    ));
                });

            // メニューボタン群
            let menu_items = [
                ("ゲーム開始 (NEW GAME)", MenuButtonAction::NewGame),
                ("ロード (LOAD GAME)", MenuButtonAction::LoadGame),
                ("設定 (SETTINGS)", MenuButtonAction::Settings),
                ("終了 (EXIT)", MenuButtonAction::Exit),
            ];

            for (idx, (label, action)) in menu_items.iter().enumerate() {
                parent
                    .spawn((
                        Button,
                        *action,
                        TitleMenuButton(idx),
                        Node {
                            width: Val::Px(280.0),
                            height: Val::Px(52.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        BorderColor::all(if idx == 0 {
                            surfaces.accent()
                        } else {
                            Color::srgb(0.25, 0.38, 0.50)
                        }),
                        BackgroundColor(if idx == 0 {
                            standard_btn.hovered()
                        } else {
                            standard_btn.normal()
                        }),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(*label),
                            TextFont {
                                font: font_regular.clone().into(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                            TextColor(text.main()),
                        ));
                    });
            }

            // 操作ガイド & フッターバージョン情報
            parent.spawn((
                Text::new("[↑/↓] 選択  [Enter/Space] 決定  |  v0.1.0-alpha"),
                TextFont {
                    font: font_regular.clone().into(),
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.50, 0.62, 0.72)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

pub fn cleanup_title_ui(mut commands: Commands, query: Query<Entity, With<TitleRootUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
