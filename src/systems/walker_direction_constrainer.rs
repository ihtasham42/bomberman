use bevy::prelude::*;

use crate::components::{Walker, WalkerConstrainer};
use crate::features::map::{closest_tile_pos, WallLookup};
use crate::features::movement::DirectionAxis;

pub fn run(
    wall_lookup: Res<WallLookup>,
    mut query: Query<(&mut Walker, &Transform), With<WalkerConstrainer>>,
) {
    for (mut walker, transform) in query.iter_mut() {
        let (tile_x, tile_y) = closest_tile_pos(transform.translation.x, transform.translation.y);

        if transform.translation.x == tile_x && transform.translation.y == tile_y {
            let Some(horizontal_direction) = &walker.horizontal_direction else {
                continue;
            };

            let Some(vertical_direction) = &walker.vertical_direction else {
                continue;
            };

            let priority_direction;

            if let DirectionAxis::Horizontal = walker.priority_direction_axis {
                priority_direction = horizontal_direction;
            } else {
                priority_direction = vertical_direction;
            }

            let (dx, dy) = priority_direction.to_delta();

            let fx = transform.translation.x + dx;
            let fy = transform.translation.y + dy;

            if wall_lookup.get(fx, fy).is_some() {
                continue;
            }

            if let DirectionAxis::Horizontal = walker.priority_direction_axis {
                walker.vertical_direction = None;
            } else {
                walker.horizontal_direction = None;
            }
        }
    }
}
