use bevy::prelude::*;

use crate::state::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_game_world);
    }
}

#[derive(Component)]
pub struct WorldEntity;

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
