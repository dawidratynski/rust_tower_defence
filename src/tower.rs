use crate::*;
use rand::Rng;

#[derive(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_spawn_offset: Vec3,
    pub tower_type: TowerType,
    pub spread: f32,
    pub range: f32,
}

#[derive(Component, Clone, Copy)]
pub enum TowerType {
    Basic,
    Sniper,
    Minigun,
    Piercer,
}

impl TowerType {
    fn get_tower(&self) -> (Sprite, Tower) {
        match self {
            TowerType::Basic => (
                Sprite::from_color(css::FIRE_BRICK, Vec2::splat(TILE_SIZE)),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_spawn_offset: Vec3::new(0.0, 0.0, 0.1),
                    tower_type: *self,
                    spread: 0.0,
                    range: 300.0,
                },
            ),
            TowerType::Sniper => (
                Sprite::from_color(css::DARK_GREEN, Vec2::splat(TILE_SIZE)),
                Tower {
                    shooting_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                    bullet_spawn_offset: Vec3::new(0.0, 0.0, 0.1),
                    tower_type: *self,
                    spread: 0.0,
                    range: 50000.0,
                },
            ),
            TowerType::Minigun => (
                Sprite::from_color(css::DARK_MAGENTA, Vec2::splat(TILE_SIZE)),
                Tower {
                    shooting_timer: Timer::from_seconds(0.01, TimerMode::Repeating),
                    bullet_spawn_offset: Vec3::new(0.0, 0.0, 0.1),
                    tower_type: *self,
                    spread: 0.2,
                    range: 300.0,
                },
            ),
            TowerType::Piercer => (
                Sprite::from_color(css::DARK_CYAN, Vec2::splat(TILE_SIZE)),
                Tower {
                    shooting_timer: Timer::from_seconds(1.2, TimerMode::Repeating),
                    bullet_spawn_offset: Vec3::new(0.0, 0.0, 0.1),
                    tower_type: *self,
                    spread: 0.0,
                    range: 1000.0,
                },
            ),
        }
    }

    fn get_bullet(&self, direction: Vec3) -> (Sprite, Bullet) {
        match self {
            TowerType::Basic => (
                Sprite::from_color(css::BLUE_VIOLET, Vec2::splat(TILE_SIZE * 0.25)),
                Bullet {
                    lifetime_timer: Timer::from_seconds(2.5, TimerMode::Once),
                    direction,
                    speed: 1500.0,
                    hitbox_radius: 20.0,
                    damage: 5.0,
                    pierce: 1,
                    already_hit: vec![],
                },
            ),
            TowerType::Sniper => (
                Sprite::from_color(css::BLUE_VIOLET, Vec2::splat(TILE_SIZE * 0.25)),
                Bullet {
                    lifetime_timer: Timer::from_seconds(2.5, TimerMode::Once),
                    direction,
                    speed: 3000.0,
                    hitbox_radius: 50.0,
                    damage: 50.0,
                    pierce: 1,
                    already_hit: vec![],
                },
            ),
            TowerType::Minigun => (
                Sprite::from_color(css::BLUE_VIOLET, Vec2::splat(TILE_SIZE * 0.25)),
                Bullet {
                    lifetime_timer: Timer::from_seconds(2.5, TimerMode::Once),
                    direction,
                    speed: 1000.0,
                    hitbox_radius: 20.0,
                    damage: 1.0,
                    pierce: 1,
                    already_hit: vec![],
                },
            ),
            TowerType::Piercer => (
                Sprite::from_color(css::BLUE_VIOLET, Vec2::splat(TILE_SIZE * 0.25)),
                Bullet {
                    lifetime_timer: Timer::from_seconds(10.0, TimerMode::Once),
                    direction,
                    speed: 500.0,
                    hitbox_radius: 50.0,
                    damage: 5.0,
                    pierce: 10,
                    already_hit: vec![],
                },
            ),
        }
    }

    pub fn get_cost(&self) -> u32 {
        match self {
            TowerType::Basic => 10,
            TowerType::Sniper => 30,
            TowerType::Minigun => 50,
            TowerType::Piercer => 15,
        }
    }
}

#[derive(Resource)]
pub struct SelectedTower(pub TowerType);

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_shooting)
            .insert_resource(SelectedTower(TowerType::Basic));
    }
}

pub fn spawn_tower(commands: &mut Commands, position: (i32, i32), tower_type: TowerType) -> Entity {
    let (sprite, tower) = tower_type.get_tower();
    commands
        .spawn((sprite, tower))
        .insert(Transform::from_translation(vec3_from_tile(
            position.0, position.1, 0.0,
        )))
        .id()
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(&mut Tower, &Transform)>,
    enemies: Query<&GlobalTransform, With<Enemy>>,
    time: Res<Time>,
) {
    for (mut tower, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
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

                commands
                    .spawn(tower.tower_type.get_bullet(spread_direction * direction))
                    .insert(Transform::from_translation(
                        transform.translation + tower.bullet_spawn_offset,
                    ));
            }
        }
    }
}
