use std::time::Duration;

use crate::*;

#[derive(Component)]
pub struct EnemySpawner {
    pub power_scale: f32,
    pub waves: Vec<EnemyWave>,
    pub wave_ix: usize,
}

impl EnemySpawner {
    pub fn new(waves: Vec<EnemyWave>) -> EnemySpawner {
        EnemySpawner {
            power_scale: 1.0,
            waves,
            wave_ix: 0,
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
    pub wait_before: f32,
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
            wait_before,
            wait_between,
            wait_after,
            segment_timer: Timer::new(Duration::from_secs_f32(wait_before), TimerMode::Repeating),
        }
    }
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
    mut player: ResMut<Player>,
    enemies: Query<(), With<Enemy>>,
) {
    for (mut spawner, transform) in &mut spawners {
        let wave_ix = spawner.wave_ix;

        if wave_ix == spawner.waves.len() {
            if enemies.is_empty() {
                eprintln!("You won!");
                unimplemented!();
            }
            else {
                return;
            }
        }
        
        let wave = &mut spawner.waves[wave_ix];
        for segment in &mut wave.segments {
            segment.segment_timer.tick(time.delta());
            if segment.segment_timer.just_finished() {
                if segment.count == 0 {
                    segment.segment_timer.pause();
                    wave.segments_left -= 1;

                    if wave.segments_left == 0 {
                        player.money += wave.reward;
                        spawner.wave_ix += 1;
                        break;
                    }
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
}
