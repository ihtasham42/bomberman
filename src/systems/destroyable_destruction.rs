use bevy::prelude::*;
use rand::Rng;

use crate::components::{Destroyable, DropsPowerup, Wall};
use crate::constants::POWERUP_DROP_RATE;
use crate::entity::{
    create_bomb_power_powerup, create_max_bombs_powerup, create_player_speed_powerup,
};
use crate::features::map::{destroy_wall, WallLookup};

pub fn run(
    mut commands: Commands,
    mut wall_lookup: ResMut<WallLookup>,
    query: Query<(Entity, &Destroyable, &Transform, Option<&DropsPowerup>), With<Wall>>,
) {
    for (entity, destroyable, transform, drops_powerup) in query.iter() {
        if destroyable.hitpoints == 0 {
            destroy_wall(
                &mut commands,
                &mut wall_lookup,
                transform.translation.x,
                transform.translation.y,
                entity,
            );

            if let Some(_) = drops_powerup {
                spawn_powerup(&mut commands, transform);
            }
        }
    }
}

fn spawn_powerup(mut commands: &mut Commands, transform: &Transform) {
    let mut rng = rand::thread_rng();

    let spawn_choice = rng.gen_range(0.0..1.0);

    if spawn_choice <= POWERUP_DROP_RATE {
        let powerup_type_choice = rng.gen_range(0..3);

        match powerup_type_choice {
            0 => create_bomb_power_powerup(
                &mut commands,
                transform.translation.x,
                transform.translation.y,
            ),
            1 => create_player_speed_powerup(
                &mut commands,
                transform.translation.x,
                transform.translation.y,
            ),
            2 => create_max_bombs_powerup(
                &mut commands,
                transform.translation.x,
                transform.translation.y,
            ),
            _ => unreachable!(),
        };
    }
}
