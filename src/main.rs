use bevy::color::palettes::css;
use bevy::math::FloatOrd;
use bevy::prelude::*;

pub const SPRITE_RESOLUTION: u16 = 16;
pub const SPRITE_SCALE: u16 = 4;
pub const TILE_SIZE: f32 = SPRITE_RESOLUTION as f32 * SPRITE_SCALE as f32;

pub const MAP_HEIGHT: u32 = 8;
pub const MAP_WIDTH: u32 = 12;

pub const WINDOW_HEIGHT: f32 = MAP_HEIGHT as f32 * TILE_SIZE as f32;
pub const WINDOW_WIDTH: f32 = MAP_WIDTH as f32 * TILE_SIZE as f32;

pub const GAME_WINDOW_TITLE: &str = "Game prototye v1";

mod bullet;
mod enemy;
mod enemy_spawner;
mod map;
mod tower;

pub use bullet::*;
pub use enemy::*;
pub use enemy_spawner::*;
pub use map::*;
pub use tower::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: GAME_WINDOW_TITLE.to_owned(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_plugins(TowerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(EnemySpawnerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    spawn_camera(&mut commands);
    spawn_basic_scene(&mut commands);
}

fn spawn_camera(commands: &mut Commands) {
    commands.spawn(Camera2d).insert(Transform::from_xyz(
        WINDOW_WIDTH / 2.0,
        WINDOW_HEIGHT / 2.0,
        0.0,
    ));
}
