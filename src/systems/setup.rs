use bevy::prelude::Commands;

use crate::entity;
use crate::features;

pub fn run(mut commands: Commands) {
    features::map::generate_map(&mut commands);
    entity::create_player(&mut commands, 1, 1);
    entity::create_camera(&mut commands);
}
