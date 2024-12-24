use std::default::Default;

use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod entity;
pub mod features;
pub mod systems;
pub mod util;

fn main() {
    features::app::create_app();
}
