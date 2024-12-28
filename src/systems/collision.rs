use bevy::prelude::*;

use crate::components::{Collider, Velocity, Wall};
use crate::constants::TILE_SIZE;
use crate::features::collision::are_collision_points_colliding;

pub fn run(
    mut colliding_query: Query<(&mut Transform, &mut Velocity), With<Collider>>,
    collided_query: Query<&Transform, (With<Wall>, Without<Collider>)>,
) {
    for (mut colliding_transform, mut velocity) in colliding_query.iter_mut() {
        for collided_transform in collided_query.iter() {
            let new_x = colliding_transform.translation.x;
            let new_y = colliding_transform.translation.y;

            let prev_x = new_x - velocity.x;
            let prev_y = new_y - velocity.y;

            if !are_collision_points_colliding(&colliding_transform, collided_transform) {
                continue;
            }

            colliding_transform.translation.x = new_x;
            colliding_transform.translation.y = prev_y;

            if are_collision_points_colliding(&colliding_transform, collided_transform) {
                if new_x >= prev_x {
                    colliding_transform.translation.x =
                        collided_transform.translation.x - TILE_SIZE;
                } else {
                    colliding_transform.translation.x =
                        collided_transform.translation.x + TILE_SIZE;
                }

                velocity.x = 0.0;
            }

            let amended_x = colliding_transform.translation.x;

            colliding_transform.translation.x = prev_x;
            colliding_transform.translation.y = new_y;

            if are_collision_points_colliding(&colliding_transform, collided_transform) {
                if new_y >= prev_y {
                    colliding_transform.translation.y =
                        collided_transform.translation.y - TILE_SIZE;
                } else {
                    colliding_transform.translation.y =
                        collided_transform.translation.y + TILE_SIZE;
                }

                velocity.y = 0.0;
            }

            let amended_y = colliding_transform.translation.y;

            colliding_transform.translation.x = amended_x;
            colliding_transform.translation.y = amended_y;
        }
    }
}
