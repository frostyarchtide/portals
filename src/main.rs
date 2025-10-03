mod player;
mod portal;

use crate::player::*;
use crate::portal::*;

use bevy::camera::visibility::RenderLayers;
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
    let portal_layer = RenderLayers::layer(1);

    commands.spawn((
        Player,
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(standard_materials.add(StandardMaterial {
            base_color: Color::linear_rgb(1., 0., 0.),
            ..default()
        })),
        Transform::from_xyz(1., 1., 2.).looking_at(Vec3::ZERO, Vec3::Y),
        RenderLayers::layer(0).with(1),
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

    commands
        .spawn((
            Portal {
                target: Transform::from_xyz(3., 1., -1.).looking_at(Vec3::Y, Vec3::Y),
            },
            Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(0.5, 1.)))),
            MeshMaterial3d(portal_materials.add(PortalMaterial {
                color_texture: Some(image_handle.clone().into()),
                alpha_mode: AlphaMode::Opaque,
            })),
            Transform::from_xyz(0., 1., -3.),
            portal_layer.clone(),
        ))
        .with_child((
            PortalCamera::default(),
            Camera {
                order: -2,
                target: image_handle.clone().into(),
                clear_color: Color::BLACK.into(),
                ..default()
            },
        ));
    
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(0.5, 1.)))),
        MeshMaterial3d(standard_materials.add(StandardMaterial::default())),
        Transform::from_xyz(0., 1., -3.),
    ));

    commands
        .spawn((
            Portal {
                target: Transform::from_xyz(0., 1., -3.),
            },
            Mesh3d(meshes.add(Plane3d::new(Vec3::NEG_Z, Vec2::new(0.5, 1.)))),
            MeshMaterial3d(portal_materials.add(PortalMaterial {
                color_texture: Some(image_handle.clone().into()),
                alpha_mode: AlphaMode::Opaque,
            })),
            Transform::from_xyz(3., 1., -1.).looking_at(Vec3::Y, Vec3::Y),
            portal_layer.clone(),
        ))
        .with_child((
            PortalCamera::default(),
            Camera {
                order: -1,
                target: image_handle.clone().into(),
                clear_color: Color::BLACK.into(),
                ..default()
            },
        ));
}

fn update(
    player: Single<&Transform, (With<Player>, Without<PortalCamera>)>,
    portal_query: Query<(&Children, &Transform, &Portal), Without<PortalCamera>>,
    mut portal_camera_query: Query<
        &mut Transform,
        (With<PortalCamera>, Without<Portal>, Without<Player>),
    >,
) {
    let player_transform = player.into_inner();

    for (children, portal_transform, portal) in portal_query.iter() {
        let portal_transform_inverse = Transform::from_matrix(portal_transform.to_matrix().inverse());
        let relative_transform = portal_transform_inverse * *player_transform;

        for child in children.iter() {
            let Ok(mut portal_camera_transform) = portal_camera_query.get_mut(child) else {
                continue;
            };

            *portal_camera_transform = portal_transform_inverse * portal.target * relative_transform;
        }
    }
}
