use crate::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub health: f32,
    pub base_damage: u32,
    pub money_for_kill: u32,
    pub path_stage: u32,
}

#[derive(Clone, Copy)]
pub enum EnemyTemplate {
    Basic,
    Strong,
    Tank,
    Fast,
    Boss,
}

impl Enemy {
    pub fn from_template(template: EnemyTemplate, power_scale: f32) -> (Enemy, Sprite) {
        match template {
            EnemyTemplate::Basic => (
                Enemy {
                    speed: 55.0,
                    health: 10.0 * power_scale,
                    base_damage: 1,
                    money_for_kill: 1,
                    path_stage: 0,
                },
                Sprite::from_color(css::MEDIUM_VIOLET_RED, Vec2::splat(TILE_SIZE * 0.4)),
            ),
            EnemyTemplate::Strong => (
                Enemy {
                    speed: 50.0,
                    health: 25.0 * power_scale,
                    base_damage: 3,
                    money_for_kill: 5,
                    path_stage: 0,
                },
                Sprite::from_color(css::DARK_MAGENTA, Vec2::splat(TILE_SIZE * 0.55)),
            ),
            EnemyTemplate::Tank => (
                Enemy {
                    speed: 40.0,
                    health: 60.0 * power_scale,
                    base_damage: 5,
                    money_for_kill: 10,
                    path_stage: 0,
                },
                Sprite::from_color(css::BLACK, Vec2::splat(TILE_SIZE * 0.7)),
            ),
            EnemyTemplate::Fast => (
                Enemy {
                    speed: 100.0,
                    health: 20.0 * power_scale,
                    base_damage: 5,
                    money_for_kill: 5,
                    path_stage: 0,
                },
                Sprite::from_color(css::DARK_SLATE_GRAY, Vec2::splat(TILE_SIZE * 0.5)),
            ),
            EnemyTemplate::Boss => (
                Enemy {
                    speed: 30.0,
                    health: 250.0 * power_scale,
                    base_damage: 100,
                    money_for_kill: 100,
                    path_stage: 0,
                },
                Sprite::from_color(css::DARK_VIOLET, Vec2::splat(TILE_SIZE * 0.9)),
            ),
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_movement)
            .add_systems(Update, enemy_death);
    }
}

fn enemy_movement(
    mut enemies: Query<(&mut Enemy, &mut Transform)>,
    path: Res<EnemyPath>,
    time: Res<Time>,
    game_time: Res<GameTime>,
) {
    for (mut enemy, mut transform) in &mut enemies {
        let movement_distance = enemy.speed * game_time.delta_secs(&time);
        let delta_to_goal = vec2_from_tile_tuple(path.nodes[enemy.path_stage as usize])
            - transform.translation.xy();

        if delta_to_goal.length() > movement_distance {
            transform.translation += delta_to_goal.normalize().extend(0.0) * movement_distance;
        } else if enemy.path_stage < path.nodes.len() as u32 - 1 {
            enemy.path_stage += 1;
        }
    }
}

fn enemy_death(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy)>,
    mut player: ResMut<Player>,
) {
    for (entity, mut enemy) in &mut enemies {
        if enemy.health <= 0.0 {
            player.money += enemy.money_for_kill;
            enemy.money_for_kill = 0;
            commands.entity(entity).insert(Despawn);
        }
    }
}
