use bevy::prelude::*;

use crate::state::AppState;

pub struct SettingsUiPlugin;

impl Plugin for SettingsUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>()
            .add_systems(OnEnter(AppState::Settings), setup_settings_ui)
            .add_systems(
                Update,
                (
                    settings_button_interaction_system,
                    settings_button_action_system,
                    update_settings_display_system,
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

#[derive(Component, Debug, Clone, Copy)]
enum SettingsButtonAction {
    MasterVolumeDown,
    MasterVolumeUp,
    BgmVolumeDown,
    BgmVolumeUp,
    SfxVolumeDown,
    SfxVolumeUp,
    ToggleFullscreen,
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
                            "画面モード",
                            if settings.fullscreen { "フルスクリーン" } else { "ウィンドウ" },
                            SettingValueLabel::Fullscreen,
                            SettingsButtonAction::ToggleFullscreen,
                            &font_regular,
                            &font_bold,
                        );
                    });

                // フッター/戻るボタン
                panel
                    .spawn((
                        Button,
                        SettingsButtonAction::Back,
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
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(48.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
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

fn spawn_toggle_setting_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    initial_val: &str,
    val_marker: SettingValueLabel,
    action_toggle: SettingsButtonAction,
    font_regular: &Handle<Font>,
    font_bold: &Handle<Font>,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(48.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..default()
            },
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
    (&'static Interaction, &'static mut BackgroundColor, &'static mut BorderColor),
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
    for (interaction, mut bg_color, mut border_color) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(PRESSED_BUTTON);
                *border_color = BorderColor::all(ACCENT_COLOR);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(HOVERED_BUTTON);
                *border_color = BorderColor::all(ACCENT_COLOR);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(NORMAL_BUTTON);
                *border_color = BorderColor::all(BORDER_COLOR);
            }
        }
    }
}

fn settings_button_action_system(
    interaction_query: SettingsButtonActionQuery,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut windows: Query<&mut Window>,
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
                SettingsButtonAction::Back => {
                    info!("Returning to {:?}...", settings.return_state);
                    next_state.set(settings.return_state);
                }
            }
        }
    }
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

fn cleanup_settings_ui(mut commands: Commands, query: Query<Entity, With<SettingsRootUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
