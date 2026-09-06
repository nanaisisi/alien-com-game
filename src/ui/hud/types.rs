use bevy::prelude::*;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudAction {
    EndTurn,
    OpenDiplomacy,
    OpenMenu,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HudLabel {
    Turn,
    Energy,
    Production,
    Science,
    Food,
}

pub const END_TURN_NORMAL: Color = Color::srgb(0.10, 0.30, 0.35);
pub const END_TURN_HOVER: Color = Color::srgb(0.16, 0.48, 0.55);
pub const END_TURN_PRESSED: Color = Color::srgb(0.25, 0.75, 0.80);
