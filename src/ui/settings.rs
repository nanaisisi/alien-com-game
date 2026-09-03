use bevy::prelude::*;

use crate::state::AppState;

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>()
            .init_resource::<SettingsNavFocus>()
            .add_systems(OnEnter(AppState::Settings), (setup_settings_ui, reset_settings_focus))
            .add_systems(
                Update,
                (
                    settings_keyboard_navigation_system,
                    settings_button_interaction_system,
                    settings_button_action_system,
                    update_settings_display_system,
                    update_settings_focus_highlight_system,
                )
                    .run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), cleanup_settings_ui);
    }
}

/// ゲーム全体の設定リソース
#[derive(Resource, Debug, Clone)]
pub struct GameSettings {
    pub master_volume: u32, // 0..=100 (10%刻み)
    pub bgm_volume: u32,
    pub sfx_volume: u32,
    pub fullscreen: bool,
    pub return_state: AppState,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            master_volume: 80,
            bgm_volume: 70,
            sfx_volume: 80,
            fullscreen: false,
            return_state: AppState::Title,
        }
    }
}

#[derive(Component)]
struct SettingsRootUi;

#[derive(Component)]
struct ReturnToTitleConfirmModal;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsFocusItem {
    MasterVolume,
    BgmVolume,
    SfxVolume,
    Fullscreen,
    Resume,
    ReturnToTitle,
    Back,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalFocusItem {
    Confirm,
    Cancel,
}

#[derive(Resource, Debug)]
pub struct SettingsNavFocus {
    pub current_item: SettingsFocusItem,
    pub modal_item: ModalFocusItem,
}

impl Default for SettingsNavFocus {
    fn default() -> Self {
        Self {
            current_item: SettingsFocusItem::MasterVolume,
            modal_item: ModalFocusItem::Cancel, // モーダルはデフォルトで「中止（安全）」にフォーカス
        }
    }
}

fn reset_settings_focus(mut focus: ResMut<SettingsNavFocus>) {
    focus.current_item = SettingsFocusItem::MasterVolume;
    focus.modal_item = ModalFocusItem::Cancel;
}

#[derive(Component)]
struct SettingsNavRow(SettingsFocusItem);

#[derive(Component)]
struct ModalNavButton(ModalFocusItem);

#[derive(Component, Debug, Clone, Copy)]
enum SettingsButtonAction {
    MasterVolumeDown,
    MasterVolumeUp,
    BgmVolumeDown,
    BgmVolumeUp,
    SfxVolumeDown,
    SfxVolumeUp,
    ToggleFullscreen,
    ResumeGame,
    RequestReturnToTitle,
    ConfirmReturnToTitle,
    CancelReturnToTitle,
    Back,
}

#[derive(Component)]
enum SettingValueLabel {
    MasterVolume,
    BgmVolume,
    SfxVolume,
    Fullscreen,
}

// カラーテーマ（title.rsと統一感のあるSFダークサイバー調）
const PANEL_BG: Color = Color::srgba(0.06, 0.09, 0.14, 0.94);
const ROW_BG: Color = Color::srgba(0.10, 0.14, 0.20, 0.70);
const NORMAL_BUTTON: Color = Color::srgb(0.14, 0.19, 0.26);
const HOVERED_BUTTON: Color = Color::srgb(0.22, 0.35, 0.48);
const PRESSED_BUTTON: Color = Color::srgb(0.18, 0.52, 0.68);
const BORDER_COLOR: Color = Color::srgb(0.25, 0.38, 0.50);
const ACCENT_COLOR: Color = Color::srgb(0.25, 0.85, 0.75);
const TEXT_COLOR: Color = Color::srgb(0.92, 0.95, 0.98);

// 警告・赤色ボタンカラー
const DANGER_NORMAL_BUTTON: Color = Color::srgb(0.55, 0.15, 0.18);
const DANGER_HOVERED_BUTTON: Color = Color::srgb(0.72, 0.22, 0.26);
const DANGER_PRESSED_BUTTON: Color = Color::srgb(0.85, 0.30, 0.35);
const DANGER_BORDER: Color = Color::srgb(0.80, 0.35, 0.35);

fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    let font_regular = asset_server.load("fonts/UDEVGothicNF-Regular.ttf");
    let font_bold = asset_server.load("fonts/UDEVGothicNF-Bold.ttf");

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
                    width: Val::Px(560.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(28.0)),
                    row_gap: Val::Px(20.0),
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
                        font_size: FontSize::Px(32.0),
                        ..default()
                    },
                    TextColor(ACCENT_COLOR),
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

                // 設定リストコンテナ
                panel
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        ..default()
                    })
                    .with_children(|list| {
                        // 1. 主音量 (Master Volume)
                        spawn_stepper_setting_row(
                            list,
                            SettingsFocusItem::MasterVolume,
                            StepperRowConfig {
                                label: "マスター音量 (Master)",
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

                        // 4. 画面モード (Fullscreen Toggle)
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
                    });

                // フッターボタン群
                if settings.return_state == AppState::InGame {
                    // InGame中: 「ゲームに戻る」と「タイトルへ戻る」の2つを表示
                    panel
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(16.0),
                            margin: UiRect::top(Val::Px(12.0)),
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
                                    height: Val::Px(46.0),
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
                                        font_size: FontSize::Px(15.0),
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
                                    height: Val::Px(46.0),
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
                                        font_size: FontSize::Px(15.0),
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.95, 0.75, 0.75)),
                                ));
                            });
                        });
                } else {
                    // タイトル画面等からの設定画面: 「戻る」ボタン1つ
                    panel
                        .spawn((
                            Button,
                            SettingsButtonAction::Back,
                            SettingsNavRow(SettingsFocusItem::Back),
                            Node {
                                width: Val::Px(240.0),
                                height: Val::Px(46.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(12.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            BorderColor::all(BORDER_COLOR),
                            BackgroundColor(NORMAL_BUTTON),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("タイトルへ戻る (BACK)"),
                                TextFont {
                                    font: font_bold.clone().into(),
                                    font_size: FontSize::Px(16.0),
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
                        margin: UiRect::top(Val::Px(8.0)),
                        ..default()
                    },
                ));
            });
        });
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
                height: Val::Px(48.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
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
                    font_size: FontSize::Px(16.0),
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
                        width: Val::Px(70.0),
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
                            font_size: FontSize::Px(16.0),
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
                height: Val::Px(48.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
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
                    font_size: FontSize::Px(16.0),
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
                    height: Val::Px(34.0),
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
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(ACCENT_COLOR),
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
                width: Val::Px(32.0),
                height: Val::Px(32.0),
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
                Text::new(label),
                TextFont {
                    font: font_bold.clone().into(),
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(TEXT_COLOR),
            ));
        });
}

type SettingsButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static mut BackgroundColor, &'static mut BorderColor, &'static SettingsButtonAction),
    (Changed<Interaction>, With<Button>),
>;

type SettingsButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static SettingsButtonAction),
    (Changed<Interaction>, With<Button>),
>;

fn settings_button_interaction_system(
    mut query: SettingsButtonInteractionQuery,
) {
    for (interaction, mut bg_color, mut border_color, action) in &mut query {
        let is_danger = matches!(action, SettingsButtonAction::ConfirmReturnToTitle);
        match *interaction {
            Interaction::Pressed => {
                if is_danger {
                    *bg_color = BackgroundColor(DANGER_PRESSED_BUTTON);
                    *border_color = BorderColor::all(Color::WHITE);
                } else {
                    *bg_color = BackgroundColor(PRESSED_BUTTON);
                    *border_color = BorderColor::all(ACCENT_COLOR);
                }
            }
            Interaction::Hovered => {
                if is_danger {
                    *bg_color = BackgroundColor(DANGER_HOVERED_BUTTON);
                    *border_color = BorderColor::all(DANGER_BORDER);
                } else {
                    *bg_color = BackgroundColor(HOVERED_BUTTON);
                    *border_color = BorderColor::all(ACCENT_COLOR);
                }
            }
            Interaction::None => {
                if is_danger {
                    *bg_color = BackgroundColor(DANGER_NORMAL_BUTTON);
                    *border_color = BorderColor::all(DANGER_BORDER);
                } else {
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                    *border_color = BorderColor::all(BORDER_COLOR);
                }
            }
        }
    }
}

