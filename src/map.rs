use bevy::utils::hashbrown::{HashMap, HashSet};
use std::f32::consts::PI;

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

#[derive(Resource)]
pub struct EnemyPath {
    pub nodes: Vec<(i32, i32)>,
}

impl EnemyPath {
    fn tiles_on_path(&self) -> HashSet<(i32, i32)> {
        let mut tiles = HashSet::new();

        for window in self.nodes.windows(2) {
            if let [start, end] = window {
                if start.0 == end.0 {
                    for y in start.1.min(end.1)..=end.1.max(start.1) {
                        tiles.insert((start.0, y));
                    }
                } else if start.1 == end.1 {
                    for x in start.0.min(end.0)..=end.0.max(start.0) {
                        tiles.insert((x, start.1));
                    }
                } else {
                    panic!("Invalid path segment: path must be horizontal or vertical");
                }
            }
        }
        tiles
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap {
            tile_map: HashMap::new(),
        })
        .insert_resource(EnemyPath {
            nodes: vec![(1, 5), (1, 0), (5, 0), (5, 5), (12, 5)],
        })
        .add_systems(Startup, spawn_basic_scene);
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut tile_map: ResMut<TileMap>,
    enemy_path: Res<EnemyPath>,
) {
    commands
        .spawn(Sprite::from_color(
            css::DARK_RED,
            Vec2::splat(TILE_SIZE * 0.6),
        ))
        .insert(
            Transform::from_translation(vec3_from_tile(1, 5, 0.0))
                .with_rotation(Quat::from_rotation_z(PI / 4.0)),
        )
        .insert(EnemySpawner::new(vec![
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
                    EnemyWaveSegment::new(EnemyTemplate::Fast, 50, 0.5, 0.5, 10.0),
                    EnemyWaveSegment::new(EnemyTemplate::Boss, 3, 5.0, 5.0, 10.0),
                ],
                50,
            ),
        ]));

    commands
        .spawn(Sprite::from_color(css::DARK_BLUE, Vec2::splat(TILE_SIZE)))
        .insert(Transform::from_translation(vec3_from_tile(12, 5, 0.0)))
        .insert(Base);

    let enemy_path_tiles = enemy_path.tiles_on_path();

    for x_tile in -100..100 {
        for y_tile in -100..100 {
            let tile_bundle = if enemy_path_tiles.contains(&(x_tile, y_tile)) {
                TileBundle {
                    sprite: Sprite::from_color(css::STEEL_BLUE, Vec2::splat(TILE_SIZE * 1.0)),
                    tile_data: TileData { empty: false },
                }
            } else {
                TileBundle {
                    sprite: Sprite::from_color(css::GRAY, Vec2::splat(TILE_SIZE * 0.8)),
                    tile_data: TileData { empty: true },
                }
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
