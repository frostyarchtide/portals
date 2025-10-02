use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use std::f32::consts::FRAC_PI_2;

#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.005, 0.005))
    }
}

#[derive(Component)]
#[require(Camera3d, CameraSensitivity)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, lock_cursor)
            .add_systems(Update, move_player);
    }
}

fn lock_cursor(mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>) {
    primary_cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player: Single<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let (mut transform, camera_sensitivity) = player.into_inner();

    let mut movement_input: Vec3 = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement_input.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement_input.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement_input.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement_input.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        movement_input.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyC) {
        movement_input.y -= 1.0;
    }

    let movement = movement_input.normalize_or_zero() * time.delta_secs();
    let rotation = transform.rotation;
    transform.translation += rotation.mul_vec3(movement);

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
