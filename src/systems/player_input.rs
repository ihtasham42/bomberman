use bevy::prelude::*;

use crate::components::{BombPlacer, Player, Walker};
use crate::features::movement::Direction;

pub fn run(
    mut walker_query: Query<&mut Walker, With<Player>>,
    mut bomb_placer_query: Query<&mut BombPlacer, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for mut walker in &mut walker_query {
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

    for mut bomb_placer in &mut bomb_placer_query {
        if !bomb_placer.wants_to_place {
            bomb_placer.wants_to_place = keys.just_pressed(KeyCode::Space);
        }
    }
}
