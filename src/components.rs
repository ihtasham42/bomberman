use bevy::prelude::{Component, Entity};

use crate::features::movement::Direction;

#[derive(Component)]
pub struct Wall {
    pub(crate) ignore: Vec<Entity>,
}

impl Default for Wall {
    fn default() -> Self {
        Self { ignore: vec![] }
    }
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Walker {
    pub horizontal_direction: Option<Direction>,
    pub vertical_direction: Option<Direction>,
    pub last_direction: Option<Direction>,
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            horizontal_direction: None,
            vertical_direction: None,
            last_direction: None,
        }
    }
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Component)]
pub struct CameraTarget {}
#[derive(Component)]
pub struct Collider {}

#[derive(Component)]
pub struct BombPlacer {
    pub wants_to_place: bool,
}

impl Default for BombPlacer {
    fn default() -> Self {
        Self {
            wants_to_place: false,
        }
    }
}

#[derive(Component)]
pub struct Bomb {
    pub lifetime: i32,
}

impl Default for Bomb {
    fn default() -> Self {
        Self { lifetime: 0 }
    }
}

#[derive(Component)]
pub struct Explosion {
    pub lifetime: i32,
}
