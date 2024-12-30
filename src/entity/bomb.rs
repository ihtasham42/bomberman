use bevy::prelude::*;

use crate::components::{Bomb, Wall};
use crate::constants::{BOMB_EXPLOSION_INITIAL_LIFETIME, COLOR_BOMB, ITEM_Z, TILE_SIZE};
use crate::features;
use crate::features::util::seconds_to_freq;

pub fn create_bomb(
    commands: &mut Commands,
    x: f32,
    y: f32,
    ignore_colliders: Vec<Entity>,
    bomb_power: i32,
    bomb_placer: Entity,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: COLOR_BOMB,
                    ..Default::default()
                },
                transform: features::map::create_transform_from_tile_pos(x, y, ITEM_Z, TILE_SIZE),
                ..Default::default()
            },
            Bomb {
                lifetime: seconds_to_freq(BOMB_EXPLOSION_INITIAL_LIFETIME),
                power: bomb_power,
                placer: bomb_placer,
            },
            Wall {
                ignore: ignore_colliders,
            },
        ))
        .id()
}
