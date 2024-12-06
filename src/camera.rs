use crate::*;

const CAMERA_SPEED: f32 = 200.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_control);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d).insert(Transform::from_xyz(
        WINDOW_WIDTH / 2.0,
        WINDOW_HEIGHT / 2.0,
        0.0,
    ));
}

fn camera_control(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        camera.translation.y += CAMERA_SPEED * time.delta_secs();
    }
    if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        camera.translation.x -= CAMERA_SPEED * time.delta_secs();
    }
    if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        camera.translation.y -= CAMERA_SPEED * time.delta_secs();
    }
    if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        camera.translation.x += CAMERA_SPEED * time.delta_secs();
    }
}
