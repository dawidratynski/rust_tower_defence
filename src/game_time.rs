use bevy::prelude::*;

use std::time::Duration;

const TIME_SPEEDUP_KEY: KeyCode = KeyCode::KeyC;
const PAUSE_KEY: KeyCode = KeyCode::KeyP;
const TIME_SPEEDUP_SCALE: f32 = 2.0;

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime {
            scale: 1.0,
            delta_secs: 0.0,
            paused: false,
        })
        .add_systems(Startup, setup_pause_ui)
        .add_systems(PreUpdate, update_game_time)
        .add_systems(PreUpdate, time_flow_controls)
        .add_systems(PreUpdate, handle_pause);
    }
}

#[derive(Resource)]
pub struct GameTime {
    pub scale: f32,
    delta_secs: f32,
    paused: bool,
}

impl GameTime {
    pub fn delta_secs(&self) -> f32 {
        if self.paused {
            0.0
        } else {
            self.delta_secs * self.scale
        }
    }

    pub fn delta(&self) -> Duration {
        Duration::from_secs_f32(self.delta_secs())
    }
}

fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.delta_secs = time.delta_secs();
}

fn time_flow_controls(keys: Res<ButtonInput<KeyCode>>, mut game_time: ResMut<GameTime>) {
    if keys.pressed(TIME_SPEEDUP_KEY) {
        game_time.scale = TIME_SPEEDUP_SCALE;
    } else {
        game_time.scale = 1.0;
    }
}

#[derive(Component)]
struct PauseOverlay;

fn handle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_time: ResMut<GameTime>,
    mut query: Query<&mut Node, With<PauseOverlay>>,
) {
    if keys.just_pressed(PAUSE_KEY) {
        game_time.paused = !game_time.paused;

        for mut ui_element in &mut query {
            ui_element.display = match game_time.paused {
                true => Display::Flex,
                false => Display::None,
            }
        }
    }
}

fn setup_pause_ui(mut commands: Commands) {
    commands.spawn((
        PauseOverlay,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::None,
            ..default()
        },
        GlobalZIndex(-100),
        PickingBehavior::IGNORE,
        BackgroundColor(Color::linear_rgba(0.5, 0.5, 0.5, 0.3)),
    ));

    commands
        .spawn((
            PauseOverlay,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::None,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..default()
            },
            PickingBehavior::IGNORE,
        ))
        .with_child((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(80.0),
            flex_direction: FlexDirection::Row,
            ..default()
        },))
        .with_child((
            Text::new("Paused"),
            TextFont::from_font_size(60.0),
            TextColor::BLACK,
            TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
        ));
}
