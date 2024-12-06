use crate::*;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_control);
    }
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2d).insert(Transform::from_xyz(
        WINDOW_WIDTH / 2.0,
        WINDOW_HEIGHT / 2.0,
        0.0,
    ));
}


fn camera_control(){}