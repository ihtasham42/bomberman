use bevy::prelude::*;

use crate::features;
use crate::features::map::WallLookup;

pub fn run(mut commands: Commands, mut wall_lookup: ResMut<WallLookup>) {
    features::map_generation::generate_map(&mut commands, &mut wall_lookup);
}
