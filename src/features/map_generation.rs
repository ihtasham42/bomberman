use bevy::prelude::Commands;
use rand::Rng;

use crate::constants::{MAP_SIZE, TILE_SIZE, WOOD_CRATE_SPAWN_RATE};
use crate::entity;
use crate::features::map::{tile_pos, WallLookup};

enum WallType {
    Wall,
    Crate,
}

fn create_wall(
    commands: &mut Commands,
    wall_lookup: &mut WallLookup,
    x: i32,
    y: i32,
    wall_type: WallType,
) {
    let x = x as f32 * TILE_SIZE;
    let y = y as f32 * TILE_SIZE;

    let entity = match wall_type {
        WallType::Wall => entity::create_wall(commands, x, y),
        WallType::Crate => entity::create_wood_crate(commands, x, y),
    };

    wall_lookup.set(x, y, entity)
}

fn generate_walls(commands: &mut Commands, wall_lookup: &mut WallLookup) {
    for y in 0..MAP_SIZE {
        create_wall(commands, wall_lookup, 0, y, WallType::Wall);
    }

    for x in 1..MAP_SIZE - 1 {
        create_wall(commands, wall_lookup, x, 0, WallType::Wall);
    }

    for y in 0..MAP_SIZE {
        create_wall(commands, wall_lookup, MAP_SIZE - 1, y, WallType::Wall);
    }

    for x in 1..MAP_SIZE - 1 {
        create_wall(commands, wall_lookup, x, MAP_SIZE - 1, WallType::Wall);
    }

    for y in 1..MAP_SIZE - 1 {
        for x in 1..MAP_SIZE - 1 {
            if x % 2 == 0 && y % 2 == 0 {
                create_wall(commands, wall_lookup, x, y, WallType::Wall)
            }
        }
    }
}

fn generate_wood_crates(commands: &mut Commands, wall_lookup: &mut WallLookup) {
    for y in 1..MAP_SIZE - 1 {
        for x in 1..MAP_SIZE - 1 {
            let mut rng = rand::thread_rng();
            let random_number: f64 = rng.gen_range(0.0..1.0);

            let (tile_x, tile_y) = tile_pos(x as f32, y as f32);

            let wall_entity = wall_lookup.get(tile_x, tile_y);

            if random_number <= WOOD_CRATE_SPAWN_RATE && wall_entity.is_none() {
                create_wall(commands, wall_lookup, x, y, WallType::Crate);
            }
        }
    }
}

pub fn generate_map(commands: &mut Commands, wall_lookup: &mut WallLookup) {
    generate_walls(commands, wall_lookup);
    generate_wood_crates(commands, wall_lookup);
}
