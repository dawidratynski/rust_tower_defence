use bevy::prelude::*;

use crate::game_config::*;

const CAMERA_SPEED: f32 = 500.0;
const CAMERA_ROTATION_SPEED: f32 = 1.5;
const CAMERA_SCALE_SPEED: f32 = 1.0;

const CAMERA_MAX_SCALE: f32 = 3.0;
const CAMERA_MIN_SCALE: f32 = 0.2;
const CAMERA_DEFAULT_SCALE: f32 = 1.5;

const CAMERA_MAX_X: f32 = TILE_SIZE * 10.0;
const CAMERA_MIN_X: f32 = -TILE_SIZE * 10.0;
const CAMERA_MAX_Y: f32 = TILE_SIZE * 10.0;
const CAMERA_MIN_Y: f32 = -TILE_SIZE * 10.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_control);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: CAMERA_DEFAULT_SCALE,
            ..OrthographicProjection::default_2d()
        },
    ));
}

fn camera_control(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    time: Res<Time>,
) {
    let (mut camera_transform, mut camera_projection) = camera_query.single_mut();

    let local_right = camera_transform.right();
    let local_up = camera_transform.up();

    if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        camera_transform.translation += local_up * CAMERA_SPEED * time.delta_secs();
    }
    if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        camera_transform.translation -= local_right * CAMERA_SPEED * time.delta_secs();
    }
    if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        camera_transform.translation -= local_up * CAMERA_SPEED * time.delta_secs();
    }
    if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        camera_transform.translation += local_right * CAMERA_SPEED * time.delta_secs();
    }

    if keys.any_pressed([KeyCode::KeyQ]) {
        camera_transform.rotate_z(CAMERA_ROTATION_SPEED * time.delta_secs());
    }
    if keys.any_pressed([KeyCode::KeyE]) {
        camera_transform.rotate_z(-CAMERA_ROTATION_SPEED * time.delta_secs());
    }

    if keys.any_pressed([KeyCode::KeyZ]) {
        camera_projection.scale += CAMERA_SCALE_SPEED * time.delta_secs();
    }
    if keys.any_pressed([KeyCode::KeyX]) {
        camera_projection.scale -= CAMERA_SCALE_SPEED * time.delta_secs();
    }

    camera_projection.scale = camera_projection
        .scale
        .clamp(CAMERA_MIN_SCALE, CAMERA_MAX_SCALE);
    camera_transform.translation.x = camera_transform
        .translation
        .x
        .clamp(CAMERA_MIN_X, CAMERA_MAX_X);
    camera_transform.translation.y = camera_transform
        .translation
        .y
        .clamp(CAMERA_MIN_Y, CAMERA_MAX_Y);
}
