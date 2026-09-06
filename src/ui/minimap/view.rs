use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use super::MinimapState;
use crate::ui::theme::UiTheme;

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
pub const MINIMAP_BORDER_COLOR: Color = Color::srgb(0.20, 0.45, 0.60);
pub const MINIMAP_ACCENT_COLOR: Color = Color::srgb(0.25, 0.85, 0.75);

pub fn setup_minimap_ui(
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

    // 初期のプレースホルダー画像を作成（後ほど render::update_minimap_texture_system でマップデータから描画）
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
            GlobalZIndex(25),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(96.0),
                right: Val::Px(24.0),
                width: Val::Px(MINIMAP_WIDTH + 16.0),
                height: Val::Px(MINIMAP_HEIGHT + 44.0),
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
            Pickable::IGNORE,
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
                    Button,
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

pub fn cleanup_minimap_ui(mut commands: Commands, query: Query<Entity, With<MinimapRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
