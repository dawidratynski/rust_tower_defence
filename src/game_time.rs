use bevy::prelude::*;

use std::time::Duration;

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime {
            scale: 1.0,
            delta_secs: 0.0,
        })
        .add_systems(PreUpdate, update_game_time);
    }
}

#[derive(Resource)]
pub struct GameTime {
    pub scale: f32,
    delta_secs: f32,
}

impl GameTime {
    pub fn delta_secs(&self) -> f32 {
        self.delta_secs * self.scale
    }

    pub fn delta(&self) -> Duration {
        Duration::from_secs_f32(self.delta_secs())
    }
}

fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.delta_secs = time.delta_secs();
}
