use crate::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const BUTTON_TOWER_TYPES: [tower::TowerType; 4] = [TowerType::Basic, TowerType::Minigun, TowerType::Piercer, TowerType::Sniper];

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, tower_button_system);
    }
}

fn spawn_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        })
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {
            for tower_type in BUTTON_TOWER_TYPES.iter() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            align_self: AlignSelf::FlexEnd,
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor(Color::BLACK),
                        BorderRadius::ZERO,
                        BackgroundColor(NORMAL_BUTTON),
                        *tower_type,
                    ))
                    .with_child((
                        Text::new(
                            match tower_type {
                                TowerType::Basic => "Basic",
                                TowerType::Minigun => "Minigun",
                                TowerType::Piercer => "Pierce",
                                TowerType::Sniper => "Sniper",
                            }
                        ),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
            }
        });
}

fn tower_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &TowerType,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected_tower: ResMut<SelectedTower>,
) {
    for (interaction, mut color, mut border_color, tower_type) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = css::RED.into();
                selected_tower.0 = *tower_type;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
