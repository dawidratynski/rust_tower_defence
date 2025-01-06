use std::{collections::VecDeque, task::Context};

use bevy::{
    color::palettes::css,
    prelude::*,
    tasks::{futures_lite::FutureExt, AsyncComputeTaskPool, Task},
    utils::hashbrown::{HashMap, HashSet},
};
use futures::task::noop_waker_ref;

use crate::map::{EnemyNextTile, ObstacleMap, TileData, TileMap};

#[derive(Resource)]
pub struct PathfindingPromise {
    pub future: Option<Task<HashMap<(i32, i32), (i32, i32)>>>,
}

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathfindingPromise { future: None })
            .add_systems(PreUpdate, check_pathfinding_result)
            .add_systems(Startup, update_pathfinding);
    }
}

fn check_pathfinding_result(
    mut pathfinding_promise: ResMut<PathfindingPromise>,
    mut enemy_next_tile: ResMut<EnemyNextTile>,
    tiles: Query<(&mut Sprite, &mut TileData)>,
    tile_map: Res<TileMap>,
) {
    if let Some(mut future) = pathfinding_promise.future.take() {
        let mut cx = Context::from_waker(noop_waker_ref());
        match future.poll(&mut cx) {
            std::task::Poll::Ready(res) => {
                enemy_next_tile.next_tile = res;
                mark_path_tiles(tiles, enemy_next_tile, tile_map);
            }
            std::task::Poll::Pending => {
                pathfinding_promise.future = Some(future);
            }
        };
    }
}

fn mark_path_tiles(
    mut tiles: Query<(&mut Sprite, &mut TileData)>,
    enemy_next_tile: ResMut<EnemyNextTile>,
    tile_map: Res<TileMap>,
) {
    for (mut sprite, _tile) in &mut tiles {
        if sprite.color == bevy::prelude::Color::Srgba(css::LIGHT_STEEL_BLUE) {
            sprite.color = bevy::prelude::Color::Srgba(css::GRAY);
        }
    }
    let mut loc = (1, 5);
    let root = (12, 5);
    while loc != root {
        let tile_id = tile_map.tile_map.get(&loc).unwrap();
        let (mut sprite, _tile_data) = tiles.get_mut(*tile_id).unwrap();
        sprite.color = bevy::prelude::Color::Srgba(css::LIGHT_STEEL_BLUE);
        loc = enemy_next_tile[&loc];
    }
}

pub fn update_pathfinding(
    mut pathfinding_promise: ResMut<PathfindingPromise>,
    obstacle_map: Res<ObstacleMap>,
) {
    let obstacle_map_clone = obstacle_map.clone();

    let future = AsyncComputeTaskPool::get().spawn(calculate_enemy_paths(obstacle_map_clone));

    pathfinding_promise.future = Some(future);
}

async fn calculate_enemy_paths(
    obstacle_map: HashSet<(i32, i32)>,
) -> HashMap<(i32, i32), (i32, i32)> {
    let inf = 1000000;
    let mut grid = [[inf; 31]; 31];
    let mut next_tile = HashMap::new();

    let base = (12, 5);

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Prevent crash if enemy gets trapped
    for x in -15..=15 {
        for y in -15..=15 {
            next_tile.insert((x, y), base);
        }
    }

    next_tile.insert(base, base);
    grid[(base.0 + 15) as usize][(base.1 + 15) as usize] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(base);

    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;

            if !(-15..=15).contains(&nx) || !(-15..=15).contains(&ny) {
                continue;
            }

            let grid_nx = (nx + 15) as usize;
            let grid_ny = (ny + 15) as usize;

            if grid[grid_nx][grid_ny] == inf {
                grid[grid_nx][grid_ny] = grid[(x + 15) as usize][(y + 15) as usize] + 1;
                next_tile.insert((nx, ny), (x, y));
                if !obstacle_map.contains(&(nx, ny)) {
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    next_tile
}
