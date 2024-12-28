use std::collections::HashSet;

use bevy::prelude::*;

use crate::components::{Bomb, Collider, Wall};
use crate::features::collision::are_collision_points_colliding;

pub fn run(
    mut bomb_query: Query<(&Transform, &mut Wall), With<Bomb>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
) {
    for (bomb_transform, mut wall) in bomb_query.iter_mut() {
        let mut entities_to_remove = HashSet::new();

        for ignored_entity in wall.ignore.iter() {
            for (colliding_entity, colliding_transform) in collider_query.iter() {
                if colliding_entity != *ignored_entity {
                    continue;
                }

                if are_collision_points_colliding(colliding_transform, bomb_transform) {
                    continue;
                }

                entities_to_remove.insert(colliding_entity.clone());
            }
        }

        wall.ignore
            .retain(|ignored_entity| !entities_to_remove.contains(ignored_entity));
    }
}
