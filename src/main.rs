mod player;

use crate::player::*;
use bevy::prelude::*;
#[allow(unused_imports)]
#[cfg(debug_assertions)]
use bevy_dylib;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        Transform::from_xyz(1., 1., 3.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(-1., 1., 1.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
    ));
}
