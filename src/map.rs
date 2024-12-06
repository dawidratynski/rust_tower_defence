use crate::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_basic_scene);
    }
}

fn spawn_basic_scene(mut commands: Commands) {
    commands
        .spawn(Sprite::from_color(css::FIRE_BRICK, Vec2::splat(TILE_SIZE)))
        .insert(Transform::from_xyz(7.0 * TILE_SIZE, 4.0 * TILE_SIZE, 0.0))
        .insert(Tower {
            shooting_timer: Timer::from_seconds(0.3, TimerMode::Repeating),
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
        .insert(Transform::from_xyz(1.0 * TILE_SIZE, 5.0 * TILE_SIZE, 0.0))
        .insert(EnemySpawner {
            spawn_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        });

    commands
        .spawn(Sprite::from_color(
            css::DARK_BLUE,
            Vec2::splat(TILE_SIZE),
        ))
        .insert(Transform::from_xyz(12.0 * TILE_SIZE, 5.0 * TILE_SIZE, 0.0))
        .insert(Base);

    for x_tile in -100..100 {
        for y_tile in -100..100 {
            commands
                .spawn(Sprite::from_color(css::GRAY, Vec2::splat(TILE_SIZE * 0.8)))
                .insert(Transform::from_xyz(
                    TILE_SIZE * x_tile as f32,
                    TILE_SIZE * y_tile as f32,
                    -500.0,
                ));
        }
    }
}
