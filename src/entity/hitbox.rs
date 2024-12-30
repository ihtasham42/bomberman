use bevy::prelude::*;

use crate::components::Hitbox;
use crate::constants::{HITBOX_TO_TILE_SCALE, TILE_SIZE};
use crate::features;

#[derive(Bundle)]
pub struct HitboxBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub hitbox: Hitbox,
}

impl HitboxBundle {
    pub fn new() -> Self {
        HitboxBundle {
            transform: features::map::create_transform_from_tile_pos(
                0.0,
                0.0,
                crate::constants::ITEM_Z,
                TILE_SIZE * HITBOX_TO_TILE_SCALE,
            ),
            global_transform: GlobalTransform::default(),
            hitbox: Hitbox {},
        }
    }
}
