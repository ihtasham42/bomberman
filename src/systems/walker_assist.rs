use bevy::prelude::*;

use crate::components::{Collider, Velocity, Walker, WalkerAssist, Wall};
use crate::constants::TILE_SIZE;
use crate::features::collision::get_walls_colliding_with_collision_points;
use crate::features::map::{closest_tile_pos, WallLookup};
use crate::features::movement::{Direction, DirectionAxis};

pub fn run(
    wall_lookup: Res<WallLookup>,
    mut walker_query: Query<
        (&Transform, &mut Walker, &Velocity),
        (With<Collider>, With<WalkerAssist>),
    >,
    wall_query: Query<&Transform, (With<Wall>, Without<Collider>)>,
) {
    for (walker_transform, mut walker, velocity) in walker_query.iter_mut() {
        let (tile_x, tile_y) = closest_tile_pos(
            walker_transform.translation.x,
            walker_transform.translation.y,
        );

        if tile_x == walker_transform.translation.x && tile_y == walker_transform.translation.y {
            continue;
        }

        let exclusive_direction = match (&walker.horizontal_direction, &walker.vertical_direction) {
            (Some(direction), None) => Some(direction),
            (None, Some(direction)) => Some(direction),
            _ => None,
        };

        let Some(exclusive_direction) = exclusive_direction else {
            continue;
        };

        let direction_axis = DirectionAxis::from(exclusive_direction.clone());

        let (dx, dy) = exclusive_direction.to_delta();

        let mut ghost_transform = walker_transform.clone();

        ghost_transform.translation.x += dx / 2.0;
        ghost_transform.translation.y += dy / 2.0;

        let wall_entities =
            get_walls_colliding_with_collision_points(&ghost_transform, &wall_lookup);

        if wall_entities.len() == 0 || wall_entities.len() >= 2 {
            continue;
        }

        let wall_entity = *wall_entities
            .first()
            .expect("Only 1 wall entity should exist");

        let Ok(wall_transform) = wall_query.get(wall_entity) else {
            continue;
        };

        let slip_factor = 3.0;

        match direction_axis {
            DirectionAxis::Horizontal => {
                let dy = walker_transform.translation.y - wall_transform.translation.y;

                if dy.round().abs() == TILE_SIZE {
                    continue;
                }

                if dy.abs() < TILE_SIZE / slip_factor {
                    continue;
                }

                if dy <= 0.0 {
                    walker.vertical_direction = Some(Direction::Down);
                } else {
                    walker.vertical_direction = Some(Direction::Up);
                }
            }
            DirectionAxis::Vertical => {
                let dx = walker_transform.translation.x - wall_transform.translation.x;

                if dx.round().abs() == TILE_SIZE {
                    continue;
                }

                if dx.abs() < TILE_SIZE / slip_factor {
                    continue;
                }

                if dx <= 0.0 {
                    walker.horizontal_direction = Some(Direction::Left);
                } else {
                    walker.horizontal_direction = Some(Direction::Right);
                }
            }
        }
    }
}
