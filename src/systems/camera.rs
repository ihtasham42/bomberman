use bevy::prelude::*;

use crate::components::CameraTarget;

pub fn run(
    mut param_set: ParamSet<(
        Query<&Transform, With<CameraTarget>>,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    let Ok(target_transform) = param_set.p0().get_single().cloned() else {
        return;
    };

    if let Ok(mut camera_transform) = param_set.p1().get_single_mut() {
        camera_transform.translation.x = target_transform.translation.x;
        camera_transform.translation.y = target_transform.translation.y;
    }
}
