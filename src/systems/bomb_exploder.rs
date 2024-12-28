use bevy::prelude::*;

use crate::components::Bomb;

pub fn run(mut query: Query<(&Bomb, &Transform)>) {
    for (bomb, transform) in query.iter() {}
}
