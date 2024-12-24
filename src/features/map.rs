use bevy::prelude::Commands;

use crate::entity;

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