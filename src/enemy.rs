use crate::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub health: f32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_movement)
            .add_systems(Update, enemy_death);
    }
}

fn enemy_movement(mut enemies: Query<(&Enemy, &mut Transform)>, time: Res<Time>) {
    for (enemy, mut transform) in &mut enemies {
        transform.translation.x += enemy.speed * time.delta_secs();
    }
}

fn enemy_death(mut commands: Commands, enemies: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in &enemies {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
