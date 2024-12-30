use bevy::prelude::*;

use crate::components::Targeter;

pub fn run(
    mut camera_query: Query<(&mut Transform, &Targeter), With<Camera>>,
    target_query: Query<&Transform, Without<Camera>>,
) {
    if let Ok((mut camera_transform, targeter)) = camera_query.get_single_mut() {
        if let Some(target) = targeter.target {
            if let Ok(target_transform) = target_query.get(target) {
                camera_transform.translation.x = target_transform.translation.x;
                camera_transform.translation.y = target_transform.translation.y;
            }
        }
    }
}
