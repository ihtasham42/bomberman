use bevy::prelude::*;

use crate::components::{Player, Walker};
use crate::features::movement::Direction;

pub fn run(mut query: Query<(&Player, &mut Walker)>, keys: Res<ButtonInput<KeyCode>>) {
    for (_player, mut walker) in &mut query {
        walker.horizontal_direction =
            match (keys.pressed(KeyCode::KeyA), keys.pressed(KeyCode::KeyD)) {
                (true, false) => Some(Direction::Left),
                (false, true) => Some(Direction::Right),
                _ => None,
            };

        walker.vertical_direction = match (keys.pressed(KeyCode::KeyW), keys.pressed(KeyCode::KeyS))
        {
            (true, false) => Some(Direction::Up),
            (false, true) => Some(Direction::Down),
            _ => None,
        };
    }
}
