use bevy::color::Color;

pub const WINDOW_WIDTH: f32 = 640.0;
pub const WINDOW_HEIGHT: f32 = 640.0;

pub const TILE_SIZE: f32 = 48.0;

pub const FIXED_UPDATE_FREQUENCY: f64 = 60.0;

pub const COLOR_BACKGROUND: Color = Color::srgb(0.13, 0.13, 0.23);
pub const COLOR_WALL: Color = Color::srgb(1.0, 1.0, 1.0);
pub const COLOR_WOOD_CRATE: Color = Color::srgb(0.7, 0.4, 0.4);
pub const COLOR_PLAYER: Color = Color::srgb(0.2, 0.7, 0.2);
pub const COLOR_BOMB: Color = Color::srgb(0.9, 0.1, 0.1);
pub const COLOR_EXPLOSION: Color = Color::srgb(1.0, 0.4, 0.0);
pub const COLOR_MAX_BOMBS_POWERUP: Color = Color::srgb(0.0, 1.0, 0.0);
pub const COLOR_BOMB_POWER_POWERUP: Color = Color::srgb(1.0, 0.3, 0.3);
pub const COLOR_PLAYER_SPEED_POWERUP: Color = Color::srgb(0.4, 0.8, 1.0);

pub const ITEM_Z: f32 = 5.0;
pub const EXPLOSION_Z: f32 = 6.0;
pub const PLAYER_Z: f32 = 7.0;
pub const WALL_Z: f32 = 8.0;

pub const BOMB_EXPLOSION_INITIAL_LIFETIME: f32 = 4.0;
pub const EXPLOSION_CLEANUP_INITIAL_LIFETIME: f32 = 0.5;

pub const MAP_SIZE: i32 = 25;
pub const WOOD_CRATE_SPAWN_RATE: f64 = 0.0; //0.7;
pub const POWERUP_DROP_RATE: f64 = 1.0;
pub const BASE_MOVE_SPEED: f32 = 1.5;
pub const MOVE_SPEED_LEVEL_INCREASE: f32 = 0.4;
pub const POWERUP_PICKUP_INVULNERABILITY_TIME: f32 = 0.6;
pub const HITBOX_TO_TILE_SCALE: f32 = 0.7;
pub const MAX_CAMERA_TRAUMA: f32 = 0.4;
