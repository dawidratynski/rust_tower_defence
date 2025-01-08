use bevy::prelude::*;

use rand::Rng;
use std::time::Duration;

use crate::enemy::{Enemy, EnemyTemplate};
use crate::game_state::GameState;
use crate::game_time::GameTime;
use crate::victory_defeat::victory;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_spawn_system);
    }
}

#[derive(Component)]
pub struct EnemySpawner {
    pub waves: Vec<EnemyWave>,
    pub wave_ix: usize,
    pub tile: (i32, i32),
}

impl EnemySpawner {
    pub fn new(waves: Vec<EnemyWave>, tile: (i32, i32)) -> EnemySpawner {
        EnemySpawner {
            waves,
            wave_ix: 0,
            tile,
        }
    }
}

pub struct EnemyWave {
    pub segments: Vec<EnemyWaveSegment>,
    pub reward: u32,
    pub segments_left: u32,
}

impl EnemyWave {
    pub fn new(segments: Vec<EnemyWaveSegment>, reward: u32) -> EnemyWave {
        EnemyWave {
            segments_left: segments.len() as u32,
            segments,
            reward,
        }
    }
}

pub struct EnemyWaveSegment {
    pub template: EnemyTemplate,
    pub count: u32,
    pub wait_between: f32,
    pub wait_after: f32,
    pub segment_timer: Timer,
}

impl EnemyWaveSegment {
    pub fn new(
        template: EnemyTemplate,
        count: u32,
        wait_before: f32,
        wait_between: f32,
        wait_after: f32,
    ) -> EnemyWaveSegment {
        EnemyWaveSegment {
            template,
            count,
            wait_between,
            wait_after,
            segment_timer: Timer::new(Duration::from_secs_f32(wait_before), TimerMode::Repeating),
        }
    }
}

fn spawn_enemy(
    commands: &mut Commands,
    transform: &Transform,
    template: EnemyTemplate,
    power_scale: f32,
    tile: (i32, i32),
) {
    commands.spawn((
        Enemy::from_template(template, power_scale, tile),
        Transform::from_xyz(transform.translation.x, transform.translation.y, 5.0),
    ));
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut spawners: Query<(&mut EnemySpawner, &Transform)>,
    mut game_time: ResMut<GameTime>,
    mut game_state: ResMut<GameState>,
    enemies: Query<(), With<Enemy>>,
) {
    if game_state.game_ended {
        return;
    }

    let (mut spawner, transform) = spawners.single_mut();
    let wave_ix = spawner.wave_ix;
    let tile = spawner.tile;

    let wave_count = spawner.waves.len();
    let wave = &mut spawner.waves[wave_ix];

    if wave.segments_left == 0 {
        if enemies.is_empty() {
            if wave_ix == wave_count - 1 {
                victory(&mut commands, &mut game_time, &mut game_state);
                return;
            }
            game_state.money += wave.reward;
            spawner.wave_ix += 1;
        }
        return;
    }

    for segment in &mut wave.segments {
        segment.segment_timer.tick(game_time.delta());
        if segment.segment_timer.just_finished() {
            if segment.count == 0 {
                segment.segment_timer.pause();
                wave.segments_left -= 1;
            } else {
                segment.count -= 1;

                if segment.count == 0 {
                    segment
                        .segment_timer
                        .set_duration(Duration::from_secs_f32(segment.wait_after));
                } else {
                    segment
                        .segment_timer
                        .set_duration(Duration::from_secs_f32(segment.wait_between));
                }

                spawn_enemy(&mut commands, transform, segment.template, 1.0, tile);
            }
        }
    }
}

pub fn random_enemy_waves(count: usize, start_points: i32, point_scale: f64) -> Vec<EnemyWave> {
    let mut rng = rand::thread_rng();
    let mut result = vec![];

    let mut points = start_points as f64;
    for _ in 0..count {
        let mut points_left = points;
        let mut segments = vec![];

        while points_left > 0.0 {
            let luck = rng.gen_range(1..=10);

            if luck <= 3 && points_left >= 400.0 {
                points_left -= 400.0;
                segments.push(EnemyWaveSegment::new(
                    EnemyTemplate::Boss,
                    40,
                    0.01,
                    0.5,
                    2.0,
                ))
            } else if luck <= 5 && points_left >= 200.0 {
                points_left -= 200.0;
                segments.push(EnemyWaveSegment::new(
                    EnemyTemplate::Fast,
                    100,
                    5.0,
                    0.2,
                    2.0,
                ))
            } else if luck <= 6 && points_left >= 100.0 {
                points_left -= 100.0;
                segments.push(EnemyWaveSegment::new(
                    EnemyTemplate::Boss,
                    10,
                    0.1,
                    2.0,
                    10.0,
                ))
            } else if luck <= 7 && points_left >= 50.0 {
                points_left -= 50.0;
                segments.push(EnemyWaveSegment::new(
                    EnemyTemplate::Tank,
                    10,
                    2.0,
                    2.0,
                    3.0,
                ))
            } else if luck <= 9 && points_left >= 30.0 {
                points_left -= 30.0;
                segments.push(EnemyWaveSegment::new(
                    EnemyTemplate::Strong,
                    10,
                    0.1,
                    1.5,
                    1.0,
                ))
            } else {
                points_left -= 10.0;
                segments.push(EnemyWaveSegment::new(
                    EnemyTemplate::Basic,
                    10,
                    5.0,
                    1.0,
                    1.0,
                ))
            }
        }

        let reward = (points / 2.0).floor() as u32;
        result.push(EnemyWave::new(segments, reward));

        points *= point_scale;
    }

    result
}
