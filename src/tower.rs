use crate::*;

#[derive(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_spawn_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_shooting);
    }
}

pub fn spawn_tower(commands: &mut Commands, position: (i32, i32)) -> Entity {
    commands
        .spawn(Sprite::from_color(css::FIRE_BRICK, Vec2::splat(TILE_SIZE)))
        .insert(Transform::from_translation(from_tile(
            position.0, position.1, 0.0,
        )))
        .insert(Tower {
            shooting_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
            bullet_spawn_offset: Vec3 {
                x: 0.0,
                y: TILE_SIZE / 2.0,
                z: 0.1,
            },
        })
        .id()
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &Transform)>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    time: Res<Time>,
) {
    for (tower_entity, mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn_point = transform.translation + tower.bullet_spawn_offset;

            let closest_enemy = enemies.iter().min_by_key(|enemy_transform| {
                FloatOrd(Vec3::distance(
                    enemy_transform.translation(),
                    bullet_spawn_point,
                ))
            });

            if let Some(closest_enemy) = closest_enemy {
                commands
                    .entity(tower_entity)
                    .with_children(|child_builder| {
                        child_builder
                            .spawn(Sprite::from_color(
                                css::BLUE_VIOLET,
                                Vec2::splat(TILE_SIZE * 0.25),
                            ))
                            .insert(Transform::from_translation(tower.bullet_spawn_offset))
                            .insert(Bullet {
                                lifetime_timer: Timer::from_seconds(2.5, TimerMode::Once),
                                direction: (closest_enemy.translation() - bullet_spawn_point)
                                    .normalize(),
                                speed: 1500.0,
                                hitbox_radius: 20.0,
                                damage: 2.5,
                                pierce: 2,
                                already_hit: vec![],
                            });
                    });
            }
        }
    }
}
