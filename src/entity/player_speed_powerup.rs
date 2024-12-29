use bevy::prelude::*;

use crate::components::PowerupPickup;
use crate::constants::{COLOR_PLAYER_SPEED_POWERUP, ITEM_Z};
use crate::features;
use crate::features::powerup::PowerupType;

pub fn create_player_speed_powerup(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLAYER_SPEED_POWERUP,
                ..Default::default()
            },
            transform: features::map::create_transform_from_tile_pos(x, y, ITEM_Z),
            ..Default::default()
        },
        PowerupPickup {
            powerup_type: PowerupType::PlayerSpeed,
        },
    ));
}
