use bevy::prelude::*;

use crate::components::Bomb;
use crate::constants::{COLOR_BOMB, ITEM_Z};
use crate::features;

pub fn create_bomb(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_BOMB,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y, ITEM_Z),
            ..Default::default()
        },
        Bomb::default(),
    ));
}
