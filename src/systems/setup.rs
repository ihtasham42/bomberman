use bevy::prelude::*;

use crate::constants::TILE_SIZE;
use crate::entity;
use crate::features;
use crate::features::map::WallLookup;

pub fn run(mut commands: Commands, mut wall_lookup: ResMut<WallLookup>) {
    features::map_generation::generate_map(&mut commands, &mut wall_lookup);
    entity::create_player(&mut commands, 1.0 * TILE_SIZE, 1.0 * TILE_SIZE);
    entity::create_camera(&mut commands);
}
