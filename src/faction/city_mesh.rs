use bevy::prelude::*;

use crate::faction::types::FactionId;

/// 都市3Dモデルの子エンティティ群の親タグ
#[derive(Component, Debug, Clone, Copy)]
pub struct CityVisualModel;

/// 派閥ごとの都市・前哨基地のSFビジュアルをスポーンする
pub fn spawn_city_model(
    commands: &mut Commands,
    parent: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    faction: FactionId,
    level: u32,
) {
    let p_col = faction.primary_color().to_srgba();
    let a_col = faction.accent_color().to_srgba();

    // 派閥の装甲マテリアル
    let primary_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(p_col.red, p_col.green, p_col.blue, 1.0),
        metallic: 0.7,
        perceptual_roughness: 0.3,
        ..default()
    });

    // 派閥の発光・バイザーマテリアル（夜間・遠方でも目立つ）
    let accent_emissive_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(a_col.red, a_col.green, a_col.blue, 1.0),
        emissive: LinearRgba::rgb(a_col.red * 2.0, a_col.green * 2.0, a_col.blue * 2.0),
        metallic: 0.9,
        perceptual_roughness: 0.15,
        ..default()
    });

    // 暗色フレーム・基礎構造物
    let frame_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.17, 0.22),
        metallic: 0.8,
        perceptual_roughness: 0.4,
        ..default()
    });

    // 居住ドーム用ガラス・透過マテリアル
    let dome_glass_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.5, 0.85, 0.95, 0.55),
        alpha_mode: AlphaMode::Blend,
        metallic: 0.3,
        perceptual_roughness: 0.1,
        ..default()
    });

    // メッシュリソース
    let base_plate_mesh = meshes.add(Cylinder::new(0.65, 0.08));
    let main_tower_mesh = meshes.add(Cuboid::new(0.24, 0.70 + (level as f32 * 0.15), 0.24));
    let tower_crown_mesh = meshes.add(Cone::new(0.18, 0.25));
    let dome_mesh = meshes.add(Sphere::new(0.22));
    let small_hab_mesh = meshes.add(Cuboid::new(0.16, 0.14, 0.22));
    let antenna_rod = meshes.add(Cylinder::new(0.015, 0.45));
    let comm_dish_mesh = meshes.add(Cone::new(0.09, 0.06));

    commands.entity(parent).with_children(|builder| {
        builder.spawn(CityVisualModel);

        // 1. 基盤プラットフォーム（円形基礎）
        builder.spawn((
            Mesh3d(base_plate_mesh),
            MeshMaterial3d(frame_mat.clone()),
            Transform::from_xyz(0.0, 0.04, 0.0),
        ));

        // 2. 中央司令タワー
        let tower_height = 0.70 + (level as f32 * 0.15);
        builder.spawn((
            Mesh3d(main_tower_mesh),
            MeshMaterial3d(primary_mat.clone()),
            Transform::from_xyz(0.0, 0.08 + tower_height / 2.0, 0.0),
        ));

        // 3. タワー尖塔クラウン（発光アクセント）
        builder.spawn((
            Mesh3d(tower_crown_mesh),
            MeshMaterial3d(accent_emissive_mat.clone()),
            Transform::from_xyz(0.0, 0.08 + tower_height + 0.12, 0.0),
        ));

        // 4. 周囲の居住ドーム（半透明ガラス）
        builder.spawn((
            Mesh3d(dome_mesh),
            MeshMaterial3d(dome_glass_mat),
            Transform::from_xyz(0.28, 0.12, 0.20),
        ));

        // 5. 副居住ハブ・工場ブロック（北西側）
        builder.spawn((
            Mesh3d(small_hab_mesh),
            MeshMaterial3d(frame_mat.clone()),
            Transform::from_xyz(-0.25, 0.12, -0.20),
        ));

        // 6. 通信アンテナ塔（南東側）
        builder.spawn((
            Mesh3d(antenna_rod),
            MeshMaterial3d(accent_emissive_mat.clone()),
            Transform::from_xyz(-0.25, 0.28, 0.22),
        ));

        // 7. パラボラ通信ディッシュ
        builder.spawn((
            Mesh3d(comm_dish_mesh),
            MeshMaterial3d(primary_mat),
            Transform::from_xyz(-0.25, 0.50, 0.22)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_4)),
        ));

        // 8. 都市上空のステータスバナー（夜間・俯瞰視点でも一目で自国・他国都市を視認できる）
        builder.spawn((
            Text::new(format!("⬢ 国{}【{}】 Lv.{}", faction.code(), faction.name_ja(), level)),
            TextFont {
                font_size: FontSize::Px(13.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            Transform::from_xyz(0.0, tower_height + 0.65, 0.0)
                .with_scale(Vec3::splat(0.016)),
        ));
    });
}

