use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world_environment);
    }
}

/// メイン指向性ライトのマーカーコンポーネント
#[derive(Component)]
pub struct MainDirectionalLight;

/// 3Dワールドの環境光および太陽光（指向性ライト）の初期化
fn setup_world_environment(mut commands: Commands) {
    // メイン指向性ライト（南西上空から北東向きに照らす）
    commands.spawn((
        MainDirectionalLight,
        DirectionalLight {
            illuminance: 15_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(10.0, 30.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // アンビエント環境光
    commands.spawn((
        PointLight {
            intensity: 800_000.0,
            radius: 50.0,
            ..default()
        },
        Transform::from_xyz(0.0, 20.0, 0.0),
    ));
}
