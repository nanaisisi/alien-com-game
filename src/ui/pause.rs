use bevy::app::AppExit;
use bevy::prelude::*;

use crate::state::AppState;
use crate::ui::settings::GameSettings;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PauseMenuFocus>()
            .add_systems(
                OnEnter(AppState::PauseMenu),
                (setup_pause_menu_ui, reset_pause_focus),
            )
            .add_systems(
                Update,
                (
                    pause_keyboard_navigation_system,
                    pause_button_interaction_system,
                    pause_button_action_system,
                    update_pause_focus_highlight_system,
                )
                    .run_if(in_state(AppState::PauseMenu)),
            )
            .add_systems(OnExit(AppState::PauseMenu), cleanup_pause_menu_ui);
    }
}

#[derive(Component)]
struct PauseMenuRootUi;

#[derive(Component)]
struct PauseConfirmModal;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PauseModalType {
    #[default]
    ReturnToTitle,
    QuitToDesktop,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuItem {
    Resume,
    Settings,
    ReturnToTitle,
    QuitToDesktop,
}

const PAUSE_MENU_ITEMS: [PauseMenuItem; 4] = [
    PauseMenuItem::Resume,
    PauseMenuItem::Settings,
    PauseMenuItem::ReturnToTitle,
    PauseMenuItem::QuitToDesktop,
];

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseModalFocusItem {
    Confirm,
    Cancel,
}

#[derive(Resource, Debug)]
pub struct PauseMenuFocus {
    pub current_item: PauseMenuItem,
    pub modal_open: bool,
    pub modal_type: PauseModalType,
    pub modal_focus: PauseModalFocusItem,
}

impl Default for PauseMenuFocus {
    fn default() -> Self {
        Self {
            current_item: PauseMenuItem::Resume,
            modal_open: false,
            modal_type: PauseModalType::ReturnToTitle,
            modal_focus: PauseModalFocusItem::Cancel,
        }
    }
}

fn reset_pause_focus(mut focus: ResMut<PauseMenuFocus>) {
    focus.current_item = PauseMenuItem::Resume;
    focus.modal_open = false;
    focus.modal_type = PauseModalType::ReturnToTitle;
    focus.modal_focus = PauseModalFocusItem::Cancel;
}

#[derive(Component, Debug, Clone, Copy)]
pub enum PauseButtonAction {
    Resume,
    Settings,
    RequestReturnToTitle,
    RequestQuitToDesktop,
    ConfirmModal,
    CancelModal,
}

#[derive(Component)]
struct PauseMenuButton(PauseMenuItem);

#[derive(Component)]
struct PauseModalButton(PauseModalFocusItem);

// カラーテーマ（title.rs / settings.rs と統一したサイバー調）
const PANEL_BG: Color = Color::srgba(0.06, 0.09, 0.14, 0.95);
const NORMAL_BUTTON: Color = Color::srgb(0.12, 0.16, 0.22);
const HOVERED_BUTTON: Color = Color::srgb(0.20, 0.32, 0.45);
const PRESSED_BUTTON: Color = Color::srgb(0.15, 0.50, 0.65);
const BORDER_COLOR: Color = Color::srgb(0.25, 0.38, 0.50);
const ACCENT_COLOR: Color = Color::srgb(0.25, 0.85, 0.75);
const TEXT_COLOR: Color = Color::srgb(0.92, 0.95, 0.98);

// 警告・危険ボタンカラー
const DANGER_NORMAL_BUTTON: Color = Color::srgb(0.55, 0.15, 0.18);
const DANGER_HOVERED_BUTTON: Color = Color::srgb(0.72, 0.22, 0.26);
const DANGER_PRESSED_BUTTON: Color = Color::srgb(0.85, 0.30, 0.35);
const DANGER_BORDER: Color = Color::srgb(0.80, 0.35, 0.35);

fn setup_pause_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_regular = asset_server.load("fonts/UDEVGothicNF-Regular.ttf");
    let font_bold = asset_server.load("fonts/UDEVGothicNF-Bold.ttf");

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
                BorderColor::all(BORDER_COLOR),
                BackgroundColor(PANEL_BG),
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
                                BORDER_COLOR
                            }),
                            BackgroundColor(NORMAL_BUTTON),
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
                                    TEXT_COLOR
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

use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
struct PauseNavContext<'w, 's> {
    commands: Commands<'w, 's>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    focus: ResMut<'w, PauseMenuFocus>,
    next_state: ResMut<'w, NextState<AppState>>,
    settings: ResMut<'w, GameSettings>,
    exit_events: MessageWriter<'w, AppExit>,
    asset_server: Res<'w, AssetServer>,
    modal_query: Query<'w, 's, Entity, With<PauseConfirmModal>>,
}

