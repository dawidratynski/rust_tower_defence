use crate::*;

#[derive(Component)]
pub struct Base;

pub struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, base_collision);
    }
}

fn base_collision(
    mut commands: Commands,
    bases: Query<&Transform, With<Base>>,
    mut enemies: Query<(Entity, &Transform, &mut Enemy)>,
    mut player: ResMut<Player>,
) {
    for base in &bases {
        for (enemy_entity, enemy_transfrom, mut enemy_data) in &mut enemies {
            enemy_data.health += 0.0001;
            if Vec2::distance(base.translation.xy(), enemy_transfrom.translation.xy()) < 10.0 {
                player.health -= enemy_data.base_damage;
                if player.health <= 0 {
                    eprint!("YOU LOST");
                    unimplemented!();
                }
                enemy_data.base_damage = 0;
                commands.entity(enemy_entity).insert(Despawn);
            }
        }
    }
}
