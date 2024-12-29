use bevy::prelude::*;

use crate::components::{Bomb, Wall};
use crate::constants::TILE_SIZE;
use crate::entity;

pub fn run(
    mut commands: Commands,
    mut bomb_query: Query<(Entity, &mut Bomb, &Transform)>,
    mut interaction_query: Query<(&Transform), With<Wall>>,
) {
    for (entity, mut bomb, transform) in bomb_query.iter_mut() {
        bomb.lifetime -= 1;

        if bomb.lifetime <= 0 {
            commands.entity(entity).despawn();

            entity::create_explosion(
                &mut commands,
                transform.translation.x,
                transform.translation.y,
            );

            let mut bomb_direction_deltas = vec![
                (TILE_SIZE, 0.0),
                (-TILE_SIZE, 0.0),
                (0.0, TILE_SIZE),
                (0.0, -TILE_SIZE),
            ];

            for (dx, dy) in bomb_direction_deltas {
                let x = transform.translation.x;
                let y = transform.translation.y;

                for power in 1..bomb.power {
                    let fx = x + dx * power as f32;
                    let fy = y + dy * power as f32;

                    entity::create_explosion(&mut commands, fx, fy);

                    let mut explosion_blocked = false;

                    for interaction_transform in interaction_query.iter() {
                        if interaction_transform.translation.x == transform.translation.x
                            && interaction_transform.translation.y == transform.translation.y
                        {
                            explosion_blocked = true;
                        }
                    }

                    if explosion_blocked {
                        break;
                    }
                }
            }
        }
    }
}

fn explode_bomb(commands: &mut Commands, bomb_entity: Entity, x: f32, y: f32, bomb_power: i32) {
    commands.entity(bomb_entity).despawn();
}
