use bevy::prelude::*;

use crate::{game_state::GameState, game_time::GameTime};

pub fn victory(commands: &mut Commands, game_time: &mut GameTime, game_state: &mut GameState) {
    game_time.end_game();
    game_state.game_ended = true;
    spawn_victory_screen(commands);
}

pub fn defeat(commands: &mut Commands, game_time: &mut GameTime, game_state: &mut GameState) {
    game_time.end_game();
    game_state.game_ended = true;
    game_state.health = 0;
    spawn_defeat_screen(commands);
}

fn spawn_victory_screen(commands: &mut Commands) {
    spawn_status_screen(commands, "You won!".into());
}

fn spawn_defeat_screen(commands: &mut Commands) {
    spawn_status_screen(commands, "You lost!".into());
}

fn spawn_status_screen(commands: &mut Commands, title: String) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        GlobalZIndex(100),
        BackgroundColor(Color::linear_rgba(0.9, 0.9, 0.9, 0.3)),
    ));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            GlobalZIndex(101),
        ))
        .with_child((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            flex_direction: FlexDirection::Row,
            ..default()
        },))
        .with_child((
            Text::new(title),
            TextFont::from_font_size(120.0),
            TextColor::BLACK,
            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
        ));
}
