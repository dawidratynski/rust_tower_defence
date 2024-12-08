use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct GameTime {
    pub scale: f32,
}

impl GameTime {
    pub fn delta_secs(&self, time: &Time) -> f32 {
        time.delta_secs() * self.scale
    }

    pub fn delta(&self, time: &Time) -> Duration {
        Duration::from_secs_f32(self.delta_secs(time))
    }
}

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime { scale: 1.0 });
    }
}
