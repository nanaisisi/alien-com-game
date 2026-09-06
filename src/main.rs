use bevy::prelude::*;

mod camera;
mod faction;
mod map;
mod state;
mod ui;
mod unit;
mod world;

use camera::CameraPlugin;
use faction::FactionPlugin;
use map::MapPlugin;
use state::AppState;
use ui::GameUiPlugin;
use unit::UnitPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Alien Com Game".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    filter: format!("{},icu_provider=error", bevy::log::DEFAULT_FILTER),
                    ..default()
                }),
        )
        .init_state::<AppState>()
        .add_plugins((
            CameraPlugin,
            WorldPlugin,
            MapPlugin,
            FactionPlugin,
            UnitPlugin,
            GameUiPlugin,
        ))
        .run();
}
