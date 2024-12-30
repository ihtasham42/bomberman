use bevy::prelude::*;

use crate::components::Targeter;
use crate::constants::MAP_SIZE;
use crate::features::map::tile_pos;

pub fn run(
    mut camera_query: Query<(&mut Transform, &Targeter), With<Camera>>,
    target_query: Query<&Transform, Without<Camera>>,
) {
    if let Ok((mut camera_transform, targeter)) = camera_query.get_single_mut() {
        let mut default_target = false;

        if let Some(target) = targeter.target {
            if let Ok(target_transform) = target_query.get(target) {
                camera_transform.translation.x = target_transform.translation.x;
                camera_transform.translation.y = target_transform.translation.y;
            } else {
                default_target = true;
            }
        } else {
            default_target = true;
        }

        if default_target {
            let center = (MAP_SIZE / 2 + 1) as f32;
            let (x, y) = tile_pos(center, center);

            camera_transform.translation.x = x;
            camera_transform.translation.y = y;
        }
    }
}