fn pause_keyboard_navigation_system(
    mut ctx: PauseNavContext,
) {
    let PauseNavContext {
        ref mut commands,
        ref keys,
        ref mut focus,
        ref mut next_state,
        ref mut settings,
        ref mut exit_events,
        ref asset_server,
        ref modal_query,
    } = ctx;
    // ==========================================
    // 1. モーダル表示中のキーボード操作
    // ==========================================
    if focus.modal_open {
        if keys.just_pressed(KeyCode::ArrowLeft)
            || keys.just_pressed(KeyCode::ArrowRight)
            || keys.just_pressed(KeyCode::KeyA)
            || keys.just_pressed(KeyCode::KeyD)
            || keys.just_pressed(KeyCode::Tab)
        {
            focus.modal_focus = match focus.modal_focus {
                PauseModalFocusItem::Confirm => PauseModalFocusItem::Cancel,
                PauseModalFocusItem::Cancel => PauseModalFocusItem::Confirm,
            };
        }

        // ESC: 中止
        if keys.just_pressed(KeyCode::Escape) {
            info!("ESC pressed in PauseModal: Cancelling modal.");
            for entity in modal_query {
                commands.entity(entity).despawn();
            }
            focus.modal_open = false;
            focus.modal_focus = PauseModalFocusItem::Cancel;
            return;
        }

        // Enter / Space: 決定
        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
            match focus.modal_focus {
                PauseModalFocusItem::Confirm => {
                    for entity in modal_query {
                        commands.entity(entity).despawn();
                    }
                    focus.modal_open = false;
                    match focus.modal_type {
                        PauseModalType::ReturnToTitle => {
                            info!("Agreed to return to title. Transitioning to Title...");
                            settings.return_state = AppState::Title;
                            next_state.set(AppState::Title);
                        }
                        PauseModalType::QuitToDesktop => {
                            info!("Agreed to quit to desktop. Exiting game...");
                            exit_events.write(AppExit::Success);
                        }
                    }
                }
                PauseModalFocusItem::Cancel => {
                    info!("Cancelled pause modal.");
                    for entity in modal_query {
                        commands.entity(entity).despawn();
                    }
                    focus.modal_open = false;
                }
            }
        }
        return;
    }

    // ==========================================
    // 2. メインポーズメニューのキーボード操作
    // ==========================================
    let current_idx = PAUSE_MENU_ITEMS
        .iter()
        .position(|&item| item == focus.current_item)
        .unwrap_or(0);

    // [↑] / [W]
    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        let prev_idx = if current_idx == 0 {
            PAUSE_MENU_ITEMS.len() - 1
        } else {
            current_idx - 1
        };
        focus.current_item = PAUSE_MENU_ITEMS[prev_idx];
    }

    // [↓] / [S]
    if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        let next_idx = (current_idx + 1) % PAUSE_MENU_ITEMS.len();
        focus.current_item = PAUSE_MENU_ITEMS[next_idx];
    }

    // [Enter] / [Space]
    if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space) {
        match focus.current_item {
            PauseMenuItem::Resume => {
                info!("Resuming game via Enter/Space...");
                next_state.set(AppState::InGame);
            }
            PauseMenuItem::Settings => {
                info!("Opening Settings from Pause Menu via Enter/Space...");
                settings.return_state = AppState::PauseMenu;
                next_state.set(AppState::Settings);
            }
            PauseMenuItem::ReturnToTitle => {
                info!("Requesting return to title via Enter/Space...");
                focus.modal_open = true;
                focus.modal_type = PauseModalType::ReturnToTitle;
                focus.modal_focus = PauseModalFocusItem::Cancel;
                spawn_pause_confirm_modal(commands, asset_server, PauseModalType::ReturnToTitle);
            }
            PauseMenuItem::QuitToDesktop => {
                info!("Requesting quit to desktop via Enter/Space...");
                focus.modal_open = true;
                focus.modal_type = PauseModalType::QuitToDesktop;
                focus.modal_focus = PauseModalFocusItem::Cancel;
                spawn_pause_confirm_modal(commands, asset_server, PauseModalType::QuitToDesktop);
            }
        }
    }

    // [ESC] ゲームに戻る
    if keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed in Pause Menu: Resuming game...");
        next_state.set(AppState::InGame);
    }
}

type PauseButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static mut BackgroundColor, &'static mut BorderColor, &'static PauseButtonAction),
    (Changed<Interaction>, With<Button>),
>;

fn pause_button_interaction_system(mut query: PauseButtonInteractionQuery) {
    for (interaction, mut bg_color, mut border_color, action) in &mut query {
        let is_danger = matches!(
            action,
            PauseButtonAction::ConfirmModal
                | PauseButtonAction::RequestReturnToTitle
                | PauseButtonAction::RequestQuitToDesktop
        );
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
                if matches!(action, PauseButtonAction::ConfirmModal) {
                    *bg_color = BackgroundColor(DANGER_NORMAL_BUTTON);
                    *border_color = BorderColor::all(DANGER_BORDER);
                } else if is_danger {
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                    *border_color = BorderColor::all(Color::srgb(0.55, 0.25, 0.25));
                } else {
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                    *border_color = BorderColor::all(BORDER_COLOR);
                }
            }
        }
    }
}

type PauseButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static PauseButtonAction),
    (Changed<Interaction>, With<Button>),
>;

#[derive(SystemParam)]
struct PauseActionContext<'w, 's> {
    commands: Commands<'w, 's>,
    focus: ResMut<'w, PauseMenuFocus>,
    settings: ResMut<'w, GameSettings>,
    next_state: ResMut<'w, NextState<AppState>>,
    exit_events: MessageWriter<'w, AppExit>,
    asset_server: Res<'w, AssetServer>,
    modal_query: Query<'w, 's, Entity, With<PauseConfirmModal>>,
}

fn pause_button_action_system(
    interaction_query: PauseButtonActionQuery,
    mut ctx: PauseActionContext,
) {
    let PauseActionContext {
        ref mut commands,
        ref mut focus,
        ref mut settings,
        ref mut next_state,
        ref mut exit_events,
        ref asset_server,
        ref modal_query,
    } = ctx;
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                PauseButtonAction::Resume => {
                    info!("Resuming game from Pause Menu...");
                    next_state.set(AppState::InGame);
                }
                PauseButtonAction::Settings => {
                    info!("Opening Settings from Pause Menu...");
                    settings.return_state = AppState::PauseMenu;
                    next_state.set(AppState::Settings);
                }
                PauseButtonAction::RequestReturnToTitle => {
                    info!("Requesting return to title modal...");
                    if modal_query.is_empty() {
                        focus.modal_open = true;
                        focus.modal_type = PauseModalType::ReturnToTitle;
                        focus.modal_focus = PauseModalFocusItem::Cancel;
                        spawn_pause_confirm_modal(commands, asset_server, PauseModalType::ReturnToTitle);
                    }
                }
                PauseButtonAction::RequestQuitToDesktop => {
                    info!("Requesting quit to desktop modal...");
                    if modal_query.is_empty() {
                        focus.modal_open = true;
                        focus.modal_type = PauseModalType::QuitToDesktop;
                        focus.modal_focus = PauseModalFocusItem::Cancel;
                        spawn_pause_confirm_modal(commands, asset_server, PauseModalType::QuitToDesktop);
                    }
                }
                PauseButtonAction::ConfirmModal => {
                    for entity in modal_query {
                        commands.entity(entity).despawn();
                    }
                    focus.modal_open = false;
                    match focus.modal_type {
                        PauseModalType::ReturnToTitle => {
                            info!("Agreed to return to title.");
                            settings.return_state = AppState::Title;
                            next_state.set(AppState::Title);
                        }
                        PauseModalType::QuitToDesktop => {
                            info!("Agreed to quit to desktop.");
                            exit_events.write(AppExit::Success);
                        }
                    }
                }
                PauseButtonAction::CancelModal => {
                    info!("Cancelled pause modal.");
                    for entity in modal_query {
                        commands.entity(entity).despawn();
                    }
                    focus.modal_open = false;
                }
            }
        }
    }
}

fn spawn_pause_confirm_modal(
    commands: &mut Commands,
    asset_server: &AssetServer,
    modal_type: PauseModalType,
) {
    let font_regular = asset_server.load("fonts/UDEVGothicNF-Regular.ttf");
    let font_bold = asset_server.load("fonts/UDEVGothicNF-Bold.ttf");

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
                    BorderColor::all(DANGER_BORDER),
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
                                TextColor(TEXT_COLOR),
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
                                    BorderColor::all(DANGER_BORDER),
                                    BackgroundColor(DANGER_NORMAL_BUTTON),
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

fn update_pause_focus_highlight_system(
    focus: Res<PauseMenuFocus>,
    mut menu_query: Query<
        (&PauseMenuButton, &mut BorderColor, &mut BackgroundColor),
        Without<PauseModalButton>,
    >,
    mut modal_query: Query<
        (&PauseModalButton, &mut BorderColor, &mut BackgroundColor),
        Without<PauseMenuButton>,
    >,
) {
    if focus.modal_open {
        for (modal_btn, mut border_color, mut bg_color) in &mut modal_query {
            let is_focused = focus.modal_focus == modal_btn.0;
            match modal_btn.0 {
                PauseModalFocusItem::Confirm => {
                    if is_focused {
                        *border_color = BorderColor::all(Color::WHITE);
                        *bg_color = BackgroundColor(DANGER_HOVERED_BUTTON);
                    } else {
                        *border_color = BorderColor::all(DANGER_BORDER);
                        *bg_color = BackgroundColor(DANGER_NORMAL_BUTTON);
                    }
                }
                PauseModalFocusItem::Cancel => {
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
        for (menu_btn, mut border_color, mut bg_color) in &mut menu_query {
            let is_focused = focus.current_item == menu_btn.0;
            let is_danger = matches!(
                menu_btn.0,
                PauseMenuItem::ReturnToTitle | PauseMenuItem::QuitToDesktop
            );

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
        }
    }
}

fn cleanup_pause_menu_ui(
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
