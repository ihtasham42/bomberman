use bevy::prelude::*;

use crate::constants::{COLOR_EXPLOSION, ITEM_Z};
use crate::features;

pub fn create_explosion(mut commands: Commands, x: f32, y: f32) {
    commands.spawn(
        (SpriteBundle {
            sprite: Sprite {
                color: COLOR_EXPLOSION,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y, ITEM_Z),
            ..Default::default()
        }),
    );
}
