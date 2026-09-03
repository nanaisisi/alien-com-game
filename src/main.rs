use bevy::prelude::*;

mod camera;
mod state;
mod ui;
mod world;

use camera::CameraPlugin;
use state::AppState;
use ui::GameUiPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Alien Com Game".into(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins((CameraPlugin, WorldPlugin, GameUiPlugin))
        .run();
}
