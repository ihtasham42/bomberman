use bevy::prelude::*;

use crate::components::Explosion;

pub fn run(mut commands: Commands, mut query: Query<(Entity, &mut Explosion)>) {
    for (entity, mut explosion) in query.iter_mut() {
        explosion.lifetime -= 1;

        if explosion.lifetime <= 0 {
            commands.entity(entity).despawn();
        }
    }
}
