use crate::*;

#[derive(Resource)]
pub struct Player {
    pub money: u32,
    pub health: u32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Player {
            money: 100,
            health: 100,
        });
    }
}
