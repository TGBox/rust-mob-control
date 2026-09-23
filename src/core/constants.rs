use bevy::prelude::*;

// Field dimensions
pub const LANE_WIDTH: f32 = 14.0;
pub const LANE_LENGTH: f32 = 48.0;
pub const CANNON_Z: f32 = 20.0;
pub const BASE_Z: f32 = -22.0;

// Mob settings
pub const MOB_RADIUS: f32 = 0.28;
pub const MOB_BASE_SPEED: f32 = 10.0;
pub const MOB_SEPARATION_DISTANCE: f32 = 0.65;
pub const MOB_SEPARATION_FORCE: f32 = 18.0;

// Champion settings
pub const CHAMPION_RADIUS: f32 = 0.85;
pub const CHAMPION_BASE_HP: f32 = 30.0;
pub const CHAMPION_CHARGE_REQUIRED: f32 = 25.0; // Mobs shot to charge champion

// Cannon settings
pub const CANNON_MIN_X: f32 = -LANE_WIDTH / 2.0 + 1.2;
pub const CANNON_MAX_X: f32 = LANE_WIDTH / 2.0 - 1.2;
pub const CANNON_BASE_FIRE_RATE: f32 = 7.0; // shots per sec

// Spatial hash cell size
pub const SPATIAL_CELL_SIZE: f32 = 1.2;

// Aesthetic Colors (Sleek modern palette)
pub const COLOR_PLAYER_MOB: Color = Color::srgb(0.05, 0.55, 1.0); // Vibrant blue
pub const COLOR_PLAYER_CHAMPION: Color = Color::srgb(0.0, 0.35, 0.95);
pub const COLOR_ENEMY_MOB: Color = Color::srgb(1.0, 0.22, 0.25); // Vibrant red/crimson
pub const COLOR_ENEMY_CHAMPION: Color = Color::srgb(0.85, 0.08, 0.12);
pub const COLOR_GATE_BLUE: Color = Color::srgb(0.1, 0.75, 1.0); // Multiplier gate blue
pub const COLOR_GATE_RED: Color = Color::srgb(1.0, 0.3, 0.3); // Debuff gate red
pub const COLOR_GATE_GOLD: Color = Color::srgb(1.0, 0.82, 0.1); // High multiplier gold
pub const COLOR_FLOOR: Color = Color::srgb(0.92, 0.94, 0.96); // Clean bright runway
pub const COLOR_FLOOR_ACCENT: Color = Color::srgb(0.85, 0.88, 0.92);
pub const COLOR_WALL: Color = Color::srgb(0.2, 0.25, 0.32); // Modern slate border
pub const COLOR_BASE: Color = Color::srgb(0.95, 0.2, 0.25);
