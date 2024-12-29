use bevy::prelude::*;

use crate::components::Bomb;
use crate::constants::TILE_SIZE;
use crate::entity;
use crate::features::map::WallLookup;

pub fn run(
    mut commands: Commands,
    wall_lookup: Res<WallLookup>,
    mut bomb_query: Query<(Entity, &mut Bomb, &Transform)>,
) {
    for (entity, mut bomb, bomb_transform) in bomb_query.iter_mut() {
        bomb.lifetime -= 1;

        if bomb.lifetime <= 0 {
            commands.entity(entity).despawn();

            entity::create_explosion(
                &mut commands,
                bomb_transform.translation.x,
                bomb_transform.translation.y,
            );

            let mut bomb_direction_deltas = vec![
                (TILE_SIZE, 0.0),
                (-TILE_SIZE, 0.0),
                (0.0, TILE_SIZE),
                (0.0, -TILE_SIZE),
            ];

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

fn explode_bomb(commands: &mut Commands, bomb_entity: Entity, x: f32, y: f32, bomb_power: i32) {
    commands.entity(bomb_entity).despawn();
}
