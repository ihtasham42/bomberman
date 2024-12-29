use bevy::prelude::*;

use crate::components::{Destroyable, Wall};
use crate::constants::{COLOR_WOOD_CRATE, WALL_Z};
use crate::features;

pub fn create_wood_crate(commands: &mut Commands, x: f32, y: f32) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: COLOR_WOOD_CRATE,
                    ..Default::default()
                },
                transform: features::map::create_transform_from_tile_pos(x, y, WALL_Z),
                ..Default::default()
            },
            Wall::default(),
            Destroyable { hitpoints: 1 },
        ))
        .id()
}
