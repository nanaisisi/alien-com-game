use bevy::app::AppExit;
use bevy::prelude::*;

use crate::state::AppState;

pub struct TitleUiPlugin;

impl Plugin for TitleUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TitleMenuFocus>()
            .add_systems(OnEnter(AppState::Title), (setup_title_ui, reset_title_focus))
            .add_systems(
                Update,
                (
                    title_keyboard_navigation_system,
                    button_interaction_system,
                    button_action_system,
                )
                    .run_if(in_state(AppState::Title)),
            )
            .add_systems(OnExit(AppState::Title), cleanup_title_ui);
    }
}

#[derive(Component)]
struct TitleRootUi;

#[derive(Resource, Default)]
pub struct TitleMenuFocus {
    pub selected_index: Option<usize>,
}

fn reset_title_focus(mut focus: ResMut<TitleMenuFocus>) {
    focus.selected_index = Some(0); // デフォルトで一番上（NEW GAME）にフォーカス
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuButtonAction {
    NewGame,
    LoadGame,
    Settings,
    Exit,
}

const MENU_ACTIONS: [MenuButtonAction; 4] = [
    MenuButtonAction::NewGame,
    MenuButtonAction::LoadGame,
    MenuButtonAction::Settings,
    MenuButtonAction::Exit,
];

#[derive(Component)]
struct TitleMenuButton(usize);

use super::theme::{
    self, BUTTON_HOVERED as HOVERED_BUTTON, BUTTON_NORMAL as NORMAL_BUTTON,
    BUTTON_PRESSED as PRESSED_BUTTON, ACCENT_COLOR, TEXT_MAIN as TEXT_COLOR,
};

fn setup_title_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_regular = asset_server.load(theme::FONT_REGULAR);
    let font_bold = asset_server.load(theme::FONT_BOLD);

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
                        TextColor(ACCENT_COLOR),
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
                            ACCENT_COLOR
                        } else {
                            Color::srgb(0.25, 0.38, 0.50)
                        }),
                        BackgroundColor(if idx == 0 {
                            HOVERED_BUTTON
                        } else {
                            NORMAL_BUTTON
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
                            TextColor(TEXT_COLOR),
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

fn title_keyboard_navigation_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut focus: ResMut<TitleMenuFocus>,
    mut next_state: ResMut<NextState<AppState>>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut exit_events: bevy::ecs::message::MessageWriter<AppExit>,
) {
    let count = MENU_ACTIONS.len();
    let current = focus.selected_index.unwrap_or(0);

    if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        focus.selected_index = Some(if current == 0 { count - 1 } else { current - 1 });
    } else if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
        focus.selected_index = Some((current + 1) % count);
    }

    if (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space))
        && let Some(selected) = focus.selected_index
            && selected < count {
                execute_title_action(
                    MENU_ACTIONS[selected],
                    &mut settings,
                    &mut next_state,
                    &mut exit_events,
                );
            }
}

type TitleButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static mut BorderColor,
        &'static TitleMenuButton,
    ),
    With<Button>,
>;

type TitleButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static MenuButtonAction),
    (Changed<Interaction>, With<Button>),
>;

fn button_interaction_system(
    mut interaction_query: TitleButtonInteractionQuery,
    mut focus: ResMut<TitleMenuFocus>,
) {
    // マウスホバーされた場合はフォーカスインデックスを更新
    for (interaction, _, _, menu_btn) in &interaction_query {
        if *interaction == Interaction::Hovered || *interaction == Interaction::Pressed {
            focus.selected_index = Some(menu_btn.0);
        }
    }

    let current_selected = focus.selected_index;

    // スタイルの同期
    for (interaction, mut bg_color, mut border_color, menu_btn) in &mut interaction_query {
        let is_selected = current_selected == Some(menu_btn.0);

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
                if is_selected {
                    *bg_color = BackgroundColor(HOVERED_BUTTON);
                    *border_color = BorderColor::all(ACCENT_COLOR);
                } else {
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                    *border_color = BorderColor::all(Color::srgb(0.25, 0.38, 0.50));
                }
            }
        }
    }
}

fn execute_title_action(
    action: MenuButtonAction,
    settings: &mut ResMut<crate::ui::settings::GameSettings>,
    next_state: &mut ResMut<NextState<AppState>>,
    exit_events: &mut bevy::ecs::message::MessageWriter<AppExit>,
) {
    match action {
        MenuButtonAction::NewGame => {
            info!("Transitioning to Faction Selection...");
            next_state.set(AppState::FactionSelect);
        }
        MenuButtonAction::LoadGame => {
            info!("Load Game clicked (WIP)");
        }
        MenuButtonAction::Settings => {
            info!("Transitioning to Settings...");
            settings.return_state = AppState::Title;
            next_state.set(AppState::Settings);
        }
        MenuButtonAction::Exit => {
            info!("Exiting Game...");
            exit_events.write(AppExit::Success);
        }
    }
}

fn button_action_system(
    interaction_query: TitleButtonActionQuery,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_events: bevy::ecs::message::MessageWriter<AppExit>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            execute_title_action(*action, &mut settings, &mut next_state, &mut exit_events);
        }
    }
}

fn cleanup_title_ui(mut commands: Commands, query: Query<Entity, With<TitleRootUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

