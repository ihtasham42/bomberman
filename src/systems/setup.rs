use bevy::prelude::{Camera2dBundle, Commands, Transform};

use crate::constants::{TILE_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::features;

pub fn run(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(WINDOW_WIDTH / 2.0 - TILE_SIZE as f32 / 2.0, WINDOW_HEIGHT / 2.0 - TILE_SIZE as f32 / 2.0, 0.0),
        ..Default::default()
    });

    features::map::generate_map(&mut commands);
}