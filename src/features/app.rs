use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::constants::{COLOR_BACKGROUND, FIXED_UPDATE_FREQUENCY, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::features::map::WallLookup;
use crate::systems;

pub fn create_app() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .insert_resource(Time::<Fixed>::from_hz(FIXED_UPDATE_FREQUENCY))
        .insert_resource(WallLookup::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bomberman".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(
            Startup,
            (systems::map_generator::run, systems::player_spawner::run).chain(),
        )
        .add_systems(
            Update,
            (systems::player_input::run, systems::camera::run).chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                systems::bomb_placer::run,
                systems::bomb_wall_ignore_remover::run,
                systems::explosion_cleanup::run,
                systems::explosion_interaction::run,
                systems::walker::run,
                systems::velocity::run,
                systems::collision::run,
                systems::air_resistance::run,
                systems::walker_constrainer::run,
                systems::hitbox_follow::run,
                systems::destroyable_destruction::run,
                systems::powerup_pickup::run,
                systems::bomb_exploder::run,
            )
                .chain(),
        )
        .run();
}
