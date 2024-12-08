use crate::*;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.05, 0.05, 0.05);

const SELECTED_TOWER_BORDER: Color = Color::srgb(0.05, 0.25, 0.05);

const BUTTON_TOWER_TYPES: [tower::TowerType; 4] = [
    TowerType::Basic,
    TowerType::Minigun,
    TowerType::Piercer,
    TowerType::Sniper,
];

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, tower_button_system)
            .add_systems(Update, update_player_stats_ui);
    }
}

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct MoneyDisplay;

fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            PickingBehavior::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    PickingBehavior::IGNORE,
                ))
                .with_children(|left_column| {
                    left_column
                        .spawn((
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(Color::BLACK.with_alpha(0.5)),
                            BackgroundColor(NORMAL_BUTTON.with_alpha(0.5)),
                        ))
                        .with_child((
                            MoneyDisplay,
                            Text::new(""),
                            TextFont {
                                font_size: 33.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));

                    left_column
                        .spawn((
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(Color::BLACK.with_alpha(0.5)),
                            BackgroundColor(NORMAL_BUTTON.with_alpha(0.5)),
                        ))
                        .with_child((
                            HealthDisplay,
                            Text::new(""),
                            TextFont {
                                font_size: 33.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                });
            parent
                .spawn((
                    Node {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    PickingBehavior::IGNORE,
                ))
                .with_children(|right_column| {
                    for tower_type in BUTTON_TOWER_TYPES.iter() {
                        right_column
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(150.0),
                                    height: Val::Px(65.0),
                                    border: UiRect::all(Val::Px(5.0)),
                                    flex_direction: FlexDirection::Row,
                                    align_self: AlignSelf::FlexEnd,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BorderColor(Color::BLACK),
                                BackgroundColor(NORMAL_BUTTON),
                                *tower_type,
                            ))
                            .with_child((
                                Text::new(format!(
                                    "{:<8} {}",
                                    match tower_type {
                                        TowerType::Basic => "Basic",
                                        TowerType::Minigun => "Minigun",
                                        TowerType::Piercer => "Pierce",
                                        TowerType::Sniper => "Sniper",
                                    },
                                    tower_type.get_cost(),
                                )),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                            ));
                    }
                });
        });
}

fn update_player_stats_ui(
    player: Res<Player>,
    mut health_text: Query<&mut Text, (With<HealthDisplay>, Without<MoneyDisplay>)>,
    mut money_text: Query<&mut Text, (With<MoneyDisplay>, Without<HealthDisplay>)>,
) {
    *(health_text.single_mut()) = format!("HP {}", player.health).into();
    *(money_text.single_mut()) = format!("$  {}", player.money).into();
}

#[derive(Component)]
struct PressedButton;

fn tower_button_system(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &TowerType,
        ),
        (Changed<Interaction>, With<Button>, Without<PressedButton>),
    >,
    mut pressed_button_query: Query<
        (Entity, &mut BackgroundColor, &mut BorderColor),
        With<PressedButton>,
    >,
    mut selected_tower_opt: ResMut<SelectedTower>,
    mut commands: Commands,
) {
    for (entity, interaction, mut color, mut border_color, tower_type) in &mut interaction_query {
        if selected_tower_opt.0.is_none_or(|x| x != *tower_type) {
            match *interaction {
                Interaction::Pressed => {
                    selected_tower_opt.0 = Some(*tower_type);
                    *color = PRESSED_BUTTON.into();
                    border_color.0 = SELECTED_TOWER_BORDER;
                    for (button_entity, mut button_background, mut button_border) in &mut pressed_button_query {
                        *button_background = NORMAL_BUTTON.into();
                        button_border.0 = Color::BLACK;
                        commands.entity(button_entity).remove::<PressedButton>();
                    }
                    commands.entity(entity).insert(PressedButton);
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
}
