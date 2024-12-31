use bevy::prelude::*;

use crate::components::Targeter;
use crate::constants::MAP_SIZE;
use crate::features::map::tile_pos;

pub fn run(
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &Targeter), With<Camera>>,
    target_query: Query<&Transform, Without<Camera>>,
) {
    if let Ok((mut camera_transform, targeter)) = camera_query.get_single_mut() {
        let mut target_position = Vec3::ZERO;
        let mut target_scale = Vec3::new(2.0, 2.0, 1.0);
        let mut default_target = false;

        if let Some(target) = targeter.target {
            if let Ok(target_transform) = target_query.get(target) {
                target_position = target_transform.translation;
                target_scale = Vec3::new(1.0, 1.0, 1.0);
            } else {
                default_target = true;
            }
        } else {
            default_target = true;
        }

        if default_target {
            let center = (MAP_SIZE / 2) as f32;
            let (x, y) = tile_pos(center, center);
            target_position = Vec3::new(x, y, 0.0);
        }

        let lerp_factor = 5.0 * time.delta_seconds();
        camera_transform.translation = camera_transform
            .translation
            .lerp(target_position, lerp_factor);
        camera_transform.scale = camera_transform.scale.lerp(target_scale, lerp_factor);
    }
}
