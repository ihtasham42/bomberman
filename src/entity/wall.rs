use bevy::prelude::*;

use crate::components::Wall;
use crate::constants::{COLOR_WALL, TILE_SIZE, WALL_Z};
use crate::features;

pub fn create_wall(commands: &mut Commands, x: f32, y: f32) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: COLOR_WALL,
                    ..Default::default()
                },
                transform: features::map::create_transform_from_tile_pos(x, y, WALL_Z, TILE_SIZE),
                ..Default::default()
            },
            Wall::default(),
        ))
        .id()
}
