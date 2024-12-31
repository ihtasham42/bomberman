use bevy::math::Vec3;
use bevy::prelude::*;

use crate::components::UserPlayer;
use crate::entity;

pub fn run(mut commands: Commands, player_query: Query<(Entity, &Transform), With<UserPlayer>>) {
    let (player_entity, player_transform) = player_query
        .get_single()
        .expect("Expected user player to exist");

    let initial_transform = Transform {
        scale: Vec3::new(2.0, 2.0, 2.0),
        ..player_transform.clone()
    };

    entity::create_camera(&mut commands, player_entity, initial_transform);
}
