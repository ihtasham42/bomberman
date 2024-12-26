use bevy::prelude::Component;

use crate::features::movement::Direction;

#[derive(Component)]
pub struct Wall {}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Walker {
    pub horizontal_direction: Option<Direction>,
    pub vertical_direction: Option<Direction>,
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            horizontal_direction: None,
            vertical_direction: None,
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
