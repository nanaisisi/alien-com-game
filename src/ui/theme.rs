use bevy::prelude::*;

// -----------------------------------------------------------------------------
// フォントリソース定数
// -----------------------------------------------------------------------------
pub const FONT_REGULAR: &str = "fonts/UDEVGothicNF-Regular.ttf";
pub const FONT_BOLD: &str = "fonts/UDEVGothicNF-Bold.ttf";

// -----------------------------------------------------------------------------
// 基本パレット（SFサイバー / ディープスペース調）
// -----------------------------------------------------------------------------
/// 全画面オーバーレイ（背後を暗く透かす背景）
pub const OVERLAY_BG: Color = Color::srgba(0.02, 0.04, 0.07, 0.82);

/// パネルの主背景色
pub const PANEL_BG: Color = Color::srgba(0.06, 0.09, 0.14, 0.95);

/// カード・サブパネルの背景色
pub const CARD_BG: Color = Color::srgba(0.08, 0.14, 0.22, 0.85);

/// 行・リスト項目の背景色
pub const ROW_BG: Color = Color::srgba(0.10, 0.14, 0.20, 0.70);

/// 通常の境界線カラー
pub const BORDER_COLOR: Color = Color::srgb(0.22, 0.38, 0.52);

/// ハイライト／アクティブな境界線・アクセントシアン
pub const ACCENT_COLOR: Color = Color::srgb(0.25, 0.85, 0.75);

/// テキスト主色（高コントラスト白）
pub const TEXT_MAIN: Color = Color::srgb(0.92, 0.96, 0.98);

/// テキスト補助色（淡い水色・グレー）
pub const TEXT_MUTED: Color = Color::srgb(0.60, 0.72, 0.82);

// -----------------------------------------------------------------------------
// ボタン状態カラー（通常ボタン）
// -----------------------------------------------------------------------------
pub const BUTTON_NORMAL: Color = Color::srgb(0.12, 0.16, 0.22);
pub const BUTTON_HOVERED: Color = Color::srgb(0.20, 0.32, 0.45);
pub const BUTTON_PRESSED: Color = Color::srgb(0.15, 0.50, 0.65);

// -----------------------------------------------------------------------------
// ボタン状態カラー（警告・危険ボタン: 破棄/終了/キャンセル等）
// -----------------------------------------------------------------------------
pub const BUTTON_DANGER_NORMAL: Color = Color::srgb(0.55, 0.15, 0.18);
pub const BUTTON_DANGER_HOVERED: Color = Color::srgb(0.72, 0.22, 0.26);
pub const BUTTON_DANGER_PRESSED: Color = Color::srgb(0.85, 0.30, 0.35);
pub const BORDER_DANGER: Color = Color::srgb(0.80, 0.35, 0.35);

// -----------------------------------------------------------------------------
// 構造化テーマ（求心性結合の緩和と役割ごとの凝集度向上）
// -----------------------------------------------------------------------------

/// ボタン関連のカラースタイル
#[derive(Debug, Clone, Copy)]
pub struct ButtonStyle {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub border: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonTheme {
    pub standard: ButtonStyle,
    pub danger: ButtonStyle,
}

/// 面・背景・枠線関連のカラースタイル
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct SurfaceTheme {
    pub overlay: Color,
    pub panel: Color,
    pub card: Color,
    pub row: Color,
    pub border: Color,
    pub accent: Color,
}

/// テキスト文字色スタイル
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct TextTheme {
    pub main: Color,
    pub muted: Color,
    pub accent: Color,
}

/// フォントファイルパス
#[derive(Debug, Clone, Copy)]
pub struct FontTheme {
    pub regular: &'static str,
    pub bold: &'static str,
}

impl ButtonStyle {
    #[inline]
    pub const fn normal(&self) -> Color {
        self.normal
    }

    #[inline]
    pub const fn hovered(&self) -> Color {
        self.hovered
    }

    #[inline]
    pub const fn pressed(&self) -> Color {
        self.pressed
    }

    #[inline]
    pub const fn border(&self) -> Color {
        self.border
    }
}

#[allow(dead_code)]
impl ButtonTheme {
    #[inline]
    pub const fn standard(&self) -> &ButtonStyle {
        &self.standard
    }

    #[inline]
    pub const fn danger(&self) -> &ButtonStyle {
        &self.danger
    }

    #[inline]
    pub const fn get(&self, danger: bool) -> &ButtonStyle {
        if danger {
            &self.danger
        } else {
            &self.standard
        }
    }
}

#[allow(dead_code)]
impl SurfaceTheme {
    #[inline]
    pub const fn overlay(&self) -> Color {
        self.overlay
    }

    #[inline]
    pub const fn panel(&self) -> Color {
        self.panel
    }

    #[inline]
    pub const fn card(&self) -> Color {
        self.card
    }

    #[inline]
    pub const fn row(&self) -> Color {
        self.row
    }

    #[inline]
    pub const fn border(&self) -> Color {
        self.border
    }

    #[inline]
    pub const fn accent(&self) -> Color {
        self.accent
    }
}

#[allow(dead_code)]
impl TextTheme {
    #[inline]
    pub const fn main(&self) -> Color {
        self.main
    }

    #[inline]
    pub const fn muted(&self) -> Color {
        self.muted
    }

    #[inline]
    pub const fn accent(&self) -> Color {
        self.accent
    }
}

impl FontTheme {
    #[inline]
    pub const fn regular(&self) -> &'static str {
        self.regular
    }

    #[inline]
    pub const fn bold(&self) -> &'static str {
        self.bold
    }
}

/// 統一UIテーマインターフェース
#[derive(Debug, Clone, Copy, Default)]
pub struct UiTheme;

#[allow(dead_code)]
impl UiTheme {
    pub const BUTTONS: ButtonTheme = ButtonTheme {
        standard: ButtonStyle {
            normal: BUTTON_NORMAL,
            hovered: BUTTON_HOVERED,
            pressed: BUTTON_PRESSED,
            border: BORDER_COLOR,
        },
        danger: ButtonStyle {
            normal: BUTTON_DANGER_NORMAL,
            hovered: BUTTON_DANGER_HOVERED,
            pressed: BUTTON_DANGER_PRESSED,
            border: BORDER_DANGER,
        },
    };

    pub const SURFACES: SurfaceTheme = SurfaceTheme {
        overlay: OVERLAY_BG,
        panel: PANEL_BG,
        card: CARD_BG,
        row: ROW_BG,
        border: BORDER_COLOR,
        accent: ACCENT_COLOR,
    };

    pub const TEXT: TextTheme = TextTheme {
        main: TEXT_MAIN,
        muted: TEXT_MUTED,
        accent: ACCENT_COLOR,
    };

    pub const FONTS: FontTheme = FontTheme {
        regular: FONT_REGULAR,
        bold: FONT_BOLD,
    };

    #[inline]
    pub const fn buttons() -> &'static ButtonTheme {
        &Self::BUTTONS
    }

    #[inline]
    pub const fn button(danger: bool) -> &'static ButtonStyle {
        Self::BUTTONS.get(danger)
    }

    #[inline]
    pub const fn surfaces() -> &'static SurfaceTheme {
        &Self::SURFACES
    }

    #[inline]
    pub const fn text() -> &'static TextTheme {
        &Self::TEXT
    }

    #[inline]
    pub const fn fonts() -> &'static FontTheme {
        &Self::FONTS
    }
}
