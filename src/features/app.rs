use bevy::app::{App, Startup, Update};
use bevy::DefaultPlugins;
use bevy::prelude::{ClearColor, PluginGroup, Window, WindowPlugin};
use bevy::window::WindowResolution;

use crate::constants::{COLOR_BACKGROUND, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::systems;

pub fn create_app() {
    App::new()
        .insert_resource(ClearColor(COLOR_BACKGROUND))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bomberman".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, systems::setup::run)
        .add_systems(Update, systems::player_input::run)
        .run();
}
