use bevy::math::FloatOrd;
use bevy::prelude::*;

use rand::Rng;

use crate::enemy::Enemy;
use crate::game_time::GameTime;
use crate::tower_types::TowerType;
use crate::utils::vec3_from_tile;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_shooting)
            .insert_resource(SelectedTower(None));
    }
}

#[derive(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_spawn_offset: Vec3,
    pub tower_type: TowerType,
    pub spread: f32,
    pub range: f32,
}

#[derive(Resource)]
pub struct SelectedTower(pub Option<TowerType>);

pub fn spawn_tower(commands: &mut Commands, position: (i32, i32), tower_type: TowerType) -> Entity {
    let (sprite, tower) = tower_type.get_tower();
    commands
        .spawn((
            sprite,
            tower,
            Transform::from_translation(vec3_from_tile(position.0, position.1, 0.0)),
        ))
        .id()
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(&mut Tower, &Transform)>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    game_time: Res<GameTime>,
) {
    for (mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(game_time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn_point = transform.translation + tower.bullet_spawn_offset;

            let closest_enemy = enemies
                .iter()
                .filter(|enemy_transform| {
                    Vec3::distance(enemy_transform.translation(), bullet_spawn_point) <= tower.range
                })
                .min_by_key(|enemy_transform| {
                    FloatOrd(Vec3::distance(
                        enemy_transform.translation(),
                        bullet_spawn_point,
                    ))
                });

            if let Some(closest_enemy) = closest_enemy {
                let direction = (closest_enemy.translation() - bullet_spawn_point).normalize();
                let spread_direction = Quat::from_rotation_z(
                    rand::thread_rng().gen_range(-tower.spread..=tower.spread),
                );

                commands.spawn((
                    tower.tower_type.get_bullet(spread_direction * direction),
                    Transform::from_translation(transform.translation + tower.bullet_spawn_offset),
                ));
            }
        }
    }
}
