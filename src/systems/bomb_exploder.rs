use std::collections::HashSet;

use bevy::prelude::*;
use bevy_trauma_shake::Shake;

use crate::components::{Bomb, PowerupStats};
use crate::constants::{MAX_CAMERA_TRAUMA, TILE_SIZE};
use crate::entity;
use crate::features::map::{get_direction_deltas, WallLookup};

pub fn run(
    mut commands: Commands,
    wall_lookup: Res<WallLookup>,
    mut camera_query: Query<(&Transform, &mut Shake)>,
    mut bomb_placer_query: Query<&mut PowerupStats>,
    mut bomb_query: Query<(Entity, &mut Bomb, &Transform)>,
) {
    let mut exploded_bomb_entities = HashSet::new();
    let mut bombs_to_explode = Vec::new();
    let (camera_transform, mut camera_shake) = camera_query
        .get_single_mut()
        .expect("User camera should exist");

    for (entity, mut bomb, _) in bomb_query.iter_mut() {
        bomb.lifetime -= 1;

        if bomb.lifetime <= 0 {
            bombs_to_explode.push(entity);
        }
    }

    while let Some(bomb_entity) = bombs_to_explode.pop() {
        if exploded_bomb_entities.contains(&bomb_entity) {
            continue;
        }

        let Ok((_, bomb, bomb_transform)) = bomb_query.get(bomb_entity) else {
            continue;
        };

        exploded_bomb_entities.insert(bomb_entity);

        let new_bombs_to_explode = explode_bomb(
            &mut commands,
            &wall_lookup,
            &mut bomb_placer_query,
            &bomb_query,
            bomb_entity,
            bomb,
            bomb_transform,
        );

        shake_camera(
            camera_transform,
            &mut camera_shake,
            bomb_transform,
            bomb.power,
        );

        bombs_to_explode = [bombs_to_explode, new_bombs_to_explode].concat()
    }
}

fn shake_camera(
    camera_transform: &Transform,
    camera_shake: &mut Shake,
    bomb_transform: &Transform,
    bomb_power: i32,
) {
    let camera_pos = camera_transform.translation.truncate();
    let bomb_pos = bomb_transform.translation.truncate();
    let distance = camera_pos.distance(bomb_pos);

    let divisor_1 = (distance / TILE_SIZE - bomb_power as f32).max(0.01);
    let divisor_2 = 1.0 + divisor_1.ln() / 4.0_f32.ln();
    let factor = 1.0 / divisor_2;

    let trauma_value = (MAX_CAMERA_TRAUMA * factor - 0.05).max(0.0).min(0.3);

    camera_shake.add_trauma(trauma_value);
}

fn explode_bomb(
    mut commands: &mut Commands,
    wall_lookup: &Res<WallLookup>,
    bomb_placer_query: &mut Query<&mut PowerupStats>,
    bomb_query: &Query<(Entity, &mut Bomb, &Transform)>,
    bomb_entity: Entity,
    bomb: &Bomb,
    bomb_transform: &Transform,
) -> Vec<Entity> {
    commands.entity(bomb_entity).despawn();

    if let Ok(mut powerup_stats) = bomb_placer_query.get_mut(bomb.placer) {
        powerup_stats.current_bombs += 1;
    }

    entity::create_explosion(
        &mut commands,
        bomb_transform.translation.x,
        bomb_transform.translation.y,
    );

    let bomb_direction_deltas = get_direction_deltas();

    let mut bombs_to_explode = vec![];

    for (dx, dy) in bomb_direction_deltas {
        let x = bomb_transform.translation.x;
        let y = bomb_transform.translation.y;

        for power in 1..bomb.power + 1 {
            let fx = x + dx * power as f32;
            let fy = y + dy * power as f32;

            entity::create_explosion(&mut commands, fx, fy);

            for (chain_bomb_entity, _, chain_bomb_transform) in bomb_query.iter() {
                if bomb_transform.translation.x == chain_bomb_transform.translation.x
                    && bomb_transform.translation.y == chain_bomb_transform.translation.y
                {
                    bombs_to_explode.push(chain_bomb_entity)
                }
            }

            if wall_lookup.get(fx, fy).is_some() {
                break;
            };
        }
    }

    bombs_to_explode
}
