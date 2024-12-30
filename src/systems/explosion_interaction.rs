use bevy::prelude::*;

use crate::components::{Bomb, Destroyable, Explosion, Hitbox, Player};
use crate::features::collision::are_collision_points_colliding;

pub fn run(
    mut commands: Commands,
    explosion_query: Query<&Transform, With<Explosion>>,
    mut interaction_query: Query<(
        &Transform,
        Option<&mut Bomb>,
        Option<&mut Destroyable>,
        Option<&Hitbox>,
        Option<&Parent>,
    )>,
    player_query: Query<Entity, With<Player>>,
) {
    for explosion_transform in explosion_query.iter() {
        for (interaction_transform, bomb, destroyable, hitbox, parent) in
            interaction_query.iter_mut()
        {
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

            if let Some(_) = hitbox {
                if let Some(parent) = parent {
                    if let Ok(player_entity) = player_query.get(**parent) {
                        if !are_collision_points_colliding(
                            explosion_transform,
                            interaction_transform,
                        ) {
                            continue;
                        }

                        commands.entity(player_entity).despawn();
                    }
                }
            }
        }
    }
}
