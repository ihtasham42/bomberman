use bevy::prelude::*;

use crate::constants::TILE_SIZE;
use crate::entity;

pub fn create_transform_from_tile_pos(x: f32, y: f32, z: f32) -> Transform {
    Transform {
        translation: Vec3::new(x, y, z),
        scale: Vec3::new(TILE_SIZE, TILE_SIZE, 1.0),
        ..Default::default()
    }
}

fn create_wall(commands: &mut Commands, x: i32, y: i32) {
    entity::create_wall(commands, x as f32 * TILE_SIZE, y as f32 * TILE_SIZE);
}

pub fn generate_map(commands: &mut Commands) {
    let map_size = 17;

    for y in 0..map_size {
        create_wall(commands, 0, y);
    }

    for x in 1..map_size - 1 {
        create_wall(commands, x, 0);
    }

    for y in 0..map_size {
        create_wall(commands, map_size - 1, y);
    }

    for x in 1..map_size - 1 {
        create_wall(commands, x, map_size - 1);
    }

    for y in 1..map_size - 1 {
        for x in 1..map_size - 1 {
            if x % 2 == 0 && y % 2 == 0 {
                create_wall(commands, x, y)
            }
        }
    }
}

pub fn tile_pos(x: f32, y: f32) -> (f32, f32) {
    (x * TILE_SIZE, y * TILE_SIZE)
}

pub fn closest_tile_pos(x: f32, y: f32) -> (f32, f32) {
    (
        (x / TILE_SIZE).round() * TILE_SIZE,
        (y / TILE_SIZE).round() * TILE_SIZE,
    )
}
