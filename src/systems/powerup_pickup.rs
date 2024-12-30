use bevy::prelude::*;

use crate::components::{Hitbox, PowerupPickup, PowerupStats};
use crate::features::collision::are_collision_points_colliding;
use crate::features::powerup::PowerupType;

pub fn run(
    mut commands: Commands,
    powerup_query: Query<(Entity, &PowerupPickup, &Transform)>,
    hitbox_query: Query<(&Parent, &Transform), With<Hitbox>>,
    mut powerup_stats_query: Query<&mut PowerupStats>,
) {
    for (powerup_entity, powerup_pickup, powerup_transform) in powerup_query.iter() {
        for (hitbox_parent, hitbox_transform) in hitbox_query.iter() {
            if !are_collision_points_colliding(hitbox_transform, powerup_transform) {
                continue;
            }

            let Ok(mut powerup_stats) = powerup_stats_query.get_mut(**hitbox_parent) else {
                continue;
            };

            match powerup_pickup.powerup_type {
                PowerupType::MaxBombs => {
                    powerup_stats.max_bombs += 1;
                    powerup_stats.current_bombs += 1;
                }
                PowerupType::BombPower => {
                    powerup_stats.bomb_power += 1;
                }
                PowerupType::PlayerSpeed => {
                    powerup_stats.player_speed += 1;
                }
            }

            commands.entity(powerup_entity).despawn();
        }
    }
}