fn settings_button_action_system(
    mut commands: Commands,
    interaction_query: SettingsButtonActionQuery,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>,
    modal_query: Query<Entity, With<ReturnToTitleConfirmModal>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                SettingsButtonAction::MasterVolumeDown => {
                    settings.master_volume = settings.master_volume.saturating_sub(10);
                }
                SettingsButtonAction::MasterVolumeUp => {
                    settings.master_volume = (settings.master_volume + 10).min(100);
                }
                SettingsButtonAction::BgmVolumeDown => {
                    settings.bgm_volume = settings.bgm_volume.saturating_sub(10);
                }
                SettingsButtonAction::BgmVolumeUp => {
                    settings.bgm_volume = (settings.bgm_volume + 10).min(100);
                }
                SettingsButtonAction::SfxVolumeDown => {
                    settings.sfx_volume = settings.sfx_volume.saturating_sub(10);
                }
                SettingsButtonAction::SfxVolumeUp => {
                    settings.sfx_volume = (settings.sfx_volume + 10).min(100);
                }
                SettingsButtonAction::ToggleFullscreen => {
                    settings.fullscreen = !settings.fullscreen;
                    if let Ok(mut window) = windows.single_mut() {
                        window.mode = if settings.fullscreen {
                            bevy::window::WindowMode::BorderlessFullscreen(
                                MonitorSelection::Current,
                            )
                        } else {
                            bevy::window::WindowMode::Windowed
                        };
                    }
                }
                SettingsButtonAction::ResumeGame => {
                    info!("Resuming game...");
                    next_state.set(AppState::InGame);
                }
                SettingsButtonAction::RequestReturnToTitle => {
                    info!("Requesting return to title (opening confirmation modal)...");
                    if modal_query.is_empty() {
                        spawn_confirm_return_modal(&mut commands, &asset_server);
                    }
                }
                SettingsButtonAction::ConfirmReturnToTitle => {
                    info!("Agreed to return to title. Discarding unsaved session...");
                    for entity in &modal_query {
                        commands.entity(entity).despawn();
                    }
                    settings.return_state = AppState::Title;
                    next_state.set(AppState::Title);
                }
                SettingsButtonAction::CancelReturnToTitle => {
                    info!("Cancelled return to title.");
                    for entity in &modal_query {
                        commands.entity(entity).despawn();
                    }
                }
                SettingsButtonAction::Back => {
                    info!("Returning to {:?}...", settings.return_state);
                    next_state.set(settings.return_state);
                }
            }
        }
    }
}

/// タイトルへ戻る確認用モーダルダイアログ
fn spawn_confirm_return_modal(commands: &mut Commands, asset_server: &AssetServer) {
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

                    // ボタンコンテナ（左: 同意してタイトルへ戻る[赤]、右: デフォルト/中止してゲームを続ける[青・グレー]）
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

fn update_settings_display_system(
    settings: Res<GameSettings>,
    mut query: Query<(&mut Text, &SettingValueLabel)>,
) {
    if !settings.is_changed() {
        return;
    }

    for (mut text, marker) in &mut query {
        match marker {
            SettingValueLabel::MasterVolume => {
                **text = format!("{}%", settings.master_volume);
            }
            SettingValueLabel::BgmVolume => {
                **text = format!("{}%", settings.bgm_volume);
            }
            SettingValueLabel::SfxVolume => {
                **text = format!("{}%", settings.sfx_volume);
            }
            SettingValueLabel::Fullscreen => {
                **text = if settings.fullscreen {
                    "フルスクリーン".to_string()
                } else {
                    "ウィンドウ".to_string()
                };
            }
        }
    }
}

use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
struct SettingsNavState<'w> {
    focus: ResMut<'w, SettingsNavFocus>,
    settings: ResMut<'w, GameSettings>,
    next_state: ResMut<'w, NextState<AppState>>,
}

