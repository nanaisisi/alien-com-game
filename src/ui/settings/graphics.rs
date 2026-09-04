use bevy::prelude::*;
use bevy::window::PresentMode;

use crate::camera::MainDirectionalLight;
use super::types::{AntiAliasingMode, GameSettings, RESOLUTION_PRESETS};

/// フレームレート制限用の状態追跡リソース
#[derive(Resource)]
pub struct FpsLimiterState {
    pub last_frame_instant: std::time::Instant,
}

impl Default for FpsLimiterState {
    fn default() -> Self {
        Self {
            last_frame_instant: std::time::Instant::now(),
        }
    }
}

/// 起動時に実行環境（GPUスペック等）を検知し、最適なグラフィック設定を自動適用するシステム
pub fn setup_environment_settings_system(
    mut settings: ResMut<GameSettings>,
    adapter_info: Option<Res<bevy::render::renderer::RenderAdapterInfo>>,
    system_info: Option<Res<bevy::diagnostic::SystemInfo>>,
) {
    let adapter_ref = adapter_info.as_deref();
    let sys_ref = system_info.as_deref();
    let env_settings = GameSettings::default_for_environment(adapter_ref, sys_ref);
    settings.fps_limit = env_settings.fps_limit;
    settings.shadows_enabled = env_settings.shadows_enabled;
    settings.anti_aliasing = env_settings.anti_aliasing;
    info!(
        "System Environment Graphic Settings applied: FPS Limit={:?}, MSAA={:?}, Shadows={}",
        settings.fps_limit, settings.anti_aliasing, settings.shadows_enabled
    );
}

/// FPS制限適用システム
pub fn enforce_fps_limit_system(
    settings: Res<GameSettings>,
    mut limiter: ResMut<FpsLimiterState>,
) {
    if let Some(target_duration) = settings.fps_limit.target_frame_time() {
        let elapsed = limiter.last_frame_instant.elapsed();
        if elapsed < target_duration {
            let sleep_duration = target_duration - elapsed;
            std::thread::sleep(sleep_duration);
        }
    }
    limiter.last_frame_instant = std::time::Instant::now();
}

/// ゲーム設定の変更（解像度、フルスクリーン、VSync、影、MSAA）をウィンドウやレンダリング各部に適用するシステム
pub fn apply_graphics_settings_system(
    settings: Res<GameSettings>,
    mut windows: Query<&mut Window>,
    mut lights: Query<&mut DirectionalLight, With<MainDirectionalLight>>,
    mut cameras: Query<&mut Msaa, With<Camera3d>>,
) {
    if !settings.is_changed() {
        return;
    }

    // 1. ウィンドウ設定（解像度・画面モード・VSync）の適用
    if let Ok(mut window) = windows.single_mut() {
        let (width, height) = RESOLUTION_PRESETS[settings.resolution_index];
        window.resolution.set(width as f32, height as f32);

        window.mode = if settings.fullscreen {
            bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current)
        } else {
            bevy::window::WindowMode::Windowed
        };

        window.present_mode = if settings.vsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };
    }

    // 2. 影設定の適用
    for mut light in &mut lights {
        light.shadow_maps_enabled = settings.shadows_enabled;
    }

    // 3. アンチエイリアス（MSAA）設定の適用
    for mut msaa in &mut cameras {
        *msaa = match settings.anti_aliasing {
            AntiAliasingMode::Off => Msaa::Off,
            AntiAliasingMode::Msaa2x => Msaa::Sample2,
            AntiAliasingMode::Msaa4x => Msaa::Sample4,
        };
    }
}
