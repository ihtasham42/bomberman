use bevy::prelude::*;

use crate::{entity, features};
use crate::components::{Bomb, BombPlacer, Collider, PowerupStats};
use crate::constants::ITEM_Z;
use crate::features::collision::are_collision_points_colliding;
use crate::features::map::closest_tile_pos;

pub fn run(
    mut commands: Commands,
    mut bomb_placer_query: Query<(Entity, &Transform, &mut BombPlacer, &mut PowerupStats)>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    existing_bomb_query: Query<&Transform, With<Bomb>>,
) {
    for (bomb_placer_entity, transform, mut bomb_placer, mut powerup_stats) in
        &mut bomb_placer_query.iter_mut()
    {
        if bomb_placer.wants_to_place && powerup_stats.current_bombs > 0 {
            let (x, y) = closest_tile_pos(transform.translation.x, transform.translation.y);

            let bomb_transform = features::map::create_transform_from_tile_pos(x, y, ITEM_Z);

            if existing_bomb_query
                .iter()
                .find(|existing_bomb_transform| {
                    existing_bomb_transform.translation.x == bomb_transform.translation.x
                        && existing_bomb_transform.translation.y == bomb_transform.translation.y
                })
                .is_some()
            {
                continue;
            }

            let mut ignore_colliders = vec![];

            for (entity, collider_transform) in collider_query.iter() {
                if are_collision_points_colliding(collider_transform, &bomb_transform) {
                    ignore_colliders.push(entity);
                }
            }

            powerup_stats.current_bombs -= 1;

            bomb_placer.wants_to_place = false;

            entity::create_bomb(
                &mut commands,
                x,
                y,
                ignore_colliders,
                powerup_stats.bomb_power,
                bomb_placer_entity,
            );
        }
    }
}
