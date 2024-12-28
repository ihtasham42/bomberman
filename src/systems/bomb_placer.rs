use bevy::prelude::*;

use crate::{entity, features};
use crate::components::{BombPlacer, Collider};
use crate::constants::ITEM_Z;
use crate::features::collision::are_collision_points_colliding;
use crate::features::map::closest_tile_pos;

pub fn run(
    mut commands: Commands,
    mut bomb_placer_query: Query<(&Transform, &BombPlacer)>,
    mut collider_query: Query<(Entity, &Transform), With<Collider>>,
) {
    for (transform, bomb_placer) in &mut bomb_placer_query.iter_mut() {
        if bomb_placer.wants_to_place {
            let (x, y) = closest_tile_pos(transform.translation.x, transform.translation.y);

            let mut ignore_colliders = vec![];

            for (entity, collider_transform) in collider_query.iter() {
                let bomb_transform = features::map::create_transform_from_tile_pos(x, y, ITEM_Z);

                if are_collision_points_colliding(collider_transform, &bomb_transform) {
                    ignore_colliders.push(entity);
                }
            }

            entity::create_bomb(&mut commands, x, y, ignore_colliders);
        }
    }
}
