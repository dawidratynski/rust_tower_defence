use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::utils::hashbrown::{HashMap, HashSet};

use std::f32::consts::PI;

use crate::enemy::EnemyTemplate;
use crate::enemy_spawner::{EnemySpawner, EnemyWave, EnemyWaveSegment};
use crate::game_config::TILE_SIZE;
use crate::player_base::PlayerBase;
use crate::utils::vec3_from_tile;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap {
            tile_map: HashMap::new(),
        })
        .insert_resource(EnemyNextTile {
            next_tile: HashMap::new(),
        })
        .insert_resource(ObstacleMap {
            tile_map: HashSet::new(),
        })
        .add_systems(Startup, spawn_basic_scene);
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub sprite: Sprite,
    pub tile_data: TileData,
}

#[derive(Component)]
pub struct TileData {
    pub empty: bool,
    pub prepared: bool,
}

#[derive(Resource)]
pub struct TileMap {
    pub tile_map: HashMap<(i32, i32), Entity>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ObstacleMap {
    pub tile_map: HashSet<(i32, i32)>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct EnemyNextTile {
    pub next_tile: HashMap<(i32, i32), (i32, i32)>,
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut tile_map: ResMut<TileMap>,
    mut obstacle_map: ResMut<ObstacleMap>,
    mut enemy_next_tile: ResMut<EnemyNextTile>,
) {
    commands.spawn((
        Sprite::from_color(css::DARK_RED, Vec2::splat(TILE_SIZE * 0.6)),
        Transform::from_translation(vec3_from_tile(1, 5, 0.0))
            .with_rotation(Quat::from_rotation_z(PI / 4.0)),
        EnemySpawner::new(
            vec![
                EnemyWave::new(
                    vec![EnemyWaveSegment::new(
                        EnemyTemplate::Basic,
                        10,
                        0.5,
                        1.0,
                        3.0,
                    )],
                    50,
                ),
                EnemyWave::new(
                    vec![
                        EnemyWaveSegment::new(EnemyTemplate::Basic, 10, 0.5, 2.0, 1.0),
                        EnemyWaveSegment::new(EnemyTemplate::Strong, 3, 3.0, 4.0, 1.0),
                    ],
                    50,
                ),
                EnemyWave::new(
                    vec![
                        EnemyWaveSegment::new(EnemyTemplate::Fast, 50, 0.5, 0.5, 0.1),
                        EnemyWaveSegment::new(EnemyTemplate::Boss, 3, 5.0, 5.0, 0.1),
                    ],
                    50,
                ),
            ],
            (1, 5),
        ),
    ));

    obstacle_map.insert((1, 5));
    obstacle_map.insert((12, 5));

    commands.spawn((
        Sprite::from_color(css::DARK_BLUE, Vec2::splat(TILE_SIZE)),
        Transform::from_translation(vec3_from_tile(12, 5, 0.0)),
        PlayerBase,
    ));

    for x_tile in -15..15 {
        for y_tile in -15..15 {
            enemy_next_tile
                .next_tile
                .insert((x_tile, y_tile), (x_tile + 1, y_tile));
            let tile_bundle = TileBundle {
                sprite: Sprite::from_color(css::GRAY, Vec2::splat(TILE_SIZE * 0.8)),
                tile_data: TileData {
                    empty: true,
                    prepared: false,
                },
            };

            let tile_id = commands
                .spawn(tile_bundle)
                .insert(Transform::from_translation(vec3_from_tile(
                    x_tile, y_tile, -500.0,
                )))
                .id();

            tile_map.tile_map.insert((x_tile, y_tile), tile_id);
        }
    }
}
