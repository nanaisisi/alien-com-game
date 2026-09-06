use bevy::prelude::*;

use crate::faction::{FactionId, FactionManager, PlayerFaction};
use crate::state::AppState;

pub struct InGameHudPlugin;

impl Plugin for InGameHudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FactionResources>()
            .add_systems(OnEnter(AppState::InGame), setup_hud)
            .add_systems(
                Update,
                (
                    hud_button_interaction_system,
                    hud_button_action_system,
                    update_hud_display_system,
                    handle_keyboard_shortcuts,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_hud);
    }
}

/// プレイヤー勢力の資源とターン管理
#[derive(Resource, Debug, Clone)]
pub struct FactionResources {
    pub turn: u32,
    pub energy: i32,
    pub energy_per_turn: i32,
    pub production: i32,
    pub production_per_turn: i32,
    pub science: i32,
    pub science_per_turn: i32,
    pub food: i32,
    pub food_per_turn: i32,
}

impl Default for FactionResources {
    fn default() -> Self {
        Self {
            turn: 1,
            energy: 120,
            energy_per_turn: 15,
            production: 50,
            production_per_turn: 12,
            science: 30,
            science_per_turn: 8,
            food: 80,
            food_per_turn: 10,
        }
    }
}

#[derive(Component)]
struct HudRoot;

#[derive(Component, Debug, Clone, Copy)]
enum HudAction {
    EndTurn,
    OpenDiplomacy,
    OpenMenu,
}

#[derive(Component)]
enum HudLabel {
    Turn,
    Energy,
    Production,
    Science,
    Food,
}

use crate::ui::theme::UiTheme;

const END_TURN_NORMAL: Color = Color::srgb(0.10, 0.30, 0.35);
const END_TURN_HOVER: Color = Color::srgb(0.16, 0.48, 0.55);
const END_TURN_PRESSED: Color = Color::srgb(0.25, 0.75, 0.80);

fn setup_hud(
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

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

type ButtonInteractionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static mut BackgroundColor, &'static mut BorderColor, &'static HudAction),
    (Changed<Interaction>, With<Button>),
>;

fn hud_button_interaction_system(mut query: ButtonInteractionQuery) {
    let surfaces = UiTheme::surfaces();
    let standard_btn = UiTheme::button(false);

    for (interaction, mut bg_color, mut border_color, action) in &mut query {
        match action {
            HudAction::EndTurn => match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(END_TURN_PRESSED);
                    *border_color = BorderColor::all(Color::WHITE);
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(END_TURN_HOVER);
                    *border_color = BorderColor::all(Color::srgb(0.5, 0.95, 0.9));
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(END_TURN_NORMAL);
                    *border_color = BorderColor::all(surfaces.accent());
                }
            },
            HudAction::OpenDiplomacy => match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(standard_btn.pressed());
                    *border_color = BorderColor::all(Color::WHITE);
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(Color::srgb(0.16, 0.35, 0.44));
                    *border_color = BorderColor::all(Color::WHITE);
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(Color::srgba(0.10, 0.25, 0.32, 0.85));
                    *border_color = BorderColor::all(surfaces.accent());
                }
            },
            HudAction::OpenMenu => match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(standard_btn.pressed());
                    *border_color = BorderColor::all(surfaces.accent());
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(standard_btn.hovered());
                    *border_color = BorderColor::all(surfaces.accent());
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(standard_btn.normal());
                    *border_color = BorderColor::all(surfaces.border());
                }
            },
        }
    }
}

type ButtonActionQuery<'world, 'state> = Query<
    'world,
    'state,
    (&'static Interaction, &'static HudAction),
    (Changed<Interaction>, With<Button>),
>;

#[allow(clippy::too_many_arguments)]
fn hud_button_action_system(
    query: ButtonActionQuery,
    mut resources: ResMut<FactionResources>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_faction: Res<PlayerFaction>,
    faction_mgr: Res<FactionManager>,
    mut modal_state: ResMut<crate::ui::diplomacy::DiplomacyModalState>,
) {
    for (interaction, action) in &query {
        if *interaction == Interaction::Pressed {
            match action {
                HudAction::EndTurn => {
                    advance_turn(&mut resources);
                }
                HudAction::OpenDiplomacy => {
                    modal_state.is_open = !modal_state.is_open;
                    if modal_state.is_open {
                        if modal_state.selected_target.is_none() {
                            modal_state.selected_target = FactionId::ALL
                                .iter()
                                .copied()
                                .find(|&f| f != player_faction.0);
                        }
                        crate::ui::diplomacy::spawn_diplomacy_modal(
                            &mut commands,
                            &asset_server,
                            player_faction.0,
                            &modal_state,
                            &faction_mgr,
                        );
                    }
                }
                HudAction::OpenMenu => {
                    info!("Opening Pause Menu...");
                    settings.return_state = AppState::InGame;
                    next_state.set(AppState::PauseMenu);
                }
            }
        }
    }
}

fn handle_keyboard_shortcuts(
    keys: Res<ButtonInput<KeyCode>>,
    mut resources: ResMut<FactionResources>,
    mut settings: ResMut<crate::ui::settings::GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        advance_turn(&mut resources);
    }
    if keys.just_pressed(KeyCode::Escape) {
        info!("ESC pressed: Opening Pause Menu...");
        settings.return_state = AppState::InGame;
        next_state.set(AppState::PauseMenu);
    }
}

fn advance_turn(resources: &mut FactionResources) {
    resources.turn += 1;
    resources.energy += resources.energy_per_turn;
    resources.production += resources.production_per_turn;
    resources.science += resources.science_per_turn;
    resources.food += resources.food_per_turn;
    info!("Advancing to Turn {}", resources.turn);
}

fn update_hud_display_system(
    resources: Res<FactionResources>,
    mut query: Query<(&mut Text, &HudLabel)>,
) {
    if !resources.is_changed() {
        return;
    }

    for (mut text, label) in &mut query {
        match label {
            HudLabel::Turn => {
                **text = format!("TURN {:02}", resources.turn);
            }
            HudLabel::Energy => {
                **text = format!("{} (+{})", resources.energy, resources.energy_per_turn);
            }
            HudLabel::Production => {
                **text = format!(
                    "{} (+{})",
                    resources.production, resources.production_per_turn
                );
            }
            HudLabel::Science => {
                **text = format!("{} (+{})", resources.science, resources.science_per_turn);
            }
            HudLabel::Food => {
                **text = format!("{} (+{})", resources.food, resources.food_per_turn);
            }
        }
    }
}
