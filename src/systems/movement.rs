use bevy::prelude::*;

use crate::components::{Velocity, Walker};
use crate::features::movement::Direction;

const BASE_MOVE_SPEED: f32 = 1.0;

pub fn run(mut query: Query<(&mut Velocity, &Walker)>) {
    for (mut velocity, walker) in &mut query {
        velocity.x += match &walker.horizontal_direction {
            Some(Direction::Left) => -BASE_MOVE_SPEED,
            Some(Direction::Right) => BASE_MOVE_SPEED,
            _ => 0.0,
        };

        velocity.y += match &walker.vertical_direction {
            Some(Direction::Up) => BASE_MOVE_SPEED,
            Some(Direction::Down) => -BASE_MOVE_SPEED,
            _ => 0.0,
        };
    }
}
