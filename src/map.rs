use crate::*;

pub fn spawn_basic_scene(commands: &mut Commands) {
    commands
        .spawn(Sprite::from_color(css::FIRE_BRICK, Vec2::splat(TILE_SIZE)))
        .insert(Transform::from_xyz(150.0, 100.0, 0.0))
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            bullet_spawn_offset: Vec3 {
                x: 0.0,
                y: TILE_SIZE / 2.0,
                z: 0.1,
            },
        });

    commands
        .spawn(Sprite::from_color(
            css::DARK_GOLDENROD,
            Vec2::splat(TILE_SIZE),
        ))
        .insert(Transform::from_xyz(TILE_SIZE / 2.0, 200.0, 0.0))
        .insert(EnemySpawner {
            spawn_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        });
}
