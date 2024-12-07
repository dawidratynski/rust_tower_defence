pub use bevy::prelude::*;

use bevy::color::palettes::css;
use bevy::math::FloatOrd;

mod base;
mod bullet;
mod camera;
mod constants;
mod despawn;
mod enemy;
mod enemy_spawner;
mod map;
mod player;
mod tower;
mod ui;
mod utils;

pub use base::*;
pub use bullet::*;
pub use camera::*;
pub use constants::*;
pub use despawn::*;
pub use enemy::*;
pub use enemy_spawner::*;
pub use map::*;
pub use player::*;
pub use tower::*;
pub use ui::*;
pub use utils::*;

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
        .add_plugins(UIPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(TowerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(EnemySpawnerPlugin)
        .add_plugins(BasePlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
