use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::components::{Destroyable, Wall};
use crate::constants::MAP_SIZE;
use crate::entity;
use crate::features::map::{destroy_wall, get_direction_deltas, tile_pos, WallLookup};

pub fn spawn_player(
    commands: &mut Commands,
    wall_lookup: &mut WallLookup,
    destroyable_query: Query<Entity, (With<Wall>, With<Destroyable>)>,
) {
    let mut spawn_point_candidates = vec![];

    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let (x, y) = tile_pos(x as f32, y as f32);

            let wall_entity = wall_lookup.get(x, y);

            if let Some(wall_entity) = wall_entity {
                if let Err(_) = destroyable_query.get(*wall_entity) {
                    continue;
                }
            }

            spawn_point_candidates.push((x, y));
        }
    }

    let mut rng = rand::thread_rng();

    if let Some((x, y)) = spawn_point_candidates.choose(&mut rng) {
        destroy_surrounding_destroyables(commands, wall_lookup, *x, *y, destroyable_query);
        entity::create_player(commands, *x, *y);
    } else {
        panic!("No spawn point found")
    }
}

fn destroy_surrounding_destroyables(
    commands: &mut Commands,
    wall_lookup: &mut WallLookup,
    x: f32,
    y: f32,
    destroyable_query: Query<Entity, (With<Wall>, With<Destroyable>)>,
) {
    let direction_deltas = get_direction_deltas();

    let mut destroyable_positions = vec![(x, y)];

    for (dx, dy) in direction_deltas {
        let fx = x + dx;
        let fy = y + dy;

        destroyable_positions.push((fx, fy));
    }

    for (x, y) in destroyable_positions {
        let wall_entity = wall_lookup.get(x, y);

        if let Some(wall_entity) = wall_entity {
            if let Ok(_) = destroyable_query.get(*wall_entity) {
                destroy_wall(commands, wall_lookup, x, y, *wall_entity);
            }
        }
    }
}
