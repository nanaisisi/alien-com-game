use bevy::prelude::*;
use crate::state::AppState;
use super::types::*;

pub fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    let font_regular = asset_server.load("fonts/UDEVGothicNF-Regular.ttf");
    let font_bold = asset_server.load("fonts/UDEVGothicNF-Bold.ttf");

    let current_res = RESOLUTION_PRESETS[settings.resolution_index];
    let res_text = format!("{}x{}", current_res.0, current_res.1);

    // 全画面オーバーレイ
    commands
        .spawn((
            SettingsRootUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.04, 0.07, 0.85)),
        ))
        .with_children(|root| {
            // メイン設定パネルウィンドウ
            root.spawn((
                Node {
                    width: Val::Px(580.0),
                    max_height: Val::Percent(92.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(16.0)),
                    row_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    ..default()
                },
                BorderColor::all(BORDER_COLOR),
                BackgroundColor(PANEL_BG),
            ))
            .with_children(|panel| {
                // ヘッダー（タイトル）
                panel.spawn((
                    Text::new("設定 (SETTINGS)"),
                    TextFont {
                        font: font_bold.clone().into(),
                        font_size: FontSize::Px(26.0),
                        ..default()
                    },
                    TextColor(ACCENT_COLOR),
                ));

                // 区切り線
                panel.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(1.0),
                        margin: UiRect::vertical(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.25, 0.38, 0.50, 0.5)),
                ));

                // 設定リストコンテナ
                panel
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(6.0),
                        ..default()
                    })
                    .with_children(|list| {
                        // --- サウンド設定セクション ---
                        spawn_section_header(list, "■ サウンド設定 (Audio)", &font_bold);

                        // 1. 主音量 (Master Volume)
                        spawn_stepper_setting_row(
                            list,
                            SettingsFocusItem::MasterVolume,
                            StepperRowConfig {
                                label: "マスター音量",
                                initial_val: &format!("{}%", settings.master_volume),
                                val_marker: SettingValueLabel::MasterVolume,
                                action_dec: SettingsButtonAction::MasterVolumeDown,
                                action_inc: SettingsButtonAction::MasterVolumeUp,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // 2. BGM音量 (BGM Volume)
                        spawn_stepper_setting_row(
                            list,
                            SettingsFocusItem::BgmVolume,
                            StepperRowConfig {
                                label: "BGM 音量",
                                initial_val: &format!("{}%", settings.bgm_volume),
                                val_marker: SettingValueLabel::BgmVolume,
                                action_dec: SettingsButtonAction::BgmVolumeDown,
                                action_inc: SettingsButtonAction::BgmVolumeUp,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // 3. 効果音量 (SFX Volume)
                        spawn_stepper_setting_row(
                            list,
                            SettingsFocusItem::SfxVolume,
                            StepperRowConfig {
                                label: "効果音量 (SFX)",
                                initial_val: &format!("{}%", settings.sfx_volume),
                                val_marker: SettingValueLabel::SfxVolume,
                                action_dec: SettingsButtonAction::SfxVolumeDown,
                                action_inc: SettingsButtonAction::SfxVolumeUp,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // --- グラフィック設定セクション ---
                        spawn_section_header(list, "■ グラフィック・画面設定 (Graphics)", &font_bold);

                        // 4. 解像度 (Resolution)
                        spawn_stepper_setting_row(
                            list,
                            SettingsFocusItem::Resolution,
                            StepperRowConfig {
                                label: "解像度",
                                initial_val: &res_text,
                                val_marker: SettingValueLabel::Resolution,
                                action_dec: SettingsButtonAction::ResolutionPrev,
                                action_inc: SettingsButtonAction::ResolutionNext,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // 5. 画面モード (Fullscreen Toggle)
                        spawn_toggle_setting_row(
                            list,
                            SettingsFocusItem::Fullscreen,
                            ToggleRowConfig {
                                label: "画面モード",
                                initial_val: if settings.fullscreen { "フルスクリーン" } else { "ウィンドウ" },
                                val_marker: SettingValueLabel::Fullscreen,
                                action_toggle: SettingsButtonAction::ToggleFullscreen,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // 6. フレームレート制限 (FPS Limit)
                        spawn_stepper_setting_row(
                            list,
                            SettingsFocusItem::FpsLimit,
                            StepperRowConfig {
                                label: "フレームレート制限",
                                initial_val: settings.fps_limit.display_label(),
                                val_marker: SettingValueLabel::FpsLimit,
                                action_dec: SettingsButtonAction::FpsLimitPrev,
                                action_inc: SettingsButtonAction::FpsLimitNext,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // 7. 垂直同期 (VSync)
                        spawn_toggle_setting_row(
                            list,
                            SettingsFocusItem::Vsync,
                            ToggleRowConfig {
                                label: "垂直同期 (VSync)",
                                initial_val: if settings.vsync { "有効 (ON)" } else { "無効 (OFF)" },
                                val_marker: SettingValueLabel::Vsync,
                                action_toggle: SettingsButtonAction::ToggleVsync,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // 8. 影の描画 (Shadows)
                        spawn_toggle_setting_row(
                            list,
                            SettingsFocusItem::Shadows,
                            ToggleRowConfig {
                                label: "影の描画 (Shadows)",
                                initial_val: if settings.shadows_enabled { "有効 (ON)" } else { "無効 (OFF)" },
                                val_marker: SettingValueLabel::Shadows,
                                action_toggle: SettingsButtonAction::ToggleShadows,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // 9. アンチエイリアス (Anti-Aliasing)
                        spawn_stepper_setting_row(
                            list,
                            SettingsFocusItem::AntiAliasing,
                            StepperRowConfig {
                                label: "アンチエイリアス",
                                initial_val: settings.anti_aliasing.display_label(),
                                val_marker: SettingValueLabel::AntiAliasing,
                                action_dec: SettingsButtonAction::AntiAliasingPrev,
                                action_inc: SettingsButtonAction::AntiAliasingNext,
                            },
                            &font_regular,
                            &font_bold,
                        );

                        // --- その他セクション ---
                        spawn_section_header(list, "■ システム (System)", &font_bold);

                        // 9. 設定初期化
                        spawn_button_setting_row(
                            list,
                            SettingsFocusItem::ResetDefaults,
                            ButtonRowConfig {
                                label: "設定の初期化",
                                button_text: "初期値に戻す (RESET)",
                                action: SettingsButtonAction::ResetDefaults,
                            },
                            &font_regular,
                            &font_bold,
                        );
                    });

                // フッターボタン群
                if settings.return_state == AppState::InGame {
                    // InGame中: 「ゲームに戻る」と「タイトルへ戻る」の2つを表示
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(16.0),
                            margin: UiRect::top(Val::Px(6.0)),
                            ..default()
                        })
                        .with_children(|row| {
                            // ゲームに戻る
                            row.spawn((
                                Button,
                                SettingsButtonAction::ResumeGame,
                                SettingsNavRow(SettingsFocusItem::Resume),
                                Node {
                                    width: Val::Px(220.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(2.0)),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BorderColor::all(BORDER_COLOR),
                                BackgroundColor(NORMAL_BUTTON),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("ゲームに戻る (RESUME)"),
                                    TextFont {
                                        font: font_bold.clone().into(),
                                        font_size: FontSize::Px(14.0),
                                        ..default()
                                    },
                                    TextColor(TEXT_COLOR),
                                ));
                            });

                            // タイトルへ戻る（確認ダイアログを開く）
                            row.spawn((
                                Button,
                                SettingsButtonAction::RequestReturnToTitle,
                                SettingsNavRow(SettingsFocusItem::ReturnToTitle),
                                Node {
                                    width: Val::Px(220.0),
                                    height: Val::Px(40.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border: UiRect::all(Val::Px(2.0)),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                BorderColor::all(Color::srgb(0.55, 0.25, 0.25)),
                                BackgroundColor(NORMAL_BUTTON),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("タイトルへ戻る (TITLE)"),
                                    TextFont {
                                        font: font_bold.clone().into(),
                                        font_size: FontSize::Px(14.0),
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.95, 0.75, 0.75)),
                                ));
                            });
                        });
                } else {
                    // タイトル画面またはポーズメニューからの設定画面: 「戻る」ボタン1つ
                    let back_label = if settings.return_state == AppState::PauseMenu {
                        "メニューへ戻る (BACK)"
                    } else {
                        "タイトルへ戻る (BACK)"
                    };
                    panel
                        .spawn((
                            Button,
                            SettingsButtonAction::Back,
                            SettingsNavRow(SettingsFocusItem::Back),
                            Node {
                                width: Val::Px(240.0),
                                height: Val::Px(40.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(6.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            BorderColor::all(BORDER_COLOR),
                            BackgroundColor(NORMAL_BUTTON),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new(back_label),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(15.0),
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                            ));
                        });
                }

                // ガイドテキスト
                panel.spawn((
                    Text::new("[↑/↓] 項目選択  [←/→] 値変更/選択  [Enter/Space] 決定  [ESC] 戻る"),
                    TextFont {
                        font: font_regular.clone().into(),
                        font_size: FontSize::Px(12.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.50, 0.62, 0.72)),
                    Node {
                        margin: UiRect::top(Val::Px(4.0)),
                        ..default()
                    },
                ));
            });
        });
}

fn spawn_section_header(
    parent: &mut ChildSpawnerCommands,
    title: &str,
    font_bold: &Handle<Font>,
) {
    parent.spawn((
        Text::new(title),
        TextFont {
            font: font_bold.clone().into(),
            font_size: FontSize::Px(13.0),
            ..default()
        },
        TextColor(Color::srgb(0.45, 0.70, 0.85)),
        Node {
            margin: UiRect::axes(Val::Px(4.0), Val::Px(2.0)),
            ..default()
        },
    ));
}

struct StepperRowConfig<'a> {
    label: &'a str,
    initial_val: &'a str,
    val_marker: SettingValueLabel,
    action_dec: SettingsButtonAction,
    action_inc: SettingsButtonAction,
}

fn spawn_stepper_setting_row(
    parent: &mut ChildSpawnerCommands,
    focus_item: SettingsFocusItem,
    cfg: StepperRowConfig,
    font_regular: &Handle<Font>,
    font_bold: &Handle<Font>,
) {
    let StepperRowConfig {
        label,
        initial_val,
        val_marker,
        action_dec,
        action_inc,
    } = cfg;
    parent
        .spawn((
            SettingsNavRow(focus_item),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(38.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BorderColor::all(Color::NONE),
            BackgroundColor(ROW_BG),
        ))
        .with_children(|row| {
            // 設定名
            row.spawn((
                Text::new(label),
                TextFont {
                    font: font_regular.clone().into(),
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // 操作部 ([-] 値 [+])
            row.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            })
            .with_children(|ctrl| {
                // [-] ボタン
                spawn_icon_button(ctrl, "-", action_dec, font_bold);

                // 値表示コンテナ
                ctrl.spawn((
                    Node {
                        width: Val::Px(90.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                ))
                .with_children(|val_box| {
                    val_box.spawn((
                        Text::new(initial_val),
                        val_marker,
                        TextFont {
                            font: font_bold.clone().into(),
                            font_size: FontSize::Px(15.0),
                            ..default()
                        },
                        TextColor(ACCENT_COLOR),
                    ));
                });

                // [+] ボタン
                spawn_icon_button(ctrl, "+", action_inc, font_bold);
            });
        });
}

struct ToggleRowConfig<'a> {
    label: &'a str,
    initial_val: &'a str,
    val_marker: SettingValueLabel,
    action_toggle: SettingsButtonAction,
}

fn spawn_toggle_setting_row(
    parent: &mut ChildSpawnerCommands,
    focus_item: SettingsFocusItem,
    cfg: ToggleRowConfig,
    font_regular: &Handle<Font>,
    font_bold: &Handle<Font>,
) {
    let ToggleRowConfig {
        label,
        initial_val,
        val_marker,
        action_toggle,
    } = cfg;
    parent
        .spawn((
            SettingsNavRow(focus_item),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(38.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BorderColor::all(Color::NONE),
            BackgroundColor(ROW_BG),
        ))
        .with_children(|row| {
            // 設定名
            row.spawn((
                Text::new(label),
                TextFont {
                    font: font_regular.clone().into(),
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // 切替ボタン
            row.spawn((
                Button,
                action_toggle,
                Node {
                    width: Val::Px(130.0),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BorderColor::all(BORDER_COLOR),
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(initial_val),
                    val_marker,
                    TextFont {
                        font: font_bold.clone().into(),
                        font_size: FontSize::Px(13.5),
                        ..default()
                    },
                    TextColor(ACCENT_COLOR),
                ));
            });
        });
}

struct ButtonRowConfig<'a> {
    label: &'a str,
    button_text: &'a str,
    action: SettingsButtonAction,
}

fn spawn_button_setting_row(
    parent: &mut ChildSpawnerCommands,
    focus_item: SettingsFocusItem,
    cfg: ButtonRowConfig,
    font_regular: &Handle<Font>,
    font_bold: &Handle<Font>,
) {
    let ButtonRowConfig {
        label,
        button_text,
        action,
    } = cfg;
    parent
        .spawn((
            SettingsNavRow(focus_item),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(38.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
            BorderColor::all(Color::NONE),
            BackgroundColor(ROW_BG),
        ))
        .with_children(|row| {
            // 項目名
            row.spawn((
                Text::new(label),
                TextFont {
                    font: font_regular.clone().into(),
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));

            // アクションボタン
            row.spawn((
                Button,
                action,
                Node {
                    padding: UiRect::axes(Val::Px(14.0), Val::Px(4.0)),
                    height: Val::Px(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.5)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                BorderColor::all(BORDER_COLOR),
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(button_text),
                    TextFont {
                        font: font_bold.clone().into(),
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                ));
            });
        });
}

fn spawn_icon_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: SettingsButtonAction,
    font_bold: &Handle<Font>,
) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: Val::Px(28.0),
                height: Val::Px(28.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor::all(BORDER_COLOR),
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font: font_bold.clone().into(),
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}

/// タイトルへ戻る確認用モーダルダイアログ
pub fn spawn_confirm_return_modal(commands: &mut Commands, asset_server: &AssetServer) {
    let font_regular = asset_server.load("fonts/UDEVGothicNF-Regular.ttf");
    let font_bold = asset_server.load("fonts/UDEVGothicNF-Bold.ttf");

    commands
        .spawn((
            ReturnToTitleConfirmModal,
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
                    BorderColor::all(DANGER_BORDER),
                    BackgroundColor(Color::srgba(0.08, 0.05, 0.06, 0.98)),
                ))
                .with_children(|modal| {
                    // ダイアログ見出し（警告）
                    modal.spawn((
                        Text::new("\u{f071} 確認 / WARNING"),
                        TextFont {
                            font: font_bold.clone().into(),
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.95, 0.35, 0.35)),
                    ));

                    // 警告メッセージ
                    modal
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(8.0),
                            ..default()
                        })
                        .with_children(|msg_container| {
                            msg_container.spawn((
                                Text::new("タイトル画面に戻りますか？"),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(17.0),
                                    ..default()
                                },
                                TextColor(TEXT_COLOR),
                            ));

                            msg_container.spawn((
                                Text::new(
                                    "※セーブ機能は未実装のため、現在の進捗は保存されずに破棄されます。",
                                ),
                                TextFont {
                                    font: font_regular.clone().into(),
                                    font_size: FontSize::Px(13.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.95, 0.65, 0.65)),
                            ));
                        });

                    // ボタンコンテナ
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
                            // 左: 同意してタイトルへ戻る（赤）
                            btn_row
                                .spawn((
                                    Button,
                                    SettingsButtonAction::ConfirmReturnToTitle,
                                    ModalNavButton(ModalFocusItem::Confirm),
                                    Node {
                                        flex_grow: 1.0,
                                        height: Val::Px(46.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(1.5)),
                                        border_radius: BorderRadius::all(Val::Px(8.0)),
                                        ..default()
                                    },
                                    BorderColor::all(DANGER_BORDER),
                                    BackgroundColor(DANGER_NORMAL_BUTTON),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("同意してタイトルへ戻る"),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(14.0),
                                            ..default()
                                        },
                                        TextColor(Color::srgb(1.0, 0.9, 0.9)),
                                    ));
                                });

                            // 右: デフォルトで中止（キャンセル）
                            btn_row
                                .spawn((
                                    Button,
                                    SettingsButtonAction::CancelReturnToTitle,
                                    ModalNavButton(ModalFocusItem::Cancel),
                                    Node {
                                        flex_grow: 1.0,
                                        height: Val::Px(46.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        border_radius: BorderRadius::all(Val::Px(8.0)),
                                        ..default()
                                    },
                                    BorderColor::all(ACCENT_COLOR),
                                    BackgroundColor(NORMAL_BUTTON),
                                ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        Text::new("中止してゲームを続ける"),
                                        TextFont {
                                            font: font_bold.clone().into(),
                                            font_size: FontSize::Px(14.0),
                                            ..default()
                                        },
                                        TextColor(TEXT_COLOR),
                                    ));
                                });
                        });

                    // モーダル操作ガイド
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

pub fn cleanup_settings_ui(
    mut commands: Commands,
    query_root: Query<Entity, With<SettingsRootUi>>,
    query_modal: Query<Entity, With<ReturnToTitleConfirmModal>>,
) {
    for entity in &query_root {
        commands.entity(entity).despawn();
    }
    for entity in &query_modal {
        commands.entity(entity).despawn();
    }
}
