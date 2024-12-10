pub const GAME_WINDOW_TITLE: &str = "Game prototye v1";

pub const SPRITE_RESOLUTION: u16 = 16;
pub const SPRITE_SCALE: u16 = 4;
pub const TILE_SIZE: f32 = SPRITE_RESOLUTION as f32 * SPRITE_SCALE as f32;

pub const MAP_HEIGHT_IN_TILES: u32 = 8;
pub const MAP_WIDTH_IN_TILES: u32 = 12;

pub const WINDOW_HEIGHT: f32 = MAP_HEIGHT_IN_TILES as f32 * TILE_SIZE;
pub const WINDOW_WIDTH: f32 = MAP_WIDTH_IN_TILES as f32 * TILE_SIZE;
