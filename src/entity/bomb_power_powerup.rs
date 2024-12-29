use bevy::prelude::*;

use crate::components::PowerupPickup;
use crate::constants::{COLOR_BOMB_POWER_POWERUP, ITEM_Z};
use crate::features;
use crate::features::powerup::PowerupType;

pub fn create_bomb_power_powerup(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_BOMB_POWER_POWERUP,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y, ITEM_Z),
            ..Default::default()
        },
        PowerupPickup {
            powerup_type: PowerupType::BombPower,
        },
    ));
}
