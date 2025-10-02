mod player;

use crate::player::*;

use bevy::{prelude::*, render::render_resource::TextureFormat};
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
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = Image::new_target_texture(2048, 2048, TextureFormat::bevy_default());
    let image_handle = images.add(image);

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: -1,
            target: image_handle.clone().into(),
            clear_color: Color::BLACK.into(),
            ..default()
        },
        Transform::from_xyz(1., 1., 2.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Player,
        Transform::from_xyz(1., 1., 2.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(-1., 1., 1.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(image_handle),
            ..default()
        })),
    ));
}
