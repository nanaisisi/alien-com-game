use bevy::prelude::*;

use crate::faction::{FactionOutpost, TerritoryMap};
use crate::map::MapGrid;
use crate::map::hex::HexCoord;
use crate::map::interaction::SelectedTile;
use crate::state::AppState;
use crate::ui::theme::UiTheme;

pub struct InfoPanelPlugin;

impl Plugin for InfoPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_info_panel_ui)
            .add_systems(
                Update,
                update_info_panel_system.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_info_panel_ui);
    }
}

/// タイル情報UI表示用タグ
#[derive(Component)]
pub struct InfoPanelRoot;

#[derive(Component)]
pub struct InfoPanelText;

pub fn setup_info_panel_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing: Query<Entity, With<InfoPanelRoot>>,
) {
    if !existing.is_empty() {
        return;
    }

    let font = asset_server.load(UiTheme::fonts().regular());
    let font_bold = asset_server.load(UiTheme::fonts().bold());
    let surfaces = UiTheme::surfaces();

    // 左下に配置する半透明情報パネル
    commands
        .spawn((
            InfoPanelRoot,
            crate::ui::UiBlockMapInteraction,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(24.0),
                bottom: Val::Px(24.0),
                width: Val::Px(350.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(surfaces.panel()),
            BorderColor::all(surfaces.accent()),
        ))
        .with_children(|parent| {
            // パネル見出し
            parent.spawn((
                Text::new("PLANET SURVEYOR // 地形探査"),
                TextFont {
                    font: font_bold.clone().into(),
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(surfaces.accent()),
            ));

            // 詳細テキスト
            parent.spawn((
                InfoPanelText,
                Text::new("タイルをクリックして詳細情報を表示"),
                TextFont {
                    font: font.clone().into(),
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.90, 0.95, 0.98)),
            ));
        });
}

pub fn cleanup_info_panel_ui(mut commands: Commands, query: Query<Entity, With<InfoPanelRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

use crate::unit::{MoveModeState, SelectedUnit, Unit};

/// 選択中タイルの情報を左下HUDに反映
#[allow(clippy::too_many_arguments)]
pub fn update_info_panel_system(
    selected: Res<SelectedTile>,
    selected_unit: Res<SelectedUnit>,
    move_mode: Res<MoveModeState>,
    units_query: Query<&Unit>,
    map_grid: Res<MapGrid>,
    territory_map: Res<TerritoryMap>,
    outposts_query: Query<&FactionOutpost>,
    mut query: Query<&mut Text, With<InfoPanelText>>,
) {
    if !selected.is_changed() && !selected_unit.is_changed() && !move_mode.is_changed() {
        return;
    }

    let Ok(mut text) = query.single_mut() else {
        return;
    };

    // 選択中ユニットの情報を最優先表示
    if let Some(unit_e) = selected_unit.0
        && let Ok(unit) = units_query.get(unit_e)
    {
        let fac = unit.faction;
        let coord = unit.coord;
        let map_w = if map_grid.width > 0 {
            map_grid.width
        } else {
            crate::map::GRID_WIDTH
        };
        let (col, row) = coord.to_col_row_with_width(map_w);

        let mode_guide = if move_mode.0 {
            "【移動指示モード中 (MOVE)】\n  ※移動先タイルをクリックで移動実行\n  ※[M] / [ESC] / 右クリックで解除"
        } else {
            "【ユニット操作コマンド】\n  \
             [M]: 移動モード (クリックで移動)\n  \
             ※自部隊を右ドラッグして離すことでも移動可能\n  \
             [A]: 突撃/近接攻撃 (隣接敵・反撃あり)\n  \
             [R]: 遠隔射撃 (射程2マス・反撃なし)\n  \
             [F]: 防御態勢 (Fortify / 被ダメ軽減)\n  \
             [O]: 警戒監視 (Alert / 敵接近で覚醒)\n  \
             [H]: 回復・修理 (Heal / 全快まで休眠)\n  \
             [Z]: 休眠待機 (Sleep / 巡回スキップ)\n  \
             [T]: 装備・物資移転 (隣接友軍/拠点補給)\n  \
             [Space]: 待機・スキップ (Wait)\n  \
             [Delete]: 部隊解体 (資源一部回収)\n  \
             [Tab] / [.]: 次の部隊 ([,] 前の部隊)\n  \
             [Home] / [\\]: 首都へジャンプ\n  \
             [ESC]: 選択解除"
        };

        let info = format!(
            "【部隊選択中】\n  {}\n  所属: 国{}【{}】\n\n\
             【ステータス】\n  HP: {} / {}\n  残り移動力: {} / {}\n  攻撃力: {} (射程: {}マス)\n  態勢/状態: {}\n  行動可否: {}\n\n\
             【現在位置】\n  col: {}, row: {} (q: {}, r: {})\n\n\
             {}\n\n\
             ※移動: 右クリックドラッグ（ドラッグ中[M]で移動モードへ移行可）、または [M] キー移動モード",
            unit.group_type.display_name(),
            fac.code(),
            fac.name_ja(),
            unit.hp,
            unit.max_hp,
            unit.current_movement,
            unit.max_movement,
            unit.group_type.attack_power(),
            unit.group_type.attack_range(),
            unit.action_state.display_name(),
            if unit.is_exhausted {
                "行動終了 (待機中)"
            } else {
                "行動可能"
            },
            col,
            row,
            coord.q,
            coord.r,
            mode_guide
        );
        **text = info;
        return;
    }

    if let Some(coord) = selected.0 {
        if let Some(&terrain) = map_grid.terrain_data.get(&coord) {
            let passable_str = if terrain.is_passable_ground() {
                "通行可能"
            } else {
                "通行不能 (特殊部隊/航空機のみ)"
            };

            let move_cost_str = if terrain.movement_cost() > 100.0 {
                "N/A".to_string()
            } else {
                format!("{:.1}", terrain.movement_cost())
            };

            let map_w = if map_grid.width > 0 {
                map_grid.width
            } else {
                crate::map::GRID_WIDTH
            };
            let (col, row) = coord.to_col_row_with_width(map_w);
            let center_coord = HexCoord::from_col_row_with_width(map_w / 2, 0, map_w);
            let dist_from_center = coord.distance_with_width(center_coord, map_w);

            let owner_str = if let Some(owner) = territory_map.get_owner(&coord) {
                format!(
                    "国{}【{}】({})",
                    owner.code(),
                    owner.name_ja(),
                    owner.formal_title()
                )
            } else {
                "未領有・未開拓域 (Neutral Territory)".to_string()
            };

            let outpost_str = if let Some(outpost) =
                outposts_query.iter().find(|o| o.coord == coord)
            {
                if Some(outpost.faction) == territory_map.get_owner(&coord) {
                    format!(
                        "\n\n【都市・前哨基地】\n  {} (Lv.{})\n  ※[C]キーで都市管理・部隊生産画面を開く",
                        outpost.name, outpost.level
                    )
                } else {
                    format!(
                        "\n\n【都市・前哨基地】\n  {} (Lv.{})",
                        outpost.name, outpost.level
                    )
                }
            } else {
                String::new()
            };

            // タイル上の駐留部隊
            let stationed_unit = units_query.iter().find(|u| u.coord == coord);
            let unit_str = if let Some(u) = stationed_unit {
                format!(
                    "\n\n【駐留部隊】\n  {} (国{}) - 残移動力: {}/{}",
                    u.group_type.display_name(),
                    u.faction.code(),
                    u.current_movement,
                    u.max_movement
                )
            } else {
                String::new()
            };

            let info = format!(
                "【ヘックス座標】\n  col: {}, row: {} (q: {}, r: {})\n  中心からの距離: {}\n\n\
                 【領有勢力】\n  {}{}{}\n\n\
                 【地形種別】\n  {}\n\n\
                 【移動コスト】: {}\n\
                 【地上移動】: {}\n\n\
                 【環境特性】\n  エイリアン活動兆候あり\n  資源探査可能",
                col,
                row,
                coord.q,
                coord.r,
                dist_from_center,
                owner_str,
                outpost_str,
                unit_str,
                terrain.name(),
                move_cost_str,
                passable_str
            );
            **text = info;
        }
    } else {
        **text = "タイルをクリックして詳細情報を表示".to_string();
    }
}
