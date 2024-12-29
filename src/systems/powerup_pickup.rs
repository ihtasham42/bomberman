use bevy::prelude::*;

use crate::components::{Player, PowerupPickup, PowerupStats};
use crate::features::collision::are_collision_points_colliding;
use crate::features::powerup::PowerupType;

pub fn run(
    mut commands: Commands,
    powerup_query: Query<(Entity, &PowerupPickup, &Transform)>,
    mut player_query: Query<(&mut PowerupStats, &Transform), With<Player>>,
) {
    for (powerup_entity, powerup_pickup, powerup_transform) in powerup_query.iter() {
        for (mut powerup_stats, player_transform) in player_query.iter_mut() {
            if !are_collision_points_colliding(player_transform, powerup_transform) {
                continue;
            }

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
