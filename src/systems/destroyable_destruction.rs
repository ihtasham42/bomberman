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
    mut query: Query<(
        Entity,
        &mut Destroyable,
        &Transform,
        Option<&DropsPowerup>,
        Option<&Wall>,
    )>,
) {
    for (entity, mut destroyable, transform, drops_powerup, wall) in query.iter_mut() {
        destroyable.invulnerability_lifetime =
            i32::max(destroyable.invulnerability_lifetime - 1, 0);

        if destroyable.hitpoints <= 0 {
            if let Some(_) = wall {
                destroy_wall(
                    &mut commands,
                    &mut wall_lookup,
                    transform.translation.x,
                    transform.translation.y,
                    entity,
                );
            } else {
                commands.entity(entity).despawn();
            }

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
