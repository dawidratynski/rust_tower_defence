pub use bevy::prelude::*;

use bevy::color::palettes::css;
use bevy::math::FloatOrd;

mod bullet;
mod camera;
mod despawn;
mod enemy;
mod enemy_spawner;
mod game_config;
mod game_state;
mod game_time;
mod map;
mod player_base;
mod tower;
mod ui;
mod ui_config;
mod utils;

pub use bullet::*;
pub use camera::*;
pub use despawn::*;
pub use enemy::*;
pub use enemy_spawner::*;
pub use game_config::*;
pub use game_state::*;
pub use game_time::*;
pub use map::*;
pub use player_base::*;
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
        .add_plugins(PlayerBasePlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(GameTimePlugin)
        .run();
}
