use crate::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub health: f32,
    pub base_damage: u32,
    pub money_for_kill: u32,
    pub path_stage: u32,
}

#[derive(Resource)]
pub struct EnemyPath {
    nodes: Vec<Vec2>
} 

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_movement)
            .add_systems(Update, enemy_death)
            .insert_resource(EnemyPath {
                nodes: vec![
                    vec2_from_tile(1, 0),
                    vec2_from_tile(5, 0),
                    vec2_from_tile(5, 5),
                    vec2_from_tile(12, 5)
                ]
            });
    }
}

fn enemy_movement(
    mut enemies: Query<(&mut Enemy, &mut Transform)>, 
    path: Res<EnemyPath>,
    time: Res<Time>
) {
    for (mut enemy, mut transform) in &mut enemies {
        let movement_distance = enemy.speed * time.delta_secs();
        let delta_to_goal = path.nodes[enemy.path_stage as usize] - transform.translation.xy();

        if delta_to_goal.length() > movement_distance {
            transform.translation += delta_to_goal.normalize().extend(0.0) * movement_distance;
        }
        else if enemy.path_stage < path.nodes.len() as u32 - 1 {
            enemy.path_stage += 1;
        }
    }
}

fn enemy_death(mut commands: Commands, mut enemies: Query<(Entity, &mut Enemy)>, mut player: ResMut<Player>) {
    for (entity, mut enemy) in &mut enemies {
        if enemy.health <= 0.0 {
            player.money += enemy.money_for_kill;
            enemy.money_for_kill = 0;
            commands.entity(entity).insert(Despawn);
        }
    }
}
