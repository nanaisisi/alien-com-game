use bevy::app::AppExit;
use bevy::prelude::*;

use crate::state::AppState;

pub struct TitleUiPlugin;

impl Plugin for TitleUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Title), setup_title_ui)
            .add_systems(
                Update,
                (button_interaction_system, button_action_system)
                    .run_if(in_state(AppState::Title)),
            )
            .add_systems(OnExit(AppState::Title), cleanup_title_ui);
    }
}

#[derive(Component)]
struct TitleRootUi;

#[derive(Component, Debug, Clone, Copy)]
enum MenuButtonAction {
    NewGame,
    LoadGame,
    Settings,
    Exit,
}

// 配色テーマ: SF感のあるダークサイバー / ディープスペース調
const NORMAL_BUTTON: Color = Color::srgb(0.12, 0.16, 0.22);
const HOVERED_BUTTON: Color = Color::srgb(0.20, 0.32, 0.45);
const PRESSED_BUTTON: Color = Color::srgb(0.15, 0.50, 0.65);
const TEXT_COLOR: Color = Color::srgb(0.92, 0.95, 0.98);
const ACCENT_COLOR: Color = Color::srgb(0.25, 0.85, 0.75);

fn setup_title_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_regular = asset_server.load("fonts/UDEVGothicNF-Regular.ttf");
    let font_bold = asset_server.load("fonts/UDEVGothicNF-Bold.ttf");

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

            for (label, action) in menu_items {
                parent
                    .spawn((
                        Button,
                        action,
                        Node {
                            width: Val::Px(280.0),
                            height: Val::Px(52.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            border_radius: BorderRadius::all(Val::Px(8.0)),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.25, 0.38, 0.50)),
                        BackgroundColor(NORMAL_BUTTON),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new(label),
                            TextFont {
                                font: font_regular.clone().into(),
                                font_size: FontSize::Px(18.0),
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ));
                    });
            }

            // フッターバージョン情報
            parent.spawn((
                Text::new("v0.1.0-alpha | 4X SF Tactical Strategy"),
                TextFont {
                    font: font_regular.clone().into(),
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.40, 0.48, 0.55)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

type TitleButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static mut BackgroundColor, &'static mut BorderColor),
    (Changed<Interaction>, With<Button>),
>;

type TitleButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static MenuButtonAction),
    (Changed<Interaction>, With<Button>),
>;

fn button_interaction_system(
    mut interaction_query: TitleButtonInteractionQuery,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
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
                *border_color = BorderColor::all(Color::srgb(0.25, 0.38, 0.50));
            }
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
            match action {
                MenuButtonAction::NewGame => {
                    info!("Starting New Game...");
                    next_state.set(AppState::InGame);
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
    }
}

fn cleanup_title_ui(mut commands: Commands, query: Query<Entity, With<TitleRootUi>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
