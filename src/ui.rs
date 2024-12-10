use crate::*;

use crate::ui_config::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, update_tower_selection_buttons)
            .add_systems(Update, update_player_stats_ui);
    }
}

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct MoneyDisplay;

#[derive(Component)]
struct RoundDisplay;

#[derive(Component)]
struct SelectedTowerButton;

fn update_player_stats_ui(
    player: Res<GameState>,
    spawner: Query<&EnemySpawner>,
    mut health_text: Query<&mut Text, (With<HealthDisplay>, Without<RoundDisplay>)>,
    mut money_text: Query<&mut Text, (With<MoneyDisplay>, Without<HealthDisplay>)>,
    mut round_text: Query<&mut Text, (With<RoundDisplay>, Without<MoneyDisplay>)>,
) {
    let spawner = spawner.single();

    *(health_text.single_mut()) = format!("HP {}", player.health).into();
    *(money_text.single_mut()) = format!("$  {}", player.money).into();
    *(round_text.single_mut()) =
        format!("Round {} / {}", spawner.wave_ix + 1, spawner.waves.len()).into();
}

fn update_tower_selection_buttons(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &TowerType,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            Without<SelectedTowerButton>,
        ),
    >,
    mut selected_tower_button_query: Query<
        (Entity, &mut BackgroundColor, &mut BorderColor),
        With<SelectedTowerButton>,
    >,
    mut selected_tower_option: ResMut<SelectedTower>,
    mut commands: Commands,
) {
    for (
        button_entity,
        interaction,
        mut button_bg_color,
        mut button_border_color,
        button_tower_type,
    ) in &mut interaction_query
    {
        // Button for selected tower type should not react to hover/clicking
        if selected_tower_option
            .0
            .is_some_and(|x| x == *button_tower_type)
        {
            continue;
        }
        match *interaction {
            Interaction::Pressed => {
                selected_tower_option.0 = Some(*button_tower_type);
                button_bg_color.0 = TOWER_BUTTON_BG_SELECTED;
                button_border_color.0 = TOWER_BUTTON_BORDER_SELECTED;
                for (button_entity, mut button_background, mut button_border) in
                    &mut selected_tower_button_query
                {
                    button_background.0 = TOWER_BUTTON_BG_DEFAULT;
                    button_border.0 = TOWER_BUTTON_BORDER_DEFAULT;
                    commands
                        .entity(button_entity)
                        .remove::<SelectedTowerButton>();
                }
                commands.entity(button_entity).insert(SelectedTowerButton);
            }
            Interaction::Hovered => {
                button_bg_color.0 = TOWER_BUTTON_BG_HOVER;
                button_border_color.0 = TOWER_BUTTON_BORDER_HOVER;
            }
            Interaction::None => {
                button_bg_color.0 = TOWER_BUTTON_BG_DEFAULT;
                button_border_color.0 = TOWER_BUTTON_BORDER_DEFAULT;
            }
        }
    }
}

fn spawn_player_stats_ui(ui_parent: &mut ChildBuilder) {
    ui_parent
        .spawn((
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK.with_alpha(0.7)),
            BackgroundColor(TOWER_BUTTON_BG_DEFAULT.with_alpha(0.7)),
        ))
        .with_child((
            RoundDisplay,
            Text::new(""),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
    ui_parent
        .spawn((
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK.with_alpha(0.7)),
            BackgroundColor(TOWER_BUTTON_BG_DEFAULT.with_alpha(0.7)),
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

    ui_parent
        .spawn((
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(Color::BLACK.with_alpha(0.7)),
            BackgroundColor(TOWER_BUTTON_BG_DEFAULT.with_alpha(0.7)),
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
}

fn spawn_tower_buttons_ui(ui_parent: &mut ChildBuilder) {
    for tower_type in BUTTON_TOWER_TYPES.iter() {
        ui_parent
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
                BackgroundColor(TOWER_BUTTON_BG_DEFAULT),
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
}

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
            // Left UI Column
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
                    spawn_player_stats_ui(left_column);
                });

            // Right UI Column
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
                    spawn_tower_buttons_ui(right_column);
                });
        });
}
