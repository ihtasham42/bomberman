use bevy::prelude::*;

use crate::components::Destroyable;
use crate::features::map::{destroy_wall, WallLookup};

pub fn run(
    mut commands: Commands,
    mut wall_lookup: ResMut<WallLookup>,
    query: Query<(Entity, &Destroyable, &Transform)>,
) {
    for (entity, destroyable, transform) in query.iter() {
        if destroyable.hitpoints == 0 {
            destroy_wall(
                &mut commands,
                &mut wall_lookup,
                transform.translation.x,
                transform.translation.y,
                entity,
            );
        }
    }
}
