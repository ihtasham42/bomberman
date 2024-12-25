use bevy::prelude::*;

pub fn create_camera(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
}
