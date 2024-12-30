use bevy::prelude::*;

use crate::components::{Destroyable, DropsPowerup, Wall};
use crate::constants::{COLOR_WOOD_CRATE, TILE_SIZE, WALL_Z};
use crate::features;

pub fn create_wood_crate(commands: &mut Commands, x: f32, y: f32) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: COLOR_WOOD_CRATE,
                    ..Default::default()
                },
                transform: features::map::create_transform_from_tile_pos(x, y, WALL_Z, TILE_SIZE),
                ..Default::default()
            },
            Wall::default(),
            Destroyable {
                hitpoints: 1,
                invulnerability_lifetime: 0,
            },
            DropsPowerup {},
        ))
        .id()
}
