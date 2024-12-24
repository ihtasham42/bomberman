use crate::constants::TILE_SIZE;

pub fn tile_pos(tile_pos: i32) -> i32 {
    tile_pos * TILE_SIZE
}