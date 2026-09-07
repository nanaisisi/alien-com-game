use bevy::prelude::*;

use super::types::*;
use crate::ui::theme::UiTheme;
use crate::ui::UiBlockMapInteraction;

/// 初回注意ダイアログを表示
pub fn spawn_warning_modal(commands: &mut Commands, asset_server: &AssetServer) {
    let font_regular = asset_server.load(UiTheme::fonts().regular());
    let font_bold = asset_server.load(UiTheme::fonts().bold());
    let surfaces = UiTheme::surfaces();
    let text_col = UiTheme::text();
    let standard_btn = UiTheme::button(false);
    let danger_btn = UiTheme::button(true);

    commands
        .spawn((
            DebugWarningModal,
            UiBlockMapInteraction,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(surfaces.overlay()),
            GlobalZIndex(100),
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    Node {
                        width: Val::Px(560.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(24.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        row_gap: Val::Px(18.0),
                        ..default()
                    },
                    BackgroundColor(surfaces.panel()),
                    BorderColor::all(surfaces.border()),
                ))
                .with_children(|panel| {
                    // タイトル
                    panel.spawn((
                        Text::new("⚠️ デバッグコマンドモードの確認"),
                        TextFont {
                            font: font_bold.clone().into(),
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.75, 0.2)),
                    ));

                    // 説明文
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(6.0),
                            ..default()
                        })
                        .with_children(|desc_box| {
                            desc_box.spawn((
                                Text::new("デバッグコマンドモードに入ります。"),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(15.0),
                                    ..default()
                                },
                                TextColor(text_col.main()),
                            ));

                            desc_box.spawn((
                                Text::new("コマンドを実行すると、ゲームバランスの崩壊や予期せぬ動作が発生する可能性があります。"),
                                TextFont {
                                    font: font_regular.clone().into(),
                                    font_size: FontSize::Px(13.0),
                                    ..default()
                                },
                                TextColor(text_col.muted()),
                            ));

                            desc_box.spawn((
                                Text::new("有効化してコンソールを開きますか？"),
                                TextFont {
                                    font: font_regular.clone().into(),
                                    font_size: FontSize::Px(14.0),
                                    ..default()
                                },
                                TextColor(text_col.main()),
                            ));
                        });

                    // 操作ボタン配置
                    panel
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(20.0),
                                margin: UiRect::top(Val::Px(8.0)),
                                ..default()
                            },
                        ))
                        .with_children(|btn_row| {
                            // 有効化ボタン
                            btn_row
                                .spawn((
                                    Button,
                                    DebugWarningAction::Enable,
                                    Node {
                                        padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                                        border: UiRect::all(Val::Px(1.5)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(standard_btn.normal()),
                                    BorderColor::all(surfaces.accent()),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("有効化して開く (Enter)"),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(14.0),
                                            ..default()
                                        },
                                        TextColor(text_col.main()),
                                    ));
                                });

                            // キャンセルボタン
                            btn_row
                                .spawn((
                                    Button,
                                    DebugWarningAction::Cancel,
                                    Node {
                                        padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                                        border: UiRect::all(Val::Px(1.5)),
                                        border_radius: BorderRadius::all(Val::Px(4.0)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    BackgroundColor(danger_btn.normal()),
                                    BorderColor::all(danger_btn.border()),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("キャンセル (Esc)"),
                                        TextFont {
                                            font: font_regular.clone().into(),
                                            font_size: FontSize::Px(14.0),
                                            ..default()
                                        },
                                        TextColor(text_col.main()),
                                    ));
                                });
                        });
                });
        });
}

