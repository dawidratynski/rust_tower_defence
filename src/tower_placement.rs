use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game_config::*;
use crate::game_state::GameState;
use crate::map::ObstacleMap;
use crate::map::TileData;
use crate::map::TileMap;
use crate::pathfinding::update_pathfinding;
use crate::pathfinding::PathfindingPromise;
use crate::tower::spawn_tower;
use crate::tower::SelectedTower;
use crate::utils::get_tile;

pub struct TowerPlacementPlugin;

impl Plugin for TowerPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_placement_system);
    }
}

// Many arguments are needed, as we need to access a lot of data
#[allow(clippy::too_many_arguments)]
fn tower_placement_system(
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    mut tiles: Query<(&mut Sprite, &mut TileData)>,
    interaction_query: Query<(&Interaction,), (Changed<Interaction>, With<Button>)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    tile_map: Res<TileMap>,
    selected_tower_opt: Res<SelectedTower>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    mut obstacles: ResMut<ObstacleMap>,
    pathfinding_promise: ResMut<PathfindingPromise>,
) {
    let Some(selected_tower) = selected_tower_opt.0 else {
        return;
    };

    if !mouse_button_input.just_pressed(MouseButton::Left)
        // This prevents placing towers under buttons
        || !interaction_query.is_empty()
        || selected_tower.get_cost() > game_state.money
        || game_state.game_ended
    {
        return;
    }

    let (camera_transform, projection) = camera.single();

    if let Some(cursor_position) = primary_window_query.single().cursor_position() {
        let cursor_position_global_axis = Vec2 {
            x: cursor_position.x,
            y: -cursor_position.y,
        };

        let root_to_camera_offset = camera_transform.translation.xy();
        let camera_to_cursor_offset = projection.scale
            * (Vec2 {
                x: -WINDOW_WIDTH / 2.0,
                y: WINDOW_HEIGHT / 2.0,
            } + cursor_position_global_axis);

        let game_cursor_position = root_to_camera_offset
            + (camera_transform.rotation * Vec3::from((camera_to_cursor_offset, 0.0))).xy();

        let tile_position = get_tile(game_cursor_position.x, game_cursor_position.y);
        let tile_id_opt = tile_map.tile_map.get(&tile_position);

        if let Some(tile_id) = tile_id_opt {
            if let Ok((mut sprite, mut tile_data)) = tiles.get_mut(*tile_id) {
                if tile_data.empty && !tile_data.prepared && game_state.money >= 10 {
                    sprite.color = bevy::prelude::Color::Srgba(css::DARK_SLATE_GRAY);
                    tile_data.prepared = true;
                    game_state.money -= 10;
                    obstacles.insert(tile_position);
                    update_pathfinding(pathfinding_promise, obstacles.into());
                } else if tile_data.empty
                    && tile_data.prepared
                    && game_state.money >= selected_tower.get_cost()
                {
                    sprite.color = bevy::prelude::Color::Srgba(css::AZURE);
                    tile_data.empty = false;
                    game_state.money -= selected_tower.get_cost();
                    spawn_tower(&mut commands, tile_position, selected_tower);
                }
            }
        }
    }
}
