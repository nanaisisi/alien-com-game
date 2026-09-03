use bevy::{camera::ScalingMode, prelude::*};

mod state;
mod ui;

use state::AppState;
use ui::GameUiPlugin;

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
        .add_plugins(GameUiPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(AppState::InGame), setup_game_world)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // 3D & UI統合カメラ
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 環境光
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}

#[derive(Component)]
struct WorldEntity;

fn setup_game_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Initializing Game World...");

    // plane
    commands.spawn((
        WorldEntity,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
    // cubes
    let cube_mesh = meshes.add(Cuboid::default());
    let cube_material = materials.add(Color::srgb(0.8, 0.7, 0.6));

    for (x, z) in [(1.5, 1.5), (1.5, -1.5), (-1.5, 1.5), (-1.5, -1.5)] {
        commands.spawn((
            WorldEntity,
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_material.clone()),
            Transform::from_xyz(x, 0.5, z),
        ));
    }
}
