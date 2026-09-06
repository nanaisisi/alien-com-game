use bevy::prelude::*;

use crate::faction::types::FactionId;
use super::types::CombatGroupType;

/// 戦闘団種別と派閥に応じたSFビジュアル（子エンティティ階層）をスポーンする
pub fn spawn_unit_model(
    commands: &mut Commands,
    parent: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    faction: FactionId,
    group_type: CombatGroupType,
) {
    let p_col = faction.primary_color().to_srgba();
    let a_col = faction.accent_color().to_srgba();

    // 派閥の Primary Color マテリアル（機体装甲）
    let primary_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(p_col.red, p_col.green, p_col.blue, 1.0),
        metallic: 0.6,
        perceptual_roughness: 0.35,
        ..default()
    });

    // 派閥の Accent Color マテリアル（装飾・バイザー・発光部）
    let accent_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(a_col.red, a_col.green, a_col.blue, 1.0),
        emissive: LinearRgba::rgb(a_col.red * 1.5, a_col.green * 1.5, a_col.blue * 1.5),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });

    // 暗色フレーム・シャーシマテリアル
    let frame_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.14, 0.16),
        metallic: 0.7,
        perceptual_roughness: 0.4,
        ..default()
    });

    match group_type {
        CombatGroupType::Scout => {
            // --- 偵察戦闘団: 鋭角なホバードローン / 高速スピーダー ---
            // 浮遊感のある中央メインボディ
            let body_mesh = meshes.add(Cuboid::new(0.35, 0.12, 0.5));
            let wing_mesh = meshes.add(Cuboid::new(0.65, 0.04, 0.22));
            let sensor_mesh = meshes.add(Sphere::new(0.09));
            let thruster_mesh = meshes.add(Cylinder::new(0.06, 0.14));

            commands.entity(parent).with_children(|builder| {
                // メインボディ
                builder.spawn((
                    Mesh3d(body_mesh),
                    MeshMaterial3d(primary_mat.clone()),
                    Transform::from_xyz(0.0, 0.28, 0.0),
                ));
                // 翼部
                builder.spawn((
                    Mesh3d(wing_mesh),
                    MeshMaterial3d(frame_mat.clone()),
                    Transform::from_xyz(0.0, 0.26, -0.05),
                ));
                // 先端センサードーム（発光Accent）
                builder.spawn((
                    Mesh3d(sensor_mesh),
                    MeshMaterial3d(accent_mat.clone()),
                    Transform::from_xyz(0.0, 0.30, 0.22),
                ));
                // 後部ツインスラスター
                builder.spawn((
                    Mesh3d(thruster_mesh.clone()),
                    MeshMaterial3d(accent_mat.clone()),
                    Transform::from_xyz(-0.12, 0.28, -0.28)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ));
                builder.spawn((
                    Mesh3d(thruster_mesh),
                    MeshMaterial3d(accent_mat.clone()),
                    Transform::from_xyz(0.12, 0.28, -0.28)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ));
            });
        }
        CombatGroupType::Colonist => {
            // --- 開拓戦闘団: 重装甲モバイルハブ / 探査クローラー ---
            let base_mesh = meshes.add(Cuboid::new(0.5, 0.15, 0.65));
            let dome_mesh = meshes.add(Sphere::new(0.22));
            let wheel_mesh = meshes.add(Cylinder::new(0.10, 0.12));
            let antenna_mesh = meshes.add(Cylinder::new(0.02, 0.35));

            commands.entity(parent).with_children(|builder| {
                // シャーシベース
                builder.spawn((
                    Mesh3d(base_mesh),
                    MeshMaterial3d(primary_mat.clone()),
                    Transform::from_xyz(0.0, 0.16, 0.0),
                ));
                // 中央の居住・観測ドーム
                builder.spawn((
                    Mesh3d(dome_mesh),
                    MeshMaterial3d(accent_mat.clone()),
                    Transform::from_xyz(0.0, 0.28, -0.05),
                ));
                // 4隅の大型重機ホイール
                for (wx, wz) in [(-0.28, 0.22), (0.28, 0.22), (-0.28, -0.22), (0.28, -0.22)] {
                    builder.spawn((
                        Mesh3d(wheel_mesh.clone()),
                        MeshMaterial3d(frame_mat.clone()),
                        Transform::from_xyz(wx, 0.10, wz)
                            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ));
                }
                // 通信アンテナ
                builder.spawn((
                    Mesh3d(antenna_mesh),
                    MeshMaterial3d(frame_mat.clone()),
                    Transform::from_xyz(0.18, 0.40, -0.22),
                ));
            });
        }
        CombatGroupType::LightInfantry => {
            // --- 軽歩兵戦闘団: 戦闘ウォーカー / 歩行メック ---
            let torso_mesh = meshes.add(Cuboid::new(0.32, 0.26, 0.30));
            let visor_mesh = meshes.add(Cuboid::new(0.22, 0.06, 0.08));
            let gun_mesh = meshes.add(Cylinder::new(0.04, 0.32));
            let leg_mesh = meshes.add(Cuboid::new(0.09, 0.24, 0.10));

            commands.entity(parent).with_children(|builder| {
                // メインボディ
                builder.spawn((
                    Mesh3d(torso_mesh),
                    MeshMaterial3d(primary_mat.clone()),
                    Transform::from_xyz(0.0, 0.34, 0.0),
                ));
                // コックピットバイザー（発光Accent）
                builder.spawn((
                    Mesh3d(visor_mesh),
                    MeshMaterial3d(accent_mat.clone()),
                    Transform::from_xyz(0.0, 0.38, 0.16),
                ));
                // 右腕ガトリング砲
                builder.spawn((
                    Mesh3d(gun_mesh),
                    MeshMaterial3d(frame_mat.clone()),
                    Transform::from_xyz(0.23, 0.30, 0.10)
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ));
                // 両脚部
                builder.spawn((
                    Mesh3d(leg_mesh.clone()),
                    MeshMaterial3d(frame_mat.clone()),
                    Transform::from_xyz(-0.12, 0.12, 0.0),
                ));
                builder.spawn((
                    Mesh3d(leg_mesh),
                    MeshMaterial3d(frame_mat.clone()),
                    Transform::from_xyz(0.12, 0.12, 0.0),
                ));
            });
        }
    }
}
