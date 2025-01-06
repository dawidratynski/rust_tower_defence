pub use bevy::prelude::*;

use game_config::*;

mod bullet;
mod camera;
mod despawn;
mod enemy;
mod enemy_spawner;
mod game_config;
mod game_state;
mod game_time;
mod map;
mod pathfinding;
mod player_base;
mod status_effect;
mod tower;
mod tower_placement;
mod tower_types;
mod ui;
mod ui_config;
mod utils;
mod victory_defeat;

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
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(camera::CameraPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(tower::TowerPlugin)
        .add_plugins(bullet::BulletPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(enemy_spawner::EnemySpawnerPlugin)
        .add_plugins(player_base::PlayerBasePlugin)
        .add_plugins(despawn::DespawnPlugin)
        .add_plugins(game_state::GameStatePlugin)
        .add_plugins(game_time::GameTimePlugin)
        .add_plugins(tower_placement::TowerPlacementPlugin)
        .add_plugins(pathfinding::PathfindingPlugin)
        .run();
}
