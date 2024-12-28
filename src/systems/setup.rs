use bevy::prelude::Commands;

use crate::constants::TILE_SIZE;
use crate::entity;
use crate::features;

pub fn run(mut commands: Commands) {
    features::map::generate_map(&mut commands);
    entity::create_player(&mut commands, 1.0 * TILE_SIZE, 1.0 * TILE_SIZE);
    entity::create_camera(&mut commands);
}
