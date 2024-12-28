use bevy::prelude::*;

use crate::components::BombPlacer;
use crate::entity;
use crate::features::map::closest_tile_pos;

pub fn run(mut commands: Commands, mut query: Query<(&Transform, &BombPlacer)>) {
    for (transform, bomb_placer) in &mut query.iter_mut() {
        if bomb_placer.wants_to_place {
            let (x, y) = closest_tile_pos(transform.translation.x, transform.translation.y);

            entity::create_bomb(&mut commands, x, y);
        }
    }
}
