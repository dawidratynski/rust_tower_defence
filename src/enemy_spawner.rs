use crate::*;

#[derive(Component)]
pub struct EnemySpawner {
    pub spawn_timer: Timer,
}

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_spawn);
    }
}

fn enemy_spawn(
    mut commands: Commands,
    mut spawners: Query<(&mut EnemySpawner, &Transform)>,
    time: Res<Time>,
) {
    for (mut spawner, transform) in &mut spawners {
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            commands
                .spawn(Sprite::from_color(
                    css::LAWN_GREEN,
                    Vec2::splat(TILE_SIZE * 0.4),
                ))
                .insert(Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    5.0,
                ))
                .insert(Enemy {
                    speed: 50.0,
                    health: 5.0,
                });
        }
    }
}
