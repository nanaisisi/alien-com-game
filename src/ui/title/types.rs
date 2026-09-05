use bevy::prelude::*;

#[derive(Component)]
pub struct TitleRootUi;

#[derive(Resource, Default)]
pub struct TitleMenuFocus {
    pub selected_index: Option<usize>,
}

pub fn reset_title_focus(mut focus: ResMut<TitleMenuFocus>) {
    focus.selected_index = Some(0); // デフォルトで一番上（NEW GAME）にフォーカス
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuButtonAction {
    NewGame,
    LoadGame,
    Settings,
    Exit,
}

pub const MENU_ACTIONS: [MenuButtonAction; 4] = [
    MenuButtonAction::NewGame,
    MenuButtonAction::LoadGame,
    MenuButtonAction::Settings,
    MenuButtonAction::Exit,
];

#[derive(Component)]
pub struct TitleMenuButton(pub usize);
