use bevy::prelude::*;

use crate::components::{Velocity, Walker};
use crate::features::movement::Direction;

pub fn run(mut query: Query<(&mut Velocity, &Walker)>) {
    for (mut velocity, walker) in &mut query {
        velocity.x += match &walker.horizontal_direction {
            Some(Direction::Left) => -1.0,
            Some(Direction::Right) => 1.0,
            _ => 0.0,
        };

        velocity.y += match &walker.vertical_direction {
            Some(Direction::Up) => 1.0,
            Some(Direction::Down) => -1.0,
            _ => 0.0,
        };
    }
}
