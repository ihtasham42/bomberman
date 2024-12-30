use bevy::prelude::*;

use crate::components::{BombPlacer, Collider, Player, PowerupStats, Velocity, Walker};
use crate::constants::{COLOR_PLAYER, PLAYER_Z};
use crate::features;

pub fn create_player(commands: &mut Commands, x: f32, y: f32) -> Entity {
    commands
        .spawn((
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
            Collider {},
            BombPlacer::default(),
            PowerupStats::default(),
        ))
        .id()
}
