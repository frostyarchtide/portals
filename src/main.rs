mod player;
mod portal;

use crate::player::*;
use crate::portal::*;

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
            PlayerPlugin,
            PortalPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut portal_materials: ResMut<Assets<PortalMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let image = Image::new_target_texture(800, 600, TextureFormat::bevy_default());
    let image_handle = images.add(image);

    commands.spawn((
        PortalCamera,
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
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.)))),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::WHITE.with_luminance(0.3),
            ..default()
        })),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(0.5, 1.)))),
        MeshMaterial3d(portal_materials.add(PortalMaterial {
            color_texture: Some(image_handle),
            alpha_mode: AlphaMode::Opaque,
        })),
        Transform::from_xyz(0., 1., -3.),
    ));
}

fn update(player: Single<&Transform, With<Player>>, mut query: Query<&mut Transform, With<PortalCamera>>) {
    let player_transform = player.into_inner();

    for mut portal_transform in query.iter_mut() {
        *portal_transform = *player_transform;
    }
}
