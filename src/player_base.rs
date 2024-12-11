use bevy::prelude::*;

use crate::despawn::Despawn;
use crate::enemy::Enemy;
use crate::game_state::GameState;

pub struct PlayerBasePlugin;

impl Plugin for PlayerBasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, base_collision);
    }
}

#[derive(Component)]
pub struct PlayerBase;

fn base_collision(
    mut commands: Commands,
    bases: Query<&Transform, With<PlayerBase>>,
    mut enemies: Query<(Entity, &Transform, &mut Enemy)>,
    mut game_state: ResMut<GameState>,
) {
    for base in &bases {
        for (enemy_entity, enemy_transfrom, mut enemy_data) in &mut enemies {
            if Vec2::distance(base.translation.xy(), enemy_transfrom.translation.xy()) < 10.0 {
                game_state.health -= enemy_data.base_damage;
                if game_state.health == 0 {
                    eprint!("YOU LOST");
                    unimplemented!();
                }
                enemy_data.base_damage = 0;
                commands.entity(enemy_entity).insert(Despawn);
            }
        }
    }
}
