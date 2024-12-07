use crate::*;

pub fn from_tile(x: i32, y: i32, visibility: f32) -> Vec3 {
    Vec3 {
        x: TILE_SIZE * x as f32,
        y: TILE_SIZE * y as f32,
        z: visibility,
    }
}

pub fn get_tile(x: f32, y: f32) -> (i32, i32) {
    (
        (x / TILE_SIZE + 0.5).floor() as i32,
        (y / TILE_SIZE + 0.5).floor() as i32,
    )
}
