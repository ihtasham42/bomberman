use bevy::prelude::*;

use crate::components::{CameraTarget, Player, Velocity, Walker};
use crate::constants::COLOR_PLAYER;
use crate::features;

pub fn create_player(commands: &mut Commands, x: i32, y: i32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLAYER,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y),
            ..Default::default()
        },
        Player {},
        Walker::default(),
        Velocity::default(),
        CameraTarget {},
    ));
}
