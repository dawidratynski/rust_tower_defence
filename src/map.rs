use bevy::utils::hashbrown::HashMap;

use crate::*;

#[derive(Bundle)]
pub struct TileBundle {
    pub sprite: Sprite,
    pub tile_data: TileData,
}

#[derive(Component)]
pub struct TileData {
    pub empty: bool,
}

#[derive(Resource)]
pub struct TileMap {
    pub tile_map: HashMap<(i32, i32), Entity>,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap {
            tile_map: HashMap::new(),
        })
        .add_systems(Startup, spawn_basic_scene);
    }
}

fn spawn_basic_scene(mut commands: Commands, mut tile_map: ResMut<TileMap>) {
    commands
        .spawn(Sprite::from_color(
            css::DARK_GOLDENROD,
            Vec2::splat(TILE_SIZE),
        ))
        .insert(Transform::from_translation(from_tile(1, 5, 0.0)))
        .insert(EnemySpawner {
            spawn_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        });

    commands
        .spawn(Sprite::from_color(css::DARK_BLUE, Vec2::splat(TILE_SIZE)))
        .insert(Transform::from_translation(from_tile(12, 5, 0.0)))
        .insert(Base);

    for x_tile in -100..100 {
        for y_tile in -100..100 {
            let tile_bundle = TileBundle {
                sprite: Sprite::from_color(css::GRAY, Vec2::splat(TILE_SIZE * 0.8)),
                tile_data: TileData { empty: true },
            };

            let tile_id = commands
                .spawn(tile_bundle)
                .insert(Transform::from_translation(from_tile(
                    x_tile, y_tile, -500.0,
                )))
                .id();

            tile_map.tile_map.insert((x_tile, y_tile), tile_id);
        }
    }
}
