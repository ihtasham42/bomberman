use std::collections::HashMap;

use bevy::prelude::*;

use crate::constants::TILE_SIZE;

#[derive(Resource, Default)]
pub struct WallLookup {
    map: HashMap<String, Entity>,
}

impl WallLookup {
    fn get_key(x: f32, y: f32) -> String {
        format!("{},{}", x, y)
    }

    pub fn get(&self, x: f32, y: f32) -> Option<&Entity> {
        let key = Self::get_key(x, y);

        self.map.get(&key)
    }

    pub fn set(&mut self, x: f32, y: f32, entity: Entity) {
        let key = Self::get_key(x, y);

        self.map.insert(key, entity);
    }

    pub fn remove(&mut self, x: f32, y: f32) {
        let key = Self::get_key(x, y);

        self.map.remove(&key);
    }
}

pub fn destroy_wall(
    commands: &mut Commands,
    wall_lookup: &mut WallLookup,
    x: f32,
    y: f32,
    entity: Entity,
) {
    commands.entity(entity).despawn();
    wall_lookup.remove(x, y);
}

pub fn create_transform_from_tile_pos(x: f32, y: f32, z: f32, scale: f32) -> Transform {
    Transform {
        translation: Vec3::new(x, y, z),
        scale: Vec3::new(scale, scale, 1.0),
        ..Default::default()
    }
}

pub fn tile_pos(x: f32, y: f32) -> (f32, f32) {
    (x * TILE_SIZE, y * TILE_SIZE)
}

pub fn closest_tile_pos(x: f32, y: f32) -> (f32, f32) {
    (
        (x / TILE_SIZE).round() * TILE_SIZE,
        (y / TILE_SIZE).round() * TILE_SIZE,
    )
}

pub fn get_direction_deltas() -> Vec<(f32, f32)> {
    vec![
        (TILE_SIZE, 0.0),
        (-TILE_SIZE, 0.0),
        (0.0, TILE_SIZE),
        (0.0, -TILE_SIZE),
    ]
}
