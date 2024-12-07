use crate::*;

#[derive(Component)]
pub struct EnemySpawner {
    pub spawn_timer: Timer,
    pub power_scale: f32,
}

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_spawn_system);
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    transform: &Transform,
    template: EnemyTemplate,
    power_scale: f32,
) {
    commands
        .spawn(Enemy::from_template(template, power_scale))
        .insert(Transform::from_xyz(
            transform.translation.x,
            transform.translation.y,
            5.0,
        ));
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut spawners: Query<(&mut EnemySpawner, &Transform)>,
    time: Res<Time>,
) {
    for (mut spawner, transform) in &mut spawners {
        spawner.spawn_timer.tick(time.delta());
        if spawner.spawn_timer.just_finished() {
            spawn_enemy(
                &mut commands,
                transform,
                EnemyTemplate::Fast,
                spawner.power_scale,
            );
            spawner.power_scale *= 1.1;
        }
    }
}
