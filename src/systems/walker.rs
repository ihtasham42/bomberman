use bevy::prelude::*;

use crate::components::{PowerupStats, Velocity, Walker};
use crate::constants::{BASE_MOVE_SPEED, MOVE_SPEED_LEVEL_INCREASE};
use crate::features::movement::Direction;

pub fn run(mut query: Query<(&mut Velocity, &Walker, &PowerupStats)>) {
    for (mut velocity, walker, power_up_stats) in &mut query {
        let move_speed =
            BASE_MOVE_SPEED + MOVE_SPEED_LEVEL_INCREASE * power_up_stats.player_speed as f32;

        velocity.x += match &walker.horizontal_direction {
            Some(Direction::Left) => -move_speed,
            Some(Direction::Right) => move_speed,
            _ => 0.0,
        };

        velocity.y += match &walker.vertical_direction {
            Some(Direction::Up) => move_speed,
            Some(Direction::Down) => -move_speed,
            _ => 0.0,
        };
    }
}
