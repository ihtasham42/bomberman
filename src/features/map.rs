use crate::constants::TILE_SIZE;
use crate::entity;
use crate::util::tile_pos;
use bevy::prelude::*;

pub fn create_transform_from_tile_pos(x: i32, y: i32) -> Transform {
    Transform {
        translation: Vec3::new(tile_pos(x), tile_pos(y), 0.0),
        scale: Vec3::new(TILE_SIZE as f32, TILE_SIZE as f32, 1.0),
        ..Default::default()
    }
}

pub fn generate_map(commands: &mut Commands) {
    let map_size = 17;

    for y in 0..map_size {
        entity::create_wall(commands, 0, y);
    }

    for x in 1..map_size - 1 {
        entity::create_wall(commands, x, 0);
    }

    for y in 0..map_size {
        entity::create_wall(commands, map_size - 1, y);
    }

    for x in 1..map_size - 1 {
        entity::create_wall(commands, x, map_size - 1);
    }

    for y in 1..map_size - 1 {
        for x in 1..map_size - 1 {
            if x % 2 == 0 && y % 2 == 0 {
                entity::create_wall(commands, x, y)
            }
        }
    }
}
