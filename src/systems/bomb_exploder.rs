use bevy::prelude::*;

use crate::components::{Bomb, PowerupStats};
use crate::entity;
use crate::features::map::{get_direction_deltas, WallLookup};

pub fn run(
    mut commands: Commands,
    wall_lookup: Res<WallLookup>,
    mut bomb_placer_query: Query<&mut PowerupStats>,
    mut bomb_query: Query<(Entity, &mut Bomb, &Transform)>,
) {
    for (entity, mut bomb, bomb_transform) in bomb_query.iter_mut() {
        bomb.lifetime -= 1;

        if bomb.lifetime <= 0 {
            commands.entity(entity).despawn();

            if let Ok(mut powerup_stats) = bomb_placer_query.get_mut(bomb.placer) {
                powerup_stats.current_bombs += 1;
            }

            entity::create_explosion(
                &mut commands,
                bomb_transform.translation.x,
                bomb_transform.translation.y,
            );

            let bomb_direction_deltas = get_direction_deltas();

            for (dx, dy) in bomb_direction_deltas {
                let x = bomb_transform.translation.x;
                let y = bomb_transform.translation.y;

                for power in 1..bomb.power + 1 {
                    let fx = x + dx * power as f32;
                    let fy = y + dy * power as f32;

                    entity::create_explosion(&mut commands, fx, fy);

                    if wall_lookup.get(fx, fy).is_some() {
                        break;
                    };
                }
            }
        }
    }
}
