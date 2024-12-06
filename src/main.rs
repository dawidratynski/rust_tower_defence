pub use bevy::prelude::*;

use bevy::color::palettes::css;
use bevy::math::FloatOrd;

mod bullet;
mod constants;
mod enemy;
mod enemy_spawner;
mod map;
mod tower;
mod camera;

pub use bullet::*;
pub use constants::*;
pub use enemy::*;
pub use enemy_spawner::*;
pub use map::*;
pub use tower::*;
pub use camera::*;

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
        .add_plugins(CameraPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(TowerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(EnemySpawnerPlugin)
        .run();
}
