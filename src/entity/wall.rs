use bevy::prelude::{Commands, Sprite, SpriteBundle};

use crate::components::Wall;
use crate::constants::COLOR_WALL;
use crate::features;

pub fn create_wall(commands: &mut Commands, x: i32, y: i32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_WALL,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y),
            ..Default::default()
        },
        Wall {},
    ));
}
