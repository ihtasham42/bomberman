use bevy::prelude::*;

use crate::components::{BombPlacer, CameraTarget, Collider, Player, Velocity, Walker};
use crate::constants::{COLOR_PLAYER, PLAYER_Z};
use crate::features;

pub fn create_player(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLAYER,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y, PLAYER_Z),
            ..Default::default()
        },
        Velocity::default(),
        Player {},
        Walker::default(),
        CameraTarget {},
        Collider {},
        BombPlacer::default(),
    ));
}
