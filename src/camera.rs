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
    pub last_drag_pos: Option<Vec2>,
}

impl Default for MapCamera {
    fn default() -> Self {
        Self {
            target_focal_point: Vec3::ZERO,
            current_focal_point: Vec3::ZERO,
            target_viewport_height: 18.0,
            current_viewport_height: 18.0,
            last_drag_pos: None,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let initial_viewport_height = 18.0;
    // 南から北向きに見下ろすオフセット（+Zから-Z向き）
    let camera_offset = Vec3::new(0.0, 14.0, 12.0);

    // 3D & UI統合カメラ
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
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
}

/// WASD / 矢印キーでパン移動、右ドラッグ / 中ドラッグでマップ移動、マウスホイールでズームイン/アウト
fn pan_zoom_camera_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    map_grid: Res<crate::map::MapGrid>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut Projection, &mut MapCamera)>,
    debug_state: Option<Res<crate::ui::debug_console::DebugConsoleState>>,
) {
    let Ok((mut transform, mut projection, mut map_cam)) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    // デバッグコンソールまたは警告モーダルが開いている場合はキーボードパン操作を無効化（文字入力と衝突防止）
    let block_keyboard_pan = debug_state
        .as_ref()
        .is_some_and(|s| s.is_open || s.show_warning_modal);

    // 1. パン操作（WASD / 矢印）
    // 南から北（画面上が北 = -Z、画面右が東 = +X）
    let forward_dir = Vec3::new(0.0, 0.0, -1.0);
    let right_dir = Vec3::new(1.0, 0.0, 0.0);

    let mut move_vec = Vec3::ZERO;
    let mut impulse_vec = Vec3::ZERO;

    if !block_keyboard_pan {
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

        // 1回チョンと押しただけでも確実に1タイル程度（またはステップ分）移動を感知できるように
        // just_pressed（押した瞬間）のインパルス移動
        if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
            impulse_vec += forward_dir;
        }
        if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
            impulse_vec -= forward_dir;
        }
        if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
            impulse_vec += right_dir;
        }
        if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
            impulse_vec -= right_dir;
        }
    }

    let move_speed = (map_cam.current_viewport_height * 1.2).clamp(10.0, 35.0);

    if impulse_vec.length_squared() > 0.0 {
        // 1タップあたり約1.2ワールドユニット（およそ1ヘックスタイル分）のステップ移動
        let step_size = (crate::map::HEX_RADIUS * 1.5).max(1.2);
        map_cam.target_focal_point += impulse_vec.normalize() * step_size;
    }

    // 長押し時の連続移動
    if move_vec.length_squared() > 0.0 {
        map_cam.target_focal_point += move_vec.normalize() * move_speed * dt;
    }

    // 2. マウスドラッグによる移動（右ボタンドラッグまたはホイール中ボタンドラッグ）
    let is_dragging =
        mouse_button.pressed(MouseButton::Right) || mouse_button.pressed(MouseButton::Middle);

    if let Ok(window) = windows.single() {
        if is_dragging {
            if let Some(cursor_pos) = window.cursor_position() {
                if let Some(last_pos) = map_cam.last_drag_pos {
                    let delta = cursor_pos - last_pos;
                    let h = window.height().max(1.0);
                    // 画面高さに対する正射影表示高さの比率からワールド空間での移動量を算出
                    let world_per_pixel = map_cam.current_viewport_height / h;
                    // カメラの傾き（Y=14, Z=12の斜め見下ろし：約49.4度傾斜）を考慮
                    // 画面上のY移動量をZ平面上でのワールド移動量に正確に変換
                    let sin_angle = 14.0 / (14.0_f32.powi(2) + 12.0_f32.powi(2)).sqrt();
                    let world_delta_x = delta.x * world_per_pixel;
                    let world_delta_z = delta.y * world_per_pixel / sin_angle;

                    // カーソルのドラッグに合わせてマップを直感的に掴んで動かす（ドラッグ方向と逆にカメラ移動）
                    let drag_offset = Vec3::new(-world_delta_x, 0.0, -world_delta_z);
                    map_cam.target_focal_point += drag_offset;
                    map_cam.current_focal_point += drag_offset;
                }
                map_cam.last_drag_pos = Some(cursor_pos);
            } else {
                map_cam.last_drag_pos = None;
            }
        } else {
            map_cam.last_drag_pos = None;
        }
    } else {
        map_cam.last_drag_pos = None;
    }

    // 縦方向（Z軸）の移動範囲制限（極地付近で制限）
    let grid_h = if map_grid.height > 0 { map_grid.height } else { crate::map::GRID_HEIGHT };
    let max_z = (grid_h as f32 * 1.5 * crate::map::HEX_RADIUS) * 0.5 + 4.0;
    map_cam.target_focal_point.z = map_cam.target_focal_point.z.clamp(-max_z, max_z);

    // 横方向（X軸）のシームレスなループ（ラップアラウンド）
    let grid_w = if map_grid.width > 0 { map_grid.width } else { crate::map::GRID_WIDTH };
    let world_width = crate::map::hex::map_world_width_with_width(crate::map::HEX_RADIUS, grid_w);
    if world_width > 0.0 {
        // target_focal_point.x を [0, world_width) または [-world_width/2, world_width/2) にラップ
        let half_w = world_width / 2.0;
        if map_cam.target_focal_point.x > half_w {
            map_cam.target_focal_point.x -= world_width;
            map_cam.current_focal_point.x -= world_width;
        } else if map_cam.target_focal_point.x < -half_w {
            map_cam.target_focal_point.x += world_width;
            map_cam.current_focal_point.x += world_width;
        }
    }

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
    let camera_offset = Vec3::new(0.0, 14.0, 12.0);
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