/// 矢印キーおよび決定・キャンセルキーによる設定画面のキーボード操作システム
fn settings_keyboard_navigation_system(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut state: SettingsNavState,
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>,
    modal_query: Query<Entity, With<ReturnToTitleConfirmModal>>,
) {
    let modal_open = !modal_query.is_empty();

    // ==========================================
    // 1. 確認モーダル表示中のキーボード操作
    // ==========================================
    if modal_open {
        // 左右キー / Tab で「同意（Confirm）」と「中止（Cancel）」の切り替え
        if keys.just_pressed(KeyCode::ArrowLeft)
            || keys.just_pressed(KeyCode::ArrowRight)
            || keys.just_pressed(KeyCode::KeyA)
            || keys.just_pressed(KeyCode::KeyD)
            || keys.just_pressed(KeyCode::Tab)
        {
            state.focus.modal_item = match state.focus.modal_item {
                ModalFocusItem::Confirm => ModalFocusItem::Cancel,
                ModalFocusItem::Cancel => ModalFocusItem::Confirm,
            };
        }

        // ESC: 中止してモーダルを閉じる
        if keys.just_pressed(KeyCode::Escape) {
            info!("ESC pressed: Cancelling return to title.");
            for entity in &modal_query {
                commands.entity(entity).despawn();
            }
            state.focus.modal_item = ModalFocusItem::Cancel;
            return;
        }

        // Enter / Space: 選択中のボタンを実行
        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
            match state.focus.modal_item {
                ModalFocusItem::Confirm => {
                    info!("Agreed to return to title. Discarding unsaved session...");
                    for entity in &modal_query {
                        commands.entity(entity).despawn();
                    }
                    state.settings.return_state = AppState::Title;
                    state.next_state.set(AppState::Title);
                }
                ModalFocusItem::Cancel => {
                    info!("Cancelled return to title.");
                    for entity in &modal_query {
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
        return;
    }

    // ==========================================
    // 2. メイン設定画面でのキーボード操作
    // ==========================================
    let is_ingame = state.settings.return_state == AppState::InGame;

    let nav_items: &[SettingsFocusItem] = if is_ingame {
        &[
            SettingsFocusItem::MasterVolume,
            SettingsFocusItem::BgmVolume,
            SettingsFocusItem::SfxVolume,
            SettingsFocusItem::Fullscreen,
            SettingsFocusItem::Resume,
            SettingsFocusItem::ReturnToTitle,
        ]
    } else {
        &[
            SettingsFocusItem::MasterVolume,
            SettingsFocusItem::BgmVolume,
            SettingsFocusItem::SfxVolume,
            SettingsFocusItem::Fullscreen,
            SettingsFocusItem::Back,
        ]
    };

    let current_idx = nav_items
        .iter()
        .position(|&item| item == state.focus.current_item)
        .unwrap_or(0);

    // [↑] / [W] で上移動
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        let prev_idx = if current_idx == 0 {
            nav_items.len() - 1
        } else {
            current_idx - 1
        };
        state.focus.current_item = nav_items[prev_idx];
    }

    // [↓] / [S] で下移動
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        let next_idx = (current_idx + 1) % nav_items.len();
        state.focus.current_item = nav_items[next_idx];
    }

    // [←] / [A] で減少または切り替え
    if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        match state.focus.current_item {
            SettingsFocusItem::MasterVolume => {
                state.settings.master_volume = state.settings.master_volume.saturating_sub(10);
            }
            SettingsFocusItem::BgmVolume => {
                state.settings.bgm_volume = state.settings.bgm_volume.saturating_sub(10);
            }
            SettingsFocusItem::SfxVolume => {
                state.settings.sfx_volume = state.settings.sfx_volume.saturating_sub(10);
            }
            SettingsFocusItem::Fullscreen => {
                toggle_fullscreen(&mut state.settings, &mut windows);
            }
            SettingsFocusItem::ReturnToTitle => {
                // InGameの下部ボタン間移動: タイトルへ戻る ← ゲームに戻る
                state.focus.current_item = SettingsFocusItem::Resume;
            }
            _ => {}
        }
    }

    // [→] / [D] で増加または切り替え
    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        match state.focus.current_item {
            SettingsFocusItem::MasterVolume => {
                state.settings.master_volume = (state.settings.master_volume + 10).min(100);
            }
            SettingsFocusItem::BgmVolume => {
                state.settings.bgm_volume = (state.settings.bgm_volume + 10).min(100);
            }
            SettingsFocusItem::SfxVolume => {
                state.settings.sfx_volume = (state.settings.sfx_volume + 10).min(100);
            }
            SettingsFocusItem::Fullscreen => {
                toggle_fullscreen(&mut state.settings, &mut windows);
            }
            SettingsFocusItem::Resume => {
                // InGameの下部ボタン間移動: ゲームに戻る → タイトルへ戻る
                state.focus.current_item = SettingsFocusItem::ReturnToTitle;
            }
            _ => {}
        }
    }

    // [Enter] / [Space] 決定キー
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        match state.focus.current_item {
            SettingsFocusItem::Fullscreen => {
                toggle_fullscreen(&mut state.settings, &mut windows);
            }
            SettingsFocusItem::Resume => {
                info!("Resuming game via Enter/Space...");
                state.next_state.set(AppState::InGame);
            }
            SettingsFocusItem::ReturnToTitle => {
                info!("Requesting return to title via Enter/Space (opening modal)...");
                state.focus.modal_item = ModalFocusItem::Cancel;
                spawn_confirm_return_modal(&mut commands, &asset_server);
            }
            SettingsFocusItem::Back => {
                info!("Returning to {:?} via Enter/Space...", state.settings.return_state);
                state.next_state.set(state.settings.return_state);
            }
            _ => {}
        }
    }

    // [Escape] キャンセル/戻るキー
    if keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed: Returning to {:?}...", state.settings.return_state);
        state.next_state.set(state.settings.return_state);
    }
}

