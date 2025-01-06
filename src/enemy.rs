use bevy::color::palettes::css;
use bevy::prelude::*;

use crate::despawn::Despawn;
use crate::game_config::TILE_SIZE;
use crate::game_state::GameState;
use crate::game_time::GameTime;
use crate::map::EnemyNextTile;
use crate::status_effect::CanHaveStatusEffects;
use crate::utils::vec2_from_tile_tuple;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_movement)
            .add_systems(Update, enemy_death)
            .add_systems(PreUpdate, apply_status_effects);
    }
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub health: f32,
    pub base_damage: u32,
    pub money_for_kill: u32,
    pub current_tile: (i32, i32),
    pub next_tile: (i32, i32),
}

#[allow(dead_code)] // Unused enemy templates are ok
#[derive(Clone, Copy)]
pub enum EnemyTemplate {
    Basic,
    Strong,
    Tank,
    Fast,
    Boss,
}

impl Enemy {
    pub fn from_template(
        template: EnemyTemplate,
        power_scale: f32,
        tile: (i32, i32),
    ) -> (Enemy, Sprite, CanHaveStatusEffects<Enemy>) {
        match template {
            EnemyTemplate::Basic => (
                Enemy {
                    speed: 55.0,
                    health: 10.0 * power_scale,
                    base_damage: 1,
                    money_for_kill: 1,
                    current_tile: tile,
                    next_tile: tile,
                },
                Sprite::from_color(css::MEDIUM_VIOLET_RED, Vec2::splat(TILE_SIZE * 0.4)),
                CanHaveStatusEffects::new(),
            ),
            EnemyTemplate::Strong => (
                Enemy {
                    speed: 50.0,
                    health: 25.0 * power_scale,
                    base_damage: 3,
                    money_for_kill: 5,
                    current_tile: tile,
                    next_tile: tile,
                },
                Sprite::from_color(css::DARK_MAGENTA, Vec2::splat(TILE_SIZE * 0.55)),
                CanHaveStatusEffects::new(),
            ),
            EnemyTemplate::Tank => (
                Enemy {
                    speed: 40.0,
                    health: 60.0 * power_scale,
                    base_damage: 5,
                    money_for_kill: 10,
                    current_tile: tile,
                    next_tile: tile,
                },
                Sprite::from_color(css::BLACK, Vec2::splat(TILE_SIZE * 0.7)),
                CanHaveStatusEffects::new(),
            ),
            EnemyTemplate::Fast => (
                Enemy {
                    speed: 100.0,
                    health: 20.0 * power_scale,
                    base_damage: 5,
                    money_for_kill: 5,
                    current_tile: tile,
                    next_tile: tile,
                },
                Sprite::from_color(css::DARK_SLATE_GRAY, Vec2::splat(TILE_SIZE * 0.5)),
                CanHaveStatusEffects::new(),
            ),
            EnemyTemplate::Boss => (
                Enemy {
                    speed: 30.0,
                    health: 250.0 * power_scale,
                    base_damage: 100,
                    money_for_kill: 100,
                    current_tile: tile,
                    next_tile: tile,
                },
                Sprite::from_color(css::DARK_VIOLET, Vec2::splat(TILE_SIZE * 0.9)),
                CanHaveStatusEffects::new(),
            ),
        }
    }
}

fn enemy_movement(
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
    next_tile: Res<EnemyNextTile>,
    game_time: Res<GameTime>,
) {
    for (mut enemy, mut transform) in &mut enemies {
        let movement_distance = enemy.speed * game_time.delta_secs();
        let goal = vec2_from_tile_tuple(enemy.next_tile);
        let delta_to_goal = goal - transform.translation.xy();

        if delta_to_goal.length() > movement_distance {
            transform.translation += delta_to_goal.normalize().extend(0.0) * movement_distance;
        } else {
            enemy.next_tile = next_tile[&enemy.current_tile];
            enemy.current_tile = enemy.next_tile;
        }
    }
}

fn enemy_death(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy)>,
    mut game_state: ResMut<GameState>,
) {
    for (entity, mut enemy) in &mut enemies {
        if enemy.health <= 0.0 {
            game_state.money += enemy.money_for_kill;
            enemy.money_for_kill = 0;
            commands.entity(entity).insert(Despawn);
        }
    }
}

fn apply_status_effects(
    mut enemies: Query<(&mut Enemy, &mut CanHaveStatusEffects<Enemy>)>,
    game_time: Res<GameTime>,
) {
    for (mut enemy, mut status_effects) in &mut enemies {
        for status in &mut status_effects.status_effects {
            if status.duration <= 0.0 {
                // Expired status
                continue;
            }
            if !status.already_applied {
                (status.apply)(&mut enemy);
                status.already_applied = true;
                continue;
            }

            status.duration -= game_time.delta_secs();
            if status.duration <= 0.0 {
                (status.finish)(&mut enemy);
            } else {
                (status.tick)(&mut enemy, game_time.delta_secs());
            }
        }
    }
}
