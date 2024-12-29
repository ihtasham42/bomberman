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
    pub power: i32,
    pub placer: Entity,
}

#[derive(Component)]
pub struct Explosion {
    pub lifetime: i32,
}

#[derive(Component)]
pub struct Destroyable {
    pub hitpoints: i32,
}

#[derive(Component)]
pub struct DropsPowerup;

#[derive(Component)]
pub struct PowerupStats {
    pub max_bombs: i32,
    pub current_bombs: i32,
    pub bomb_power: i32,
    pub player_speed: i32,
}

impl Default for PowerupStats {
    fn default() -> Self {
        Self {
            max_bombs: 1,
            current_bombs: 1,
            bomb_power: 1,
            player_speed: 1,
        }
    }
}
