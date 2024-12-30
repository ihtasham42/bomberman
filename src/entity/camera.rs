use bevy::prelude::*;

use crate::components::Targeter;

pub fn create_camera(commands: &mut Commands, camera_target_entity: Entity) {
    commands.spawn((
        Camera2dBundle::default(),
        Targeter {
            target: Some(camera_target_entity),
        },
    ));
}
