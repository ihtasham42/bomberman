use bevy::prelude::*;

use crate::components::Hitbox;

pub fn run(
    parent_query: Query<&Transform, Without<Hitbox>>,
    mut hitbox_query: Query<(&Parent, &mut Transform), With<Hitbox>>,
) {
    for (hitbox_parent, mut hitbox_transform) in hitbox_query.iter_mut() {
        if let Ok(parent_transform) = parent_query.get(**hitbox_parent) {
            hitbox_transform.translation.x = parent_transform.translation.x;
            hitbox_transform.translation.y = parent_transform.translation.y;
        }
    }
}
