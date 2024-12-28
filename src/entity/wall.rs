use bevy::prelude::{Commands, Sprite, SpriteBundle};

use crate::components::Wall;
use crate::constants::{COLOR_WALL, WALL_Z};
use crate::features;

pub fn create_wall(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_WALL,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y, WALL_Z),
            ..Default::default()
        },
        Wall::default(),
    ));
}
