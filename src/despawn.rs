use bevy::prelude::*;

#[derive(Component)]
pub struct Despawn;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, despawn_marked);
    }
}

fn despawn_marked(mut commands: Commands, entities: Query<Entity, With<Despawn>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
