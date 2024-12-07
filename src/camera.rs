use bevy::window::PrimaryWindow;

use crate::*;

const CAMERA_SPEED: f32 = 500.0;
const CAMERA_ROTATION_SPEED: f32 = 1.5;
const CAMERA_SCALE_SPEED: f32 = 1.0;

const CAMERA_MAX_SCALE: f32 = 3.0;
const CAMERA_MIN_SCALE: f32 = 0.2;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_control)
            .add_systems(Update, mouse_click_system);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d).insert(Transform::from_xyz(
        WINDOW_WIDTH / 2.0 + TILE_SIZE / 2.0,
        WINDOW_HEIGHT / 2.0 + TILE_SIZE / 2.0,
        0.0,
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
        if camera_projection.scale > CAMERA_MAX_SCALE {
            camera_projection.scale = CAMERA_MAX_SCALE;
        }
    }
    if keys.any_pressed([KeyCode::KeyX]) {
        camera_projection.scale -= CAMERA_SCALE_SPEED * time.delta_secs();
        if camera_projection.scale < CAMERA_MIN_SCALE {
            camera_projection.scale = CAMERA_MIN_SCALE;
        }
    }
}

fn mouse_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    mut tiles: Query<(&mut Sprite, &mut TileData)>,
    interaction_query: Query<(&Interaction,), (Changed<Interaction>, With<Button>)>,
    tile_map: Res<TileMap>,
    mut commands: Commands,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) || !interaction_query.is_empty() {
        return;
    }

    let (camera_transform, projection) = camera.single();

    if let Some(cursor_position) = q_windows.single().cursor_position() {
        let cursor_position_global_axis = Vec2 {
            x: cursor_position.x,
            y: -cursor_position.y,
        };

        let root_to_camera_offset = camera_transform.translation.xy();
        let camera_to_cursor_offset = projection.scale
            * (Vec2 {
                x: -WINDOW_WIDTH / 2.0,
                y: WINDOW_HEIGHT / 2.0,
            } + cursor_position_global_axis);

        let game_cursor_position = root_to_camera_offset
            + (camera_transform.rotation * Vec3::from((camera_to_cursor_offset, 0.0))).xy();

        let tile_position = get_tile(game_cursor_position.x, game_cursor_position.y);
        let tile_id = tile_map.tile_map.get(&tile_position).unwrap();

        if let Ok((mut sprite, mut tile_data)) = tiles.get_mut(*tile_id) {
            if tile_data.empty {
                sprite.color = bevy::prelude::Color::Srgba(css::AZURE);
                tile_data.empty = false;
                spawn_tower(&mut commands, tile_position);
            }
        }
    }
}
