use bevy::prelude::*;

use crate::despawn::Despawn;
use crate::enemy::Enemy;
use crate::game_time::GameTime;
use crate::status_effect::{CanHaveStatusEffects, StatusBuilder};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_despawn)
            .add_systems(Update, bullet_movement)
            .add_systems(Update, bullet_hit);
    }
}

#[derive(Component)]
pub struct Bullet {
    pub lifetime_timer: Timer,
    pub direction: Vec3,
    pub speed: f32,
    pub hitbox_radius: f32,
    pub damage: f32,
    pub pierce: i32,
    pub already_hit: Vec<Entity>,
    pub status_effects: Vec<StatusBuilder<Enemy>>,
}

fn bullet_hit(
    mut commands: Commands,
    mut bullets: Query<(Entity, &GlobalTransform, &mut Bullet)>,
    mut enemies: Query<(
        Entity,
        &mut Enemy,
        &mut CanHaveStatusEffects<Enemy>,
        &GlobalTransform,
    )>,
) {
    for (bullet_entity, bullet_transform, mut bullet_data) in &mut bullets {
        for (enemy_entity, mut enemy, mut status_effects, enemy_transform) in &mut enemies {
            if Vec2::distance(
                bullet_transform.translation().xy(),
                enemy_transform.translation().xy(),
            ) < bullet_data.hitbox_radius
                && !bullet_data.already_hit.contains(&enemy_entity)
                && bullet_data.pierce > 0
            {
                bullet_data.pierce -= 1;
                bullet_data.already_hit.push(enemy_entity);
                enemy.health -= bullet_data.damage;
                if bullet_data.pierce <= 0 {
                    commands.entity(bullet_entity).insert(Despawn);
                }
                for status in &bullet_data.status_effects {
                    status_effects.status_effects.push(status());
                }
            }
        }
    }
}

fn bullet_movement(mut bullets: Query<(&Bullet, &mut Transform)>, game_time: Res<GameTime>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction * bullet.speed * game_time.delta_secs();
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet)>,
    game_time: Res<GameTime>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.lifetime_timer.tick(game_time.delta());
        if bullet.lifetime_timer.just_finished() && bullet.pierce > 0 {
            commands.entity(entity).insert(Despawn);
        }
    }
}
