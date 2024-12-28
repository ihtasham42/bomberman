use bevy::prelude::*;

use crate::components::Bomb;

pub fn run(mut commands: Commands, mut query: Query<(Entity, &mut Bomb, &Transform)>) {
    for (entity, mut bomb, transform) in query.iter_mut() {
        bomb.lifetime -= 1;

        if bomb.lifetime <= 0 {
            commands.entity(entity).despawn()
        }
    }
}
