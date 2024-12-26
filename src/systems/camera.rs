use bevy::prelude::*;

use crate::components::CameraTarget;

pub fn run(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,
) {
    if let Ok(target_transform) = target_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = target_transform.translation.x;
            camera_transform.translation.y = target_transform.translation.y;
        }
    }
}
