use bevy::prelude::*;

use crate::faction::TerritoryMap;
use crate::map::hex::HexCoord;
use crate::map::interaction::SelectedTile;
use crate::map::MapGrid;
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
                width: Val::Px(320.0),
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

pub fn cleanup_info_panel_ui(
    mut commands: Commands,
    query: Query<Entity, With<InfoPanelRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// 選択中タイルの情報を左下HUDに反映
pub fn update_info_panel_system(
    selected: Res<SelectedTile>,
    map_grid: Res<MapGrid>,
    territory_map: Res<TerritoryMap>,
    mut query: Query<&mut Text, With<InfoPanelText>>,
) {
    if !selected.is_changed() {
        return;
    }

    let Ok(mut text) = query.single_mut() else {
        return;
    };

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

            let info = format!(
                "【ヘックス座標】\n  col: {}, row: {} (q: {}, r: {})\n  中心からの距離: {}\n\n\
                 【領有勢力】\n  {}\n\n\
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
