use bevy::prelude::*;

/// デバッグコンソールの全体状態を管理するリソース
#[derive(Resource, Debug)]
pub struct DebugConsoleState {
    /// 警告ダイアログで有効化が承認されたか
    pub is_unlocked: bool,
    /// コンソールUIが開いているか
    pub is_open: bool,
    /// 初回警告モーダルが表示中か
    pub show_warning_modal: bool,
    /// 現在のコマンド入力文字列
    pub input_text: String,
    /// 実行履歴
    pub history: Vec<String>,
    /// 履歴参照インデックス（None = 最新編集行）
    pub history_index: Option<usize>,
    /// コンソール出力メッセージログ (テキスト, 色)
    pub logs: Vec<(String, Color)>,
    /// 隠しトリガーの連続クリック回数
    pub secret_click_count: u32,
    /// 最後のクリック時刻（秒数カウント用）
    pub last_click_time: f32,
}

impl Default for DebugConsoleState {
    fn default() -> Self {
        Self {
            is_unlocked: false,
            is_open: false,
            show_warning_modal: false,
            input_text: String::new(),
            history: Vec::new(),
            history_index: None,
            logs: vec![
                (
                    "=== ALIEN COM GAME DEBUG CONSOLE ===".to_string(),
                    Color::srgb(0.25, 0.85, 0.75),
                ),
                (
                    "Type 'help' for a list of available commands.".to_string(),
                    Color::srgb(0.60, 0.72, 0.82),
                ),
            ],
            secret_click_count: 0,
            last_click_time: 0.0,
        }
    }
}

impl DebugConsoleState {
    pub fn add_log(&mut self, text: impl Into<String>, color: Color) {
        self.logs.push((text.into(), color));
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    /// 隠しエリアクリック時のカウント処理。5回クリックで真を返す。
    /// 前回のクリックから5秒以上経過した場合はリセット。
    pub fn register_secret_click(&mut self, current_time: f32) -> bool {
        if self.secret_click_count > 0 && current_time - self.last_click_time > 5.0 {
            self.secret_click_count = 0;
        }
        self.last_click_time = current_time;
        self.secret_click_count += 1;
        if self.secret_click_count >= 5 {
            self.secret_click_count = 0;
            true
        } else {
            false
        }
    }
}

/// 隠しデバッグトリガーエリア用マーカー
#[derive(Component, Debug)]
pub struct DebugSecretTriggerArea;

/// デバッグモード初回警告モーダルのルートエンティティ用マーカー
#[derive(Component, Debug)]
pub struct DebugWarningModal;

/// 警告モーダルのボタンアクション
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugWarningAction {
    /// デバッグモードを有効化してコンソールを開く
    Enable,
    /// キャンセルして閉じる
    Cancel,
}

/// コンソール本体UIのルートエンティティ用マーカー
#[derive(Component, Debug)]
pub struct DebugConsoleRoot;

/// コンソール右上の閉じる「×」ボタン
#[derive(Component, Debug)]
pub struct DebugConsoleCloseButton;

/// コンソール内の入力テキスト表示部マーカー
#[derive(Component, Debug)]
pub struct DebugConsoleInputText;

/// コンソール内のログ表示コンテナマーカー
#[derive(Component, Debug)]
pub struct DebugConsoleLogContainer;

/// コンソール内のログ1行ごとのマーカー
#[derive(Component, Debug)]
pub struct DebugConsoleLogItem;