/// コンソール本体UIをスポーン（上部オーバーレイ、半透明）
pub fn spawn_console_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    state: &DebugConsoleState,
) {
    let font_regular = asset_server.load(UiTheme::fonts().regular());
    let font_bold = asset_server.load(UiTheme::fonts().bold());
    let surfaces = UiTheme::surfaces();
    let text_col = UiTheme::text();
    let standard_btn = UiTheme::button(false);

    commands
        .spawn((
            DebugConsoleRoot,
            UiBlockMapInteraction,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                right: Val::Px(20.0),
                top: Val::Px(20.0),
                height: Val::Px(340.0),
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(2.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                padding: UiRect::all(Val::Px(12.0)),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.07, 0.11, 0.94)),
            BorderColor::all(surfaces.accent()),
            GlobalZIndex(90),
        ))
        .with_children(|console| {
            // ヘッダーバー (タイトル + 閉じる「×」ボタン)
            console
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::bottom(Val::Px(6.0)),
                        border: UiRect::bottom(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(surfaces.border()),
                ))
                .with_children(|header| {
                    header.spawn((
                        Text::new("🛠️ DEBUG CONSOLE [` or Esc to close]"),
                        TextFont {
                            font: font_bold.clone().into(),
                            font_size: FontSize::Px(14.0),
                            ..default()
                        },
                        TextColor(surfaces.accent()),
                    ));

                    // 「×」閉じるボタン
                    header
                        .spawn((
                            Button,
                            DebugConsoleCloseButton,
                            Node {
                                width: Val::Px(28.0),
                                height: Val::Px(28.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                border_radius: BorderRadius::all(Val::Px(4.0)),
                                ..default()
                            },
                            BackgroundColor(standard_btn.normal()),
                            BorderColor::all(surfaces.border()),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("✕"),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(16.0),
                                    ..default()
                                },
                                TextColor(text_col.muted()),
                            ));
                        });
                });

            // ログ出力エリア（最新のログを最大12件表示）
            console
                .spawn((
                    DebugConsoleLogContainer,
                    Node {
                        width: Val::Percent(100.0),
                        flex_grow: 1.0,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexEnd,
                        overflow: Overflow::clip(),
                        padding: UiRect::all(Val::Px(4.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        row_gap: Val::Px(3.0),
                        ..default()
                    },
                    BackgroundColor(surfaces.card()),
                ))
                .with_children(|log_box| {
                    let visible_logs: Vec<_> = state.logs.iter().rev().take(12).collect();
                    for (msg, col) in visible_logs.into_iter().rev() {
                        log_box.spawn((
                            DebugConsoleLogItem,
                            Text::new(msg),
                            TextFont {
                                font: font_regular.clone().into(),
                                font_size: FontSize::Px(13.0),
                                ..default()
                            },
                            TextColor(*col),
                        ));
                    }
                });

            // コマンド入力行
            console
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(36.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Px(8.0), Val::Px(4.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        column_gap: Val::Px(6.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.02, 0.04, 0.06, 0.95)),
                    BorderColor::all(surfaces.accent()),
                ))
                .with_children(|input_row| {
                    // プロンプト記号
                    input_row.spawn((
                        Text::new(">"),
                        TextFont {
                            font: font_bold.clone().into(),
                            font_size: FontSize::Px(16.0),
                            ..default()
                        },
                        TextColor(surfaces.accent()),
                    ));

                    // 入力テキスト部
                    input_row.spawn((
                        DebugConsoleInputText,
                        Text::new(format!("{}_", state.input_text)),
                        TextFont {
                            font: font_regular.clone().into(),
                            font_size: FontSize::Px(15.0),
                            ..default()
                        },
                        TextColor(text_col.main()),
                    ));
                });
        });
}

/// ログ内容の更新
pub fn update_console_log_view(
    commands: &mut Commands,
    asset_server: &AssetServer,
    state: &DebugConsoleState,
    log_container_query: &Query<Entity, With<DebugConsoleLogContainer>>,
    log_items_query: &Query<Entity, With<DebugConsoleLogItem>>,
) {
    // 既存のログ項目を全削除
    for entity in log_items_query {
        commands.entity(entity).despawn();
    }

    let font_regular = asset_server.load(UiTheme::fonts().regular());

    if let Ok(container_entity) = log_container_query.single() {
        commands.entity(container_entity).with_children(|log_box| {
            let visible_logs: Vec<_> = state.logs.iter().rev().take(12).collect();
            for (msg, col) in visible_logs.into_iter().rev() {
                log_box.spawn((
                    DebugConsoleLogItem,
                    Text::new(msg),
                    TextFont {
                        font: font_regular.clone().into(),
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(*col),
                ));
            }
        });
    }
}
