use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState {
            money: 100,
            health: 100,
        });
    }
}

#[derive(Resource)]
pub struct GameState {
    pub money: u32,
    pub health: u32,
}
