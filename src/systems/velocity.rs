use bevy::prelude::*;

use crate::components::Velocity;

pub fn run(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        velocity.x = 0.0;
        velocity.y = 0.0;
    }
}
