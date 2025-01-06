use bevy::prelude::*;

use crate::{game_state::GameState, game_time::GameTime};

pub fn victory(_commands: &mut Commands, game_time: &mut GameTime, game_state: &mut GameState) {
    game_time.end_game();
    game_state.game_ended = true;
    // unimplemented!();
}

pub fn defeat(_commands: &mut Commands, game_time: &mut GameTime, game_state: &mut GameState) {
    game_time.end_game();
    game_state.game_ended = true;
    game_state.health = 0;
    // unimplemented!();
}