fn toggle_fullscreen(settings: &mut GameSettings, windows: &mut Query<&mut Window>) {
    settings.fullscreen = !settings.fullscreen;
    if let Ok(mut window) = windows.single_mut() {
        window.mode = if settings.fullscreen {
            bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        } else {
            bevy::window::WindowMode::Windowed
        };
    }
}

/// フォーカスされている設定行やモーダルボタンのボーダー/背景ハイライト
fn update_settings_focus_highlight_system(
    focus: Res<SettingsNavFocus>,
    modal_query: Query<Entity, With<ReturnToTitleConfirmModal>>,
    mut row_query: Query<
        (&SettingsNavRow, &mut BorderColor, &mut BackgroundColor, Option<&Button>),
        Without<ModalNavButton>,
    >,
    mut modal_btn_query: Query<
        (&ModalNavButton, &mut BorderColor, &mut BackgroundColor),
        Without<SettingsNavRow>,
    >,
) {
    let modal_open = !modal_query.is_empty();

    if modal_open {
        // モーダル表示中はモーダル側ボタンのフォーカスハイライト
        for (modal_btn, mut border_color, mut bg_color) in &mut modal_btn_query {
            let is_focused = focus.modal_item == modal_btn.0;
            match modal_btn.0 {
                ModalFocusItem::Confirm => {
                    if is_focused {
                        *border_color = BorderColor::all(Color::WHITE);
                        *bg_color = BackgroundColor(DANGER_HOVERED_BUTTON);
                    } else {
                        *border_color = BorderColor::all(DANGER_BORDER);
                        *bg_color = BackgroundColor(DANGER_NORMAL_BUTTON);
                    }
                }
                ModalFocusItem::Cancel => {
                    if is_focused {
                        *border_color = BorderColor::all(ACCENT_COLOR);
                        *bg_color = BackgroundColor(HOVERED_BUTTON);
                    } else {
                        *border_color = BorderColor::all(BORDER_COLOR);
                        *bg_color = BackgroundColor(NORMAL_BUTTON);
                    }
                }
            }
        }
    } else {
        // メイン画面の行・ボタンのフォーカスハイライト
        for (nav_row, mut border_color, mut bg_color, opt_button) in &mut row_query {
            let is_focused = focus.current_item == nav_row.0;

            if opt_button.is_some() {
                // ボタン（Resume, ReturnToTitle, Back）
                let is_danger = nav_row.0 == SettingsFocusItem::ReturnToTitle;
                if is_danger {
                    if is_focused {
                        *border_color = BorderColor::all(DANGER_BORDER);
                        *bg_color = BackgroundColor(DANGER_HOVERED_BUTTON);
                    } else {
                        *border_color = BorderColor::all(Color::srgb(0.55, 0.25, 0.25));
                        *bg_color = BackgroundColor(NORMAL_BUTTON);
                    }
                } else if is_focused {
                    *border_color = BorderColor::all(ACCENT_COLOR);
                    *bg_color = BackgroundColor(HOVERED_BUTTON);
                } else {
                    *border_color = BorderColor::all(BORDER_COLOR);
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                }
            } else {
                // 設定スライダー行（Rowコンテナ）
                if is_focused {
                    *border_color = BorderColor::all(ACCENT_COLOR);
                    *bg_color = BackgroundColor(Color::srgba(0.14, 0.20, 0.30, 0.90));
                } else {
                    *border_color = BorderColor::all(Color::NONE);
                    *bg_color = BackgroundColor(ROW_BG);
                }
            }
        }
    }
}

fn cleanup_settings_ui(
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

