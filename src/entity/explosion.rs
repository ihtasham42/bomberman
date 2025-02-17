use bevy::prelude::*;

use crate::components::Explosion;
use crate::constants::{
    COLOR_EXPLOSION, EXPLOSION_CLEANUP_INITIAL_LIFETIME, EXPLOSION_Z, TILE_SIZE,
};
use crate::features;
use crate::features::util::seconds_to_freq;

pub fn create_explosion(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_EXPLOSION,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y, EXPLOSION_Z, TILE_SIZE),
            ..Default::default()
        },
        Explosion {
            lifetime: seconds_to_freq(EXPLOSION_CLEANUP_INITIAL_LIFETIME),
        },
    ));
}
