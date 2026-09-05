use bevy::prelude::*;

use super::types::*;
use crate::ui::theme::UiTheme;

pub fn setup_pause_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_regular = asset_server.load(UiTheme::FONTS.regular);
    let font_bold = asset_server.load(UiTheme::FONTS.bold);

    // 全画面オーバーレイ（背後のゲーム画面をうっすら暗く透かす）
    commands
        .spawn((
            PauseMenuRootUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.04, 0.07, 0.82)),
            GlobalZIndex(80),
        ))
        .with_children(|root| {
            // メインポーズパネルウィンドウ（縦型）
            root.spawn((
                Node {
                    width: Val::Px(380.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(32.0)),
                    row_gap: Val::Px(16.0),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BorderColor::all(UiTheme::SURFACES.border),
                BackgroundColor(UiTheme::SURFACES.panel),
            ))
            .with_children(|panel| {
                // ヘッダータイトル
                panel.spawn((
                    Text::new("一時停止 (PAUSE)"),
                    TextFont {
                        font: font_bold.clone().into(),
                        font_size: FontSize::Px(26.0),
                        ..default()
                    },
                    TextColor(UiTheme::SURFACES.accent),
                ));

                // 区切り線
                panel.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(1.0),
                        margin: UiRect::vertical(Val::Px(4.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.25, 0.38, 0.50, 0.5)),
                ));

                // 縦並びボタンリスト
                let buttons_info = [
                    (
                        "ゲームに戻る (RESUME)",
                        PauseMenuItem::Resume,
                        PauseButtonAction::Resume,
                        false,
                    ),
                    (
                        "設定 (SETTINGS)",
                        PauseMenuItem::Settings,
                        PauseButtonAction::Settings,
                        false,
                    ),
                    (
                        "メインメニューに戻る (TITLE)",
                        PauseMenuItem::ReturnToTitle,
                        PauseButtonAction::RequestReturnToTitle,
                        true,
                    ),
                    (
                        "デスクトップに戻る (QUIT)",
                        PauseMenuItem::QuitToDesktop,
                        PauseButtonAction::RequestQuitToDesktop,
                        true,
                    ),
                ];

                for (label, item, action, is_danger_type) in buttons_info {
                    panel
                        .spawn((
                            Button,
                            action,
                            PauseMenuButton(item),
                            Node {
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(2.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            BorderColor::all(if is_danger_type {
                                Color::srgb(0.55, 0.25, 0.25)
                            } else {
                                UiTheme::SURFACES.border
                            }),
                            BackgroundColor(UiTheme::BUTTONS.standard.normal),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new(label),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(15.0),
                                    ..default()
                                },
                                TextColor(if is_danger_type {
                                    Color::srgb(0.95, 0.75, 0.75)
                                } else {
                                    UiTheme::TEXT.main
                                }),
                            ));
                        });
                }

                // 操作ガイドテキスト
                panel.spawn((
                    Text::new("[↑/↓] 選択  [Enter/Space] 決定  [ESC] ゲームに戻る"),
                    TextFont {
                        font: font_regular.clone().into(),
                        font_size: FontSize::Px(12.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.50, 0.62, 0.72)),
                    Node {
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                ));
            });
        });
}

pub fn spawn_pause_confirm_modal(
    commands: &mut Commands,
    asset_server: &AssetServer,
    modal_type: PauseModalType,
) {
    let font_regular = asset_server.load(UiTheme::FONTS.regular);
    let font_bold = asset_server.load(UiTheme::FONTS.bold);

    let (title_text, desc_text, confirm_btn_text) = match modal_type {
        PauseModalType::ReturnToTitle => (
            "タイトル画面に戻りますか？",
            "※セーブ機能は未実装のため、現在の進捗は保存されずに破棄されます。",
            "同意してタイトルへ戻る",
        ),
        PauseModalType::QuitToDesktop => (
            "デスクトップに戻りますか？",
            "※ゲームを終了します。保存されていない進捗は破棄されます。",
            "終了してデスクトップへ戻る",
        ),
    };

    commands
        .spawn((
            PauseConfirmModal,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.01, 0.02, 0.04, 0.88)),
            GlobalZIndex(100),
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        width: Val::Px(520.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(28.0)),
                        row_gap: Val::Px(20.0),
                        border: UiRect::all(Val::Px(2.0)),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BorderColor::all(UiTheme::BUTTONS.danger.border),
                    BackgroundColor(Color::srgba(0.08, 0.05, 0.06, 0.98)),
                ))
                .with_children(|modal| {
                    // 警告アイコンと見出し
                    modal.spawn((
                        Text::new("\u{f071} 確認 / WARNING"),
                        TextFont {
                            font: font_bold.clone().into(),
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.35, 0.35)),
                    ));

                    // メッセージ文
                    modal
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|msg_container| {
                            msg_container.spawn((
                                Text::new(title_text),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(17.0),
                                    ..default()
                                },
                                TextColor(UiTheme::TEXT.main),
                            ));

                            msg_container.spawn((
                                Text::new(desc_text),
                                TextFont {
                                    font: font_regular.clone().into(),
                                    font_size: FontSize::Px(13.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.95, 0.65, 0.65)),
                            ));
                        });

                    // ボタン列
                    modal
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            width: Val::Percent(100.0),
                            column_gap: Val::Px(16.0),
                            margin: UiRect::top(Val::Px(12.0)),
                            ..default()
                        })
                        .with_children(|btn_row| {
                            // 左: 同意ボタン（赤）
                            btn_row
                                .spawn((
                                    Button,
                                    PauseButtonAction::ConfirmModal,
                                    PauseModalButton(PauseModalFocusItem::Confirm),
                                    Node {
                                        flex_grow: 1.0,
                                        height: Val::Px(46.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.5)),
                                        border_radius: BorderRadius::all(Val::Px(8.0)),
                                        ..default()
                                    },
                                    BorderColor::all(UiTheme::BUTTONS.danger.border),
                                    BackgroundColor(UiTheme::BUTTONS.danger.normal),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new(confirm_btn_text),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(14.0),
                                            ..default()
                                        },
                                        TextColor(Color::srgb(1.0, 0.9, 0.9)),
                                    ));
                                });

                            // 右: 中止ボタン（デフォルト安全選択）
                            btn_row
                                .spawn((
                                    Button,
                                    PauseButtonAction::CancelModal,
                                    PauseModalButton(PauseModalFocusItem::Cancel),
                                    Node {
                                        flex_grow: 1.0,
                                        height: Val::Px(46.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        border_radius: BorderRadius::all(Val::Px(8.0)),
                                        ..default()
                                    },
                                    BorderColor::all(UiTheme::SURFACES.accent),
                                    BackgroundColor(UiTheme::BUTTONS.standard.normal),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("中止してゲームを続ける"),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(14.0),
                                            ..default()
                                        },
                                        TextColor(UiTheme::TEXT.main),
                                    ));
                                });
                        });

                    // 操作ガイド
                    modal.spawn((
                        Text::new("[←/→] 選択  [Enter/Space] 決定  [ESC] キャンセル"),
                        TextFont {
                            font: font_regular.clone().into(),
                            font_size: FontSize::Px(12.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.70, 0.75, 0.80)),
                    ));
                });
        });
}

pub fn cleanup_pause_menu_ui(
    mut commands: Commands,
    root_query: Query<Entity, With<PauseMenuRootUi>>,
    modal_query: Query<Entity, With<PauseConfirmModal>>,
) {
    for entity in &root_query {
        commands.entity(entity).despawn();
    }
    for entity in &modal_query {
        commands.entity(entity).despawn();
    }
}
