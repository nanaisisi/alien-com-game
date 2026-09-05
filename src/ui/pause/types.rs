use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenuRootUi;

#[derive(Component)]
pub struct PauseConfirmModal;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PauseModalType {
    #[default]
    ReturnToTitle,
    QuitToDesktop,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseMenuItem {
    Resume,
    Settings,
    ReturnToTitle,
    QuitToDesktop,
}

pub const PAUSE_MENU_ITEMS: [PauseMenuItem; 4] = [
    PauseMenuItem::Resume,
    PauseMenuItem::Settings,
    PauseMenuItem::ReturnToTitle,
    PauseMenuItem::QuitToDesktop,
];

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PauseModalFocusItem {
    Confirm,
    Cancel,
}

#[derive(Resource, Debug)]
pub struct PauseMenuFocus {
    pub current_item: PauseMenuItem,
    pub modal_open: bool,
    pub modal_type: PauseModalType,
    pub modal_focus: PauseModalFocusItem,
}

impl Default for PauseMenuFocus {
    fn default() -> Self {
        Self {
            current_item: PauseMenuItem::Resume,
            modal_open: false,
            modal_type: PauseModalType::ReturnToTitle,
            modal_focus: PauseModalFocusItem::Cancel,
        }
    }
}

pub fn reset_pause_focus(mut focus: ResMut<PauseMenuFocus>) {
    focus.current_item = PauseMenuItem::Resume;
    focus.modal_open = false;
    focus.modal_type = PauseModalType::ReturnToTitle;
    focus.modal_focus = PauseModalFocusItem::Cancel;
}

#[derive(Component, Debug, Clone, Copy)]
pub enum PauseButtonAction {
    Resume,
    Settings,
    RequestReturnToTitle,
    RequestQuitToDesktop,
    ConfirmModal,
    CancelModal,
}

#[derive(Component)]
pub struct PauseMenuButton(pub PauseMenuItem);

#[derive(Component)]
pub struct PauseModalButton(pub PauseModalFocusItem);
