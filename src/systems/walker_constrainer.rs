use bevy::prelude::*;

use crate::components::{Walker, WalkerConstrainer};
use crate::features::map::closest_tile_pos;

pub fn run(mut query: Query<(&mut Transform, &mut WalkerConstrainer), With<Walker>>) {
    for (mut transform, mut walker_constrainer) in query.iter_mut() {
        let (prev_tile_x, prev_tile_y) =
            closest_tile_pos(transform.translation.x, transform.translation.y);
        let (tile_x, tile_y) = closest_tile_pos(transform.translation.x, transform.translation.y);

        if prev_tile_x == tile_x {
            if walker_constrainer.prev_x < tile_x && tile_x < transform.translation.x {
                transform.translation.x = tile_x
            }

            if transform.translation.x < tile_x && tile_x < walker_constrainer.prev_x {
                transform.translation.x = tile_x
            }
        }

        if prev_tile_y == tile_y {
            if walker_constrainer.prev_y < tile_y && tile_y < transform.translation.y {
                transform.translation.y = tile_y
            }

            if transform.translation.y < tile_y && tile_y < walker_constrainer.prev_y {
                transform.translation.y = tile_y
            }
        }

        walker_constrainer.prev_x = transform.translation.x;
        walker_constrainer.prev_y = transform.translation.y;
    }
}
