use bevy::prelude::*;

use crate::components::Velocity;

pub fn run(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
