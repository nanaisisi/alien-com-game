use bevy::prelude::*;
use crate::state::AppState;

pub const RESOLUTION_PRESETS: [(u32, u32); 4] = [
    (1280, 720),
    (1600, 900),
    (1920, 1080),
    (2560, 1440),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FpsLimitMode {
    Unlimited,
    Fps30,
    Fps60,
    Fps120,
    Fps144,
}

impl FpsLimitMode {
    pub fn next(&self) -> Self {
        match self {
            Self::Unlimited => Self::Fps30,
            Self::Fps30 => Self::Fps60,
            Self::Fps60 => Self::Fps120,
            Self::Fps120 => Self::Fps144,
            Self::Fps144 => Self::Unlimited,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Unlimited => Self::Fps144,
            Self::Fps30 => Self::Unlimited,
            Self::Fps60 => Self::Fps30,
            Self::Fps120 => Self::Fps60,
            Self::Fps144 => Self::Fps120,
        }
    }

    pub fn display_label(&self) -> &'static str {
        match self {
            Self::Unlimited => "無制限 (OFF)",
            Self::Fps30 => "30 FPS",
            Self::Fps60 => "60 FPS",
            Self::Fps120 => "120 FPS",
            Self::Fps144 => "144 FPS",
        }
    }

    pub fn target_frame_time(&self) -> Option<std::time::Duration> {
        match self {
            Self::Unlimited => None,
            Self::Fps30 => Some(std::time::Duration::from_nanos(1_000_000_000 / 30)),
            Self::Fps60 => Some(std::time::Duration::from_nanos(1_000_000_000 / 60)),
            Self::Fps120 => Some(std::time::Duration::from_nanos(1_000_000_000 / 120)),
            Self::Fps144 => Some(std::time::Duration::from_nanos(1_000_000_000 / 144)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntiAliasingMode {
    Off,
    Msaa2x,
    Msaa4x,
}

impl AntiAliasingMode {
    pub fn next(&self) -> Self {
        match self {
            Self::Off => Self::Msaa2x,
            Self::Msaa2x => Self::Msaa4x,
            Self::Msaa4x => Self::Off,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Off => Self::Msaa4x,
            Self::Msaa2x => Self::Off,
            Self::Msaa4x => Self::Msaa2x,
        }
    }

    pub fn display_label(&self) -> &'static str {
        match self {
            Self::Off => "オフ (OFF)",
            Self::Msaa2x => "MSAA 2x",
            Self::Msaa4x => "MSAA 4x",
        }
    }
}

/// ゲーム全体の設定リソース
#[derive(Resource, Debug, Clone)]
pub struct GameSettings {
    pub master_volume: u32, // 0..=100 (10%刻み)
    pub bgm_volume: u32,
    pub sfx_volume: u32,
    pub fullscreen: bool,
    pub resolution_index: usize,
    pub fps_limit: FpsLimitMode,
    pub vsync: bool,
    pub shadows_enabled: bool,
    pub anti_aliasing: AntiAliasingMode,
    pub return_state: AppState,
}

impl GameSettings {
    /// 実行マシンのGPU環境やメモリ・CPU環境に応じたデフォルト設定を生成。
    pub fn default_for_environment(
        adapter_info: Option<&bevy::render::renderer::RenderAdapterInfo>,
        system_info: Option<&bevy::diagnostic::SystemInfo>,
    ) -> Self {
        let (mem_below_16gb, is_single_core) = if let Some(sys) = system_info {
            let mem_gb = sys
                .memory
                .split_whitespace()
                .next()
                .and_then(|val| val.parse::<f64>().ok());
            let is_below_16 = mem_gb.map(|gb| gb < 15.5).unwrap_or(false);

            let cores = sys.core_count.parse::<usize>().unwrap_or(4);
            let is_single = cores <= 1;

            (is_below_16, is_single)
        } else {
            (false, false)
        };

        let (fps_limit, anti_aliasing, shadows_enabled) = if mem_below_16gb {
            let mem_str = system_info.map(|s| s.memory.as_str()).unwrap_or("Unknown");
            info!(
                "System memory ({}) is below 16 GB threshold. Enforcing low graphics preset.",
                mem_str
            );
            (FpsLimitMode::Fps30, AntiAliasingMode::Off, false)
        } else if is_single_core {
            let core_str = system_info.map(|s| s.core_count.as_str()).unwrap_or("1");
            info!(
                "System CPU count ({} core) is 1 core or less. Enforcing low graphics preset.",
                core_str
            );
            (FpsLimitMode::Fps30, AntiAliasingMode::Off, false)
        } else if cfg!(debug_assertions) {
            info!("Running in dev/debug build. Applying low quality graphics preset for performance.");
            (FpsLimitMode::Fps30, AntiAliasingMode::Off, false)
        } else {
            match adapter_info {
                Some(info) => {
                    let type_str = format!("{:?}", info.device_type);
                    if type_str.contains("DiscreteGpu") {
                        info!("Detected Discrete GPU ({}). Applying high quality graphics preset.", info.name);
                        (FpsLimitMode::Fps60, AntiAliasingMode::Msaa4x, true)
                    } else if type_str.contains("IntegratedGpu") {
                        info!("Detected Integrated GPU ({}). Applying medium quality graphics preset.", info.name);
                        (FpsLimitMode::Fps60, AntiAliasingMode::Msaa2x, true)
                    } else if type_str.contains("Cpu") {
                        info!("Detected CPU/Software renderer ({}). Applying lightweight graphics preset.", info.name);
                        (FpsLimitMode::Fps30, AntiAliasingMode::Off, false)
                    } else {
                        info!("Detected GPU ({}, type: {}). Applying standard graphics preset.", info.name, type_str);
                        (FpsLimitMode::Fps60, AntiAliasingMode::Msaa2x, true)
                    }
                }
                None => (FpsLimitMode::Fps60, AntiAliasingMode::Msaa4x, true),
            }
        };

        Self {
            master_volume: 80,
            bgm_volume: 70,
            sfx_volume: 80,
            fullscreen: false,
            resolution_index: 0, // 1280x720 (ウィンドウモード標準)
            fps_limit,
            vsync: true,
            shadows_enabled,
            anti_aliasing,
            return_state: AppState::Title,
        }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self::default_for_environment(None, None)
    }
}

#[derive(Component)]
pub struct SettingsRootUi;

#[derive(Component)]
pub struct ReturnToTitleConfirmModal;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsFocusItem {
    MasterVolume,
    BgmVolume,
    SfxVolume,
    Resolution,
    Fullscreen,
    FpsLimit,
    Vsync,
    Shadows,
    AntiAliasing,
    ResetDefaults,
    Resume,
    ReturnToTitle,
    Back,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalFocusItem {
    Confirm,
    Cancel,
}

#[derive(Resource, Debug)]
pub struct SettingsNavFocus {
    pub current_item: SettingsFocusItem,
    pub modal_item: ModalFocusItem,
}

impl Default for SettingsNavFocus {
    fn default() -> Self {
        Self {
            current_item: SettingsFocusItem::MasterVolume,
            modal_item: ModalFocusItem::Cancel,
        }
    }
}

pub fn reset_settings_focus(mut focus: ResMut<SettingsNavFocus>) {
    focus.current_item = SettingsFocusItem::MasterVolume;
    focus.modal_item = ModalFocusItem::Cancel;
}

#[derive(Component)]
pub struct SettingsNavRow(pub SettingsFocusItem);

#[derive(Component)]
pub struct ModalNavButton(pub ModalFocusItem);

#[derive(Component, Debug, Clone, Copy)]
pub enum SettingsButtonAction {
    MasterVolumeDown,
    MasterVolumeUp,
    BgmVolumeDown,
    BgmVolumeUp,
    SfxVolumeDown,
    SfxVolumeUp,
    ResolutionPrev,
    ResolutionNext,
    ToggleFullscreen,
    FpsLimitPrev,
    FpsLimitNext,
    ToggleVsync,
    ToggleShadows,
    AntiAliasingPrev,
    AntiAliasingNext,
    ResetDefaults,
    ResumeGame,
    RequestReturnToTitle,
    ConfirmReturnToTitle,
    CancelReturnToTitle,
    Back,
}

#[derive(Component)]
pub enum SettingValueLabel {
    MasterVolume,
    BgmVolume,
    SfxVolume,
    Resolution,
    Fullscreen,
    FpsLimit,
    Vsync,
    Shadows,
    AntiAliasing,
}

// カラーテーマ（ui::theme より再エクスポート）
pub use crate::ui::theme::{
    ACCENT_COLOR, BORDER_COLOR, BORDER_DANGER as DANGER_BORDER,
    BUTTON_DANGER_HOVERED as DANGER_HOVERED_BUTTON, BUTTON_DANGER_NORMAL as DANGER_NORMAL_BUTTON,
    BUTTON_DANGER_PRESSED as DANGER_PRESSED_BUTTON, BUTTON_HOVERED as HOVERED_BUTTON,
    BUTTON_NORMAL as NORMAL_BUTTON, BUTTON_PRESSED as PRESSED_BUTTON, PANEL_BG, ROW_BG,
    TEXT_MAIN as TEXT_COLOR,
};
