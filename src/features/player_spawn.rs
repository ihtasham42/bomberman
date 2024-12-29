use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::constants::MAP_SIZE;
use crate::entity;
use crate::features::map::{tile_pos, WallLookup};

pub fn spawn_player(commands: &mut Commands, map_lookup: &mut WallLookup) {
    let mut spawn_point_candidates = vec![];

    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let (x, y) = tile_pos(x as f32, y as f32);

            let wall_entity = map_lookup.get(x, y);

            if wall_entity.is_none() {
                spawn_point_candidates.push((x, y));
            }
        }
    }

    let mut rng = rand::thread_rng();

    if let Some((x, y)) = spawn_point_candidates.choose(&mut rng) {
        entity::create_player(commands, *x, *y);
    } else {
        panic!("No spawn point found")
    }
}
