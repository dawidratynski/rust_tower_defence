use bevy::prelude::*;

use std::time::Duration;

use crate::enemy::{Enemy, EnemyTemplate};
use crate::game_state::GameState;
use crate::game_time::GameTime;

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enemy_spawn_system);
    }
}

// This will be changed to a resource at some point, and spawnpoints will be separated out

#[derive(Component)]
pub struct EnemySpawner {
    pub waves: Vec<EnemyWave>,
    pub wave_ix: usize,
}

impl EnemySpawner {
    pub fn new(waves: Vec<EnemyWave>) -> EnemySpawner {
        EnemySpawner { waves, wave_ix: 0 }
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
) {
    commands.spawn((
        Enemy::from_template(template, power_scale),
        Transform::from_xyz(transform.translation.x, transform.translation.y, 5.0),
    ));
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut spawners: Query<(&mut EnemySpawner, &Transform)>,
    game_time: Res<GameTime>,
    mut player: ResMut<GameState>,
    enemies: Query<(), With<Enemy>>,
) {
    let (mut spawner, transform) = spawners.single_mut();
    let wave_ix = spawner.wave_ix;

    if wave_ix == spawner.waves.len() {
        eprintln!("You won!");
        unimplemented!();
    }

    let wave = &mut spawner.waves[wave_ix];

    if wave.segments_left == 0 {
        if enemies.is_empty() {
            player.money += wave.reward;
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

                spawn_enemy(&mut commands, transform, segment.template, 1.0);
            }
        }
    }
}
