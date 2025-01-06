use bevy::prelude::*;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameState {
            money: 1000,
            health: 100,
            game_ended: false,
        });
    }
}

#[derive(Resource)]
pub struct GameState {
    pub money: u32,
    pub health: u32,
    pub game_ended: bool,
}
