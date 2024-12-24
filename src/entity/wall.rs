use bevy::math::Vec3;
use bevy::prelude::{Commands, Sprite, SpriteBundle, Transform};

use crate::components::Wall;
use crate::constants::{COLOR_WALL, TILE_SIZE};
use crate::util::tile_pos;

pub fn create_wall(commands: &mut Commands, x: i32, y: i32) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: COLOR_WALL,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(tile_pos(x) as f32, tile_pos(y) as f32, 0.0),
            scale: Vec3::new(TILE_SIZE as f32, TILE_SIZE as f32, 1.0),
            ..Default::default()
        },
        ..Default::default()
    }, Wall {}));
}