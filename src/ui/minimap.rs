use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::PrimaryWindow;

use crate::camera::MapCamera;
use crate::faction::{FactionOutpost, PlayerFaction, TerritoryMap};
use crate::map::hex::{self, HexCoord, MAP_HEIGHT, MAP_WIDTH};
use crate::map::terrain::TerrainType;
use crate::map::{MapGrid, HEX_RADIUS};
use crate::state::AppState;
use crate::ui::theme::UiTheme;

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MinimapState>()
            .add_systems(OnEnter(AppState::InGame), setup_minimap_ui)
            .add_systems(
                Update,
                (
                    update_minimap_texture_system,
                    update_minimap_viewport_system,
                    handle_minimap_interaction_system,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(AppState::Title), cleanup_minimap_ui);
    }
}

/// ミニマップの表示・制御状態
#[derive(Resource, Default)]
pub struct MinimapState {
    pub texture_handle: Option<Handle<Image>>,
    pub is_dragging: bool,
    pub needs_update: bool,
}

#[derive(Component)]
pub struct MinimapRoot;

#[derive(Component)]
pub struct MinimapImageNode;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum MinimapCameraBoxPart {
    Primary,
    Secondary,
}

#[derive(Component)]
pub struct MinimapCoordText;

// ミニマップUIのサイズ定義
pub const MINIMAP_WIDTH: f32 = 224.0;
pub const MINIMAP_HEIGHT: f32 = 140.0;
const MINIMAP_BORDER_COLOR: Color = Color::srgb(0.20, 0.45, 0.60);
const MINIMAP_ACCENT_COLOR: Color = Color::srgb(0.25, 0.85, 0.75);

fn setup_minimap_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut minimap_state: ResMut<MinimapState>,
    map_config: Res<crate::map::settings::MapConfig>,
    existing: Query<Entity, With<MinimapRoot>>,
) {
    if !existing.is_empty() {
        return;
    }

    let font_regular = asset_server.load(UiTheme::fonts().regular());
    let font_bold = asset_server.load(UiTheme::fonts().bold());

    // 初期のプレースホルダー画像を作成（後ほど update_minimap_texture_system でマップデータから描画）
    let map_w = map_config.width();
    let map_h = map_config.height();
    let img_width = (map_w as u32) * 8;
    let img_height = (map_h as u32) * 8;
    let mut image = Image::new_fill(
        Extent3d {
            width: img_width,
            height: img_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[10, 20, 35, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.sampler = bevy::image::ImageSampler::nearest();
    let texture_handle = images.add(image);
    minimap_state.texture_handle = Some(texture_handle.clone());
    minimap_state.needs_update = true;

    // 右下に配置するミニマップコンテナ（ターン終了ボタンの上）
    commands
        .spawn((
            MinimapRoot,
            crate::ui::UiBlockMapInteraction,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(96.0),
                right: Val::Px(24.0),
                width: Val::Px(MINIMAP_WIDTH + 16.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.5)),
                border_radius: BorderRadius::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.08, 0.14, 0.94)),
            BorderColor::all(MINIMAP_BORDER_COLOR),
        ))
        .with_children(|container| {
            // ヘッダーバー（タイトル & 座標表示）
            container
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::axes(Val::Px(4.0), Val::Px(0.0)),
                    ..default()
                })
                .with_children(|header| {
                    header.spawn((
                        Text::new("PLANET RADAR // 全球観測"),
                        TextFont {
                            font: font_bold.clone().into(),
                            font_size: FontSize::Px(11.0),
                            ..default()
                        },
                        TextColor(MINIMAP_ACCENT_COLOR),
                    ));

                    header.spawn((
                        MinimapCoordText,
                        Text::new("POS [0, 0]"),
                        TextFont {
                            font: font_regular.clone().into(),
                            font_size: FontSize::Px(10.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.65, 0.75, 0.85)),
                    ));
                });

            // マップ描画フレーム（実際の画像 + カメラ視界枠）
            container
                .spawn((
                    MinimapImageNode,
                    Interaction::default(),
                    bevy::ui::RelativeCursorPosition::default(),
                    Node {
                        width: Val::Px(MINIMAP_WIDTH),
                        height: Val::Px(MINIMAP_HEIGHT),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(4.0)),
                        overflow: Overflow::clip(),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    ImageNode::new(texture_handle),
                    BorderColor::all(Color::srgba(0.25, 0.85, 0.75, 0.4)),
                ))
                .with_children(|viewport_parent| {
                    // 主カメラ現在視野枠（矩形）
                    viewport_parent.spawn((
                        MinimapCameraBoxPart::Primary,
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(MINIMAP_WIDTH * 0.5 - 20.0),
                            top: Val::Px(MINIMAP_HEIGHT * 0.5 - 15.0),
                            width: Val::Px(40.0),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(1.5)),
                            border_radius: BorderRadius::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(1.0, 0.90, 0.30)), // 視認性の高いイエロー/ゴールド枠
                        BackgroundColor(Color::srgba(1.0, 0.90, 0.30, 0.12)), // 薄い半透明塗り
                        Visibility::Visible,
                        Pickable::IGNORE,
                    ));

                    // 東西ラップ時の反対側カメラ視野枠（矩形）
                    viewport_parent.spawn((
                        MinimapCameraBoxPart::Secondary,
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            width: Val::Px(40.0),
                            height: Val::Px(30.0),
                            border: UiRect::all(Val::Px(1.5)),
                            border_radius: BorderRadius::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(1.0, 0.90, 0.30)),
                        BackgroundColor(Color::srgba(1.0, 0.90, 0.30, 0.12)),
                        Visibility::Hidden,
                        Pickable::IGNORE,
                    ));
                });
        });
}

fn cleanup_minimap_ui(mut commands: Commands, query: Query<Entity, With<MinimapRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

/// マップ生成完了後、または領土変更時にミニマップ用テクスチャをピクセル描画
fn update_minimap_texture_system(
    mut minimap_state: ResMut<MinimapState>,
    mut images: ResMut<Assets<Image>>,
    map_grid: Res<MapGrid>,
    territory_map: Res<TerritoryMap>,
    outposts_query: Query<&FactionOutpost>,
    player_faction: Res<PlayerFaction>,
) {
    if !minimap_state.needs_update && !territory_map.is_changed() {
        return;
    }

    if map_grid.terrain_data.is_empty() {
        return;
    }

    let Some(handle) = &minimap_state.texture_handle else {
        return;
    };
    let Some(mut image) = images.get_mut(handle) else {
        return;
    };

    let w = image.width() as usize;
    let h = image.height() as usize;
    let map_w = if map_grid.width > 0 { map_grid.width } else { MAP_WIDTH };
    let map_h = if map_grid.height > 0 { map_grid.height } else { MAP_HEIGHT };
    let half_h = map_h / 2;

    let mut pixels = vec![0u8; w * h * 4];

    // 各ピクセル (px, py) がどの HexCoord (col, row) に該当するかを計算して着色
    // ミニマップ上端 (py = 0) を画面上部（-Z, row = -half_h）、下端 (py = h - 1) を画面下部（+Z, row = +half_h）
    // 左端 (px = 0) を col = 0、右端 (px = w - 1) を col = map_w - 1
    for py in 0..h {
        let norm_y = py as f32 / h as f32;
        let row = -half_h + ((norm_y * (map_h as f32)) as i32).clamp(0, map_h - 1);

        for px in 0..w {
            let norm_x = px as f32 / w as f32;
            let col = ((norm_x * (map_w as f32)) as i32).clamp(0, map_w - 1);

            let coord = HexCoord::from_col_row_with_width(col, row, map_w);
            let terrain = map_grid.terrain_data.get(&coord).copied().unwrap_or(TerrainType::Ocean);

            // 地形の基本色 (RGBA)
            let base_c = terrain.base_color().to_srgba();
            let mut r = (base_c.red * 255.0) as u8;
            let mut g = (base_c.green * 255.0) as u8;
            let mut b = (base_c.blue * 255.0) as u8;
            let a = 255u8;

            // 領土オーバーレイ
            if let Some(owner) = territory_map.tile_owners.get(&coord) {
                let owner_color = owner.primary_color().to_srgba();
                let blend = 0.42; // 派閥カラーのブレンド率
                r = ((1.0 - blend) * (r as f32) + blend * owner_color.red * 255.0) as u8;
                g = ((1.0 - blend) * (g as f32) + blend * owner_color.green * 255.0) as u8;
                b = ((1.0 - blend) * (b as f32) + blend * owner_color.blue * 255.0) as u8;
            }

            let idx = (py * w + px) * 4;
            pixels[idx] = r;
            pixels[idx + 1] = g;
            pixels[idx + 2] = b;
            pixels[idx + 3] = a;
        }
    }

    // 各派閥の拠点をミニマップ上にハイライト描画 (3x3 ピクセルの輝点)
    for outpost in &outposts_query {
        let (col, row) = outpost.coord.to_col_row_with_width(map_w);
        let center_x = ((col as f32 + 0.5) / (map_w as f32) * (w as f32)) as i32;
        let center_y = (((row + half_h) as f32 + 0.5) / (map_h as f32) * (h as f32)) as i32;

        let is_player = outpost.faction == player_faction.0;
        let highlight_c = if is_player {
            [255, 255, 255, 255]
        } else {
            let col = outpost.faction.accent_color().to_srgba();
            [
                (col.red * 255.0) as u8,
                (col.green * 255.0) as u8,
                (col.blue * 255.0) as u8,
                255,
            ]
        };

        for dy in -1..=1 {
            for dx in -1..=1 {
                let px = (center_x + dx).rem_euclid(w as i32) as usize;
                let py = (center_y + dy).clamp(0, (h - 1) as i32) as usize;
                let idx = (py * w + px) * 4;
                pixels[idx] = highlight_c[0];
                pixels[idx + 1] = highlight_c[1];
                pixels[idx + 2] = highlight_c[2];
                pixels[idx + 3] = 255;
            }
        }
    }

    image.data = Some(pixels);
    minimap_state.needs_update = false;
}

/// カメラの現在位置とズーム率に合わせて、ミニマップ上の視野矩形を更新（東西ラップ時の入れ違い・反対側表示に対応）
fn update_minimap_viewport_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    map_camera_query: Query<&MapCamera>,
    map_grid: Res<MapGrid>,
    mut box_query: Query<(&MinimapCameraBoxPart, &mut Node, &mut Visibility)>,
    mut text_query: Query<&mut Text, With<MinimapCoordText>>,
) {
    let Ok(map_cam) = map_camera_query.single() else {
        return;
    };

    let map_w = if map_grid.width > 0 { map_grid.width } else { MAP_WIDTH };
    let map_h = if map_grid.height > 0 { map_grid.height } else { MAP_HEIGHT };

    let world_width = hex::map_world_width_with_width(HEX_RADIUS, map_w);
    let world_height = (map_h as f32) * 1.5 * HEX_RADIUS;

    // ワールド座標 X とミニマップ X の対応：
    // col = 0 がミニマップ左端 (norm_x = 0)、col = map_w が右端 (norm_x = 1)
    // カメラの current_focal_point.x は [-world_width/2, world_width/2] にあるため、
    // 正のワールド座標系 [0, world_width] に正規化してミニマップ上の割合を求める
    let norm_x = (map_cam.current_focal_point.x / world_width).rem_euclid(1.0);
    // ワールド座標 Z とミニマップ Y の対応：
    // ミニマップ上端 (top = 0) が row = -half_h (画面奥・上部: Z = -half_world_h)
    // ミニマップ下端 (top = MINIMAP_HEIGHT) が row = +half_h (画面手前・下部: Z = +half_world_h)
    let half_world_h = world_height / 2.0;
    let norm_y = if half_world_h > 0.0 {
        ((map_cam.current_focal_point.z + half_world_h) / world_height).clamp(0.0, 1.0)
    } else {
        0.5
    };

    // カメラの視野の幅と高さをミニマップ上のピクセルサイズに換算
    let aspect_ratio = if let Ok(win) = windows.single() {
        if win.height() > 0.0 {
            win.width() / win.height()
        } else {
            16.0 / 9.0
        }
    } else {
        16.0 / 9.0
    };

    let cam_viewport_h = map_cam.current_viewport_height;
    let cam_w = cam_viewport_h * aspect_ratio;
    // カメラの傾斜（Y=14, Z=12）を考慮して地表面上での実質的な視野長さを計算
    let sin_angle = 14.0 / (14.0_f32.powi(2) + 12.0_f32.powi(2)).sqrt();
    let cam_h = cam_viewport_h / sin_angle;

    // ミニマップ上のピクセルサイズ（極端な最小値による正方形化を防ぐため、アスペクト比を保持して最小サイズを制限）
    let raw_box_w = (cam_w / world_width) * MINIMAP_WIDTH;
    let raw_box_h = (cam_h / world_height) * MINIMAP_HEIGHT;
    let min_scale = if raw_box_w > 0.0 && raw_box_h > 0.0 {
        (16.0 / raw_box_w).max(10.0 / raw_box_h).max(1.0)
    } else {
        1.0
    };
    let box_w = (raw_box_w * min_scale).min(MINIMAP_WIDTH);
    let box_h = (raw_box_h * min_scale).min(MINIMAP_HEIGHT);

    let center_px_x = norm_x * MINIMAP_WIDTH;
    let center_px_y = norm_y * MINIMAP_HEIGHT;

    let primary_left = center_px_x - box_w * 0.5;
    let top = center_px_y - box_h * 0.5;

    // 端を超えているかどうかの判定:
    // primary_left < 0.0 の場合: 西端（左端）を超えているため、右端 (primary_left + MINIMAP_WIDTH) からも出現
    // primary_left + box_w > MINIMAP_WIDTH の場合: 東端（右端）を超えているため、左端 (primary_left - MINIMAP_WIDTH) からも出現
    let secondary_info = if primary_left < 0.0 {
        Some(primary_left + MINIMAP_WIDTH)
    } else if primary_left + box_w > MINIMAP_WIDTH {
        Some(primary_left - MINIMAP_WIDTH)
    } else {
        None
    };

    for (part, mut node, mut vis) in &mut box_query {
        match part {
            MinimapCameraBoxPart::Primary => {
                node.width = Val::Px(box_w);
                node.height = Val::Px(box_h);
                node.left = Val::Px(primary_left);
                node.top = Val::Px(top);
                *vis = Visibility::Visible;
            }
            MinimapCameraBoxPart::Secondary => {
                if let Some(sec_left) = secondary_info {
                    node.width = Val::Px(box_w);
                    node.height = Val::Px(box_h);
                    node.left = Val::Px(sec_left);
                    node.top = Val::Px(top);
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
                }
            }
        }
    }

    // 座標テキスト表示の更新
    if let Ok(mut text) = text_query.single_mut() {
        let current_hex = HexCoord::from_world_pos_with_width(map_cam.current_focal_point, HEX_RADIUS, map_w);
        let (c, r) = current_hex.to_col_row_with_width(map_w);
        **text = format!("COL {:02} ROW {:+02}", c, r);
    }
}

/// ミニマップ上でのクリック＆ドラッグによるカメラ位置の移動
fn handle_minimap_interaction_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut minimap_state: ResMut<MinimapState>,
    map_grid: Res<MapGrid>,
    image_query: Query<(
        &GlobalTransform,
        &Node,
        &Interaction,
        &bevy::ui::RelativeCursorPosition,
    ), With<MinimapImageNode>>,
    mut camera_query: Query<&mut MapCamera>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let Ok((gt, node, interaction, rel_cursor)) = image_query.single() else {
        return;
    };
    let Ok(mut map_cam) = camera_query.single_mut() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        minimap_state.is_dragging = false;
        return;
    };

    let translation = gt.translation();
    let size = match (node.width, node.height) {
        (Val::Px(w), Val::Px(h)) => Vec2::new(w, h),
        _ => Vec2::new(MINIMAP_WIDTH, MINIMAP_HEIGHT),
    };

    // ノードの左上と右下のスクリーン座標（Bevy UI GlobalTransformはノードの中心）
    let min = Vec2::new(translation.x - size.x * 0.5, translation.y - size.y * 0.5);

    // Bevy UIのInteractionシステムによる判定、またはマウス押下時の矩形内判定
    let is_inside = rel_cursor.normalized.is_some()
        || (cursor_pos.x >= min.x
            && cursor_pos.x <= min.x + size.x
            && cursor_pos.y >= min.y
            && cursor_pos.y <= min.y + size.y);

    if *interaction == Interaction::Pressed
        || (mouse_button.just_pressed(MouseButton::Left) && is_inside)
    {
        minimap_state.is_dragging = true;
    }

    if mouse_button.just_released(MouseButton::Left) {
        minimap_state.is_dragging = false;
    }

    if minimap_state.is_dragging && mouse_button.pressed(MouseButton::Left) {
        // RelativeCursorPosition::normalized (0.0..1.0) が取得できれば最優先で使用。
        // ドラッグで外側に少しはみ出た場合でも cursor_pos と min/size から正確にクランプ計算。
        let (norm_x, norm_y) = if let Some(normalized) = rel_cursor.normalized {
            (normalized.x.clamp(0.0, 1.0), normalized.y.clamp(0.0, 1.0))
        } else {
            let nx = ((cursor_pos.x - min.x) / size.x).clamp(0.0, 1.0);
            let ny = ((cursor_pos.y - min.y) / size.y).clamp(0.0, 1.0);
            (nx, ny)
        };

        let map_w = if map_grid.width > 0 { map_grid.width } else { MAP_WIDTH };
        let map_h = if map_grid.height > 0 { map_grid.height } else { MAP_HEIGHT };

        let world_width = hex::map_world_width_with_width(HEX_RADIUS, map_w);
        let world_height = (map_h as f32) * 1.5 * HEX_RADIUS;

        // ミニマップ左端 (norm_x = 0.0) が X = 0、右端 (norm_x = 1.0) が X = world_width
        // cameraのwrap範囲 [-world_width/2, world_width/2] に合わせる
        let mut world_x = norm_x * world_width;
        if world_x > world_width / 2.0 {
            world_x -= world_width;
        }
        // 上端 (norm_y = 0.0) が -half_world_h (-Z)、下端 (norm_y = 1.0) が +half_world_h (+Z)
        let world_z = (norm_y - 0.5) * world_height;

        let target = Vec3::new(world_x, 0.0, world_z);
        map_cam.target_focal_point = target;
        map_cam.current_focal_point = target;
    }
}
