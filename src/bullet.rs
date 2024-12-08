use crate::*;

#[derive(Component)]
pub struct Bullet {
    pub lifetime_timer: Timer,
    pub direction: Vec3,
    pub speed: f32,
    pub hitbox_radius: f32,
    pub damage: f32,
    pub pierce: i32,
    pub already_hit: Vec<Entity>,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_despawn)
            .add_systems(Update, bullet_movement)
            .add_systems(Update, bullet_hit);
    }
}

fn bullet_hit(
    mut commands: Commands,
    mut bullets: Query<(Entity, &GlobalTransform, &mut Bullet)>,
    mut enemies: Query<(Entity, &mut Enemy, &GlobalTransform)>,
) {
    for (bullet_entity, bullet_transform, mut bullet_data) in &mut bullets {
        for (enemy_entity, mut enemy, enemy_transform) in &mut enemies {
            if Vec3::distance(
                bullet_transform.translation(),
                enemy_transform.translation(),
            ) < bullet_data.hitbox_radius
                && !bullet_data.already_hit.contains(&enemy_entity)
            {
                enemy.health -= bullet_data.damage;
                bullet_data.already_hit.push(enemy_entity);
                bullet_data.pierce -= 1;
                if bullet_data.pierce <= 0 {
                    bullet_data.damage = 0.0;
                    commands.entity(bullet_entity).insert(Despawn);
                }
            }
        }
    }
}

fn bullet_movement(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>,
    game_time: Res<GameTime>,
) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction * bullet.speed * game_time.delta_secs(&time);
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Bullet)>,
    time: Res<Time>,
    game_time: Res<GameTime>,
) {
    for (entity, mut bullet) in &mut bullets {
        bullet.lifetime_timer.tick(game_time.delta(&time));
        if bullet.lifetime_timer.just_finished() && bullet.pierce > 0 {
            commands.entity(entity).insert(Despawn);
        }
    }
}
