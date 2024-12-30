use bevy::prelude::*;

use crate::components::{Bomb, Destroyable, Explosion};

pub fn run(
    explosion_query: Query<&Transform, With<Explosion>>,
    mut interaction_query: Query<(&Transform, Option<&mut Bomb>, Option<&mut Destroyable>)>,
) {
    for explosion_transform in explosion_query.iter() {
        for (interaction_transform, bomb, destroyable) in interaction_query.iter_mut() {
            if explosion_transform.translation.x == interaction_transform.translation.x
                && explosion_transform.translation.y == interaction_transform.translation.y
            {
                if let Some(mut bomb) = bomb {
                    bomb.lifetime = 0;
                }

                if let Some(mut destroyable) = destroyable {
                    if destroyable.invulnerability_lifetime <= 0 {
                        destroyable.hitpoints -= 1;
                    }
                }
            }
        }
    }
}
