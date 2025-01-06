use std::task::Context;

use bevy::{
    prelude::*,
    tasks::{futures_lite::FutureExt, AsyncComputeTaskPool, Task},
    utils::hashbrown::{HashMap, HashSet},
};
use futures::task::noop_waker_ref;

use crate::map::{EnemyNextTile, ObstacleMap};

#[derive(Resource)]
pub struct PathfindingPromise {
    pub future: Option<Task<HashMap<(i32, i32), (i32, i32)>>>,
}

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathfindingPromise { future: None })
            .add_systems(PreUpdate, check_pathfinding_result);
    }
}

fn check_pathfinding_result(
    mut pathfinding_promise: ResMut<PathfindingPromise>,
    mut enemy_next_tile: ResMut<EnemyNextTile>,
) {
    if let Some(mut future) = pathfinding_promise.future.take() {
        let mut cx = Context::from_waker(noop_waker_ref());
        match future.poll(&mut cx) {
            std::task::Poll::Ready(res) => {
                enemy_next_tile.next_tile = res;
            }
            std::task::Poll::Pending => {
                pathfinding_promise.future = Some(future);
            }
        };
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

// Calculate-heavy task
async fn calculate_enemy_paths(
    _obstacle_map: HashSet<(i32, i32)>,
) -> HashMap<(i32, i32), (i32, i32)> {
    let next_tile = HashMap::new();
    let mut x = 7;
    for i in 1..1000000000 {
        x = (x * i) % 17;
    }
    next_tile
}
