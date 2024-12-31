use bevy::prelude::*;
use bevy_trauma_shake::Shake;

use crate::components::Targeter;

pub fn create_camera(
    commands: &mut Commands,
    camera_target_entity: Entity,
    initial_transform: Transform,
) {
    commands.spawn((
        Camera2dBundle {
            transform: initial_transform,
            ..Default::default()
        },
        Targeter {
            target: Some(camera_target_entity),
        },
        Shake::default(),
    ));
}
