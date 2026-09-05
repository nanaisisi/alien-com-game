use bevy::prelude::*;

use crate::faction::types::FactionId;
use crate::map::settings::{MapSize, PlanetEnvironment};

#[derive(Resource, Default)]
pub struct SelectedFactionMenu {
    pub faction: FactionId,
}

#[derive(Component)]
pub struct FactionSelectRoot;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum SelectAction {
    Choose(FactionId),
    ChooseEnvironment(PlanetEnvironment),
    ChooseSize(MapSize),
    RerollSeed,
    Confirm,
    Back,
}

#[derive(Component)]
pub struct DetailTitleText;

#[derive(Component)]
pub struct DetailDescText;

#[derive(Component)]
pub struct DetailPanelRoot;

#[derive(Component)]
pub struct FactionCard(pub FactionId);

#[derive(Component)]
pub struct EnvButton(pub PlanetEnvironment);

#[derive(Component)]
pub struct SizeButton(pub MapSize);

#[derive(Component)]
pub struct SeedDisplayText;

#[derive(Component)]
pub struct EnvDescText;
