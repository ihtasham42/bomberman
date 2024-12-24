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

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
