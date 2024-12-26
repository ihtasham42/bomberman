use bevy::prelude::*;

use crate::components::Velocity;

pub fn run(mut query: Query<&mut Velocity>) {
    for mut velocity in &mut query {
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
}
