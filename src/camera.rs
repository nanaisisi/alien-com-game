use bevy::camera::ScalingMode;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::state::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                pan_zoom_camera_system.run_if(in_state(AppState::InGame)),
            );
    }
}

/// RTS/4X風のカメラ操作用マーカー
#[derive(Component)]
pub struct MapCamera {
    pub target_focal_point: Vec3,
    pub current_focal_point: Vec3,
    pub target_viewport_height: f32,
    pub current_viewport_height: f32,
}

impl Default for MapCamera {
    fn default() -> Self {
        Self {
            target_focal_point: Vec3::ZERO,
            current_focal_point: Vec3::ZERO,
            target_viewport_height: 18.0,
            current_viewport_height: 18.0,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let initial_viewport_height = 18.0;
    let camera_offset = Vec3::new(10.0, 14.0, 10.0);

    // 3D & UI統合カメラ
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: initial_viewport_height,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(camera_offset).looking_at(Vec3::ZERO, Vec3::Y),
        MapCamera {
            target_viewport_height: initial_viewport_height,
            current_viewport_height: initial_viewport_height,
            ..default()
        },
    ));

    // メイン指向性ライト（太陽光）
    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(15.0, 30.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
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

/// WASD / 矢印キーでパン移動、マウスホイールでズームイン/アウト
fn pan_zoom_camera_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut Projection, &mut MapCamera)>,
) {
    let Ok((mut transform, mut projection, mut map_cam)) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    // 1. パン操作（WASD / 矢印）
    // 斜め見下ろしカメラに合わせて、X+Z軸方向に直交する向きに移動
    let forward_dir = Vec3::new(-1.0, 0.0, -1.0).normalize();
    let right_dir = Vec3::new(1.0, 0.0, -1.0).normalize();

    let mut move_vec = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        move_vec += forward_dir;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        move_vec -= forward_dir;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        move_vec += right_dir;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        move_vec -= right_dir;
    }

    let move_speed = (map_cam.current_viewport_height * 1.2).clamp(10.0, 35.0);
    if move_vec.length_squared() > 0.0 {
        map_cam.target_focal_point += move_vec.normalize() * move_speed * dt;
    }

    // 移動可能範囲の制限（マップ中心付近から離れすぎないように）
    let max_boundary = 22.0;
    map_cam.target_focal_point.x = map_cam.target_focal_point.x.clamp(-max_boundary, max_boundary);
    map_cam.target_focal_point.z = map_cam.target_focal_point.z.clamp(-max_boundary, max_boundary);

    // 2. ズーム操作（マウスホイール）
    for ev in mouse_wheel.read() {
        let zoom_factor = 1.8;
        map_cam.target_viewport_height -= ev.y * zoom_factor;
    }
    map_cam.target_viewport_height = map_cam.target_viewport_height.clamp(6.0, 36.0);

    // 3. スムーズな補間 (LERP)
    let lerp_speed = 12.0;
    map_cam.current_focal_point = map_cam
        .current_focal_point
        .lerp(map_cam.target_focal_point, (lerp_speed * dt).min(1.0));
    map_cam.current_viewport_height += (map_cam.target_viewport_height - map_cam.current_viewport_height)
        * (lerp_speed * dt).min(1.0);

    // カメラ位置を注視点 + オフセットに更新
    let camera_offset = Vec3::new(10.0, 14.0, 10.0);
    let new_cam_pos = map_cam.current_focal_point + camera_offset;
    transform.translation = new_cam_pos;
    transform.look_at(map_cam.current_focal_point, Vec3::Y);

    // プロジェクションの viewport_height を更新
    if let Projection::Orthographic(ref mut ortho) = *projection {
        ortho.scaling_mode = ScalingMode::FixedVertical {
            viewport_height: map_cam.current_viewport_height,
        };
    }
}
