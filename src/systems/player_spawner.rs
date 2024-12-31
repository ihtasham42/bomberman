use bevy::prelude::*;

use crate::components::{Destroyable, Wall};
use crate::features;
use crate::features::map::WallLookup;

pub fn run(
    mut commands: Commands,
    mut wall_lookup: ResMut<WallLookup>,
    destroyable_query: Query<Entity, (With<Wall>, With<Destroyable>)>,
) {
    features::player_spawn::spawn_player(&mut commands, &mut wall_lookup, destroyable_query);
}
