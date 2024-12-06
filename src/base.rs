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
) {
    for base in &bases {
        for (enemy_entity, enemy_transfrom, mut enemy_data) in &mut enemies {
            enemy_data.health += 0.0001;
            if Vec2::distance(base.translation.xy(), enemy_transfrom.translation.xy()) < 10.0 {
                // TODO: Decrease player health
                // Set enemy damage to 0 to prevent multiple hits
                commands.entity(enemy_entity).despawn_recursive();
            }
        }
    }
}
