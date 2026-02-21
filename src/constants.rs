// constants.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;

// Particle constants
pub const PARTICLE_COUNT: usize = 50;
pub const PARTICLE_RADIUS: f32 = 0.025;
pub const GRID_BOUNDS: f32 = 4.0;

// Grid constants
pub const GRID_SIZE: f32 = 10.0;
pub const GRID_SPACING: f32 = 1.0;
pub const GRID_LINE_RADIUS: f32 = 0.005;
pub const GRID_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

// Axis constants
pub const AXIS_LENGTH: f32 = 5.0;
pub const AXIS_RADIUS: f32 = 0.01;

// Camera constants
pub const CAMERA_FRONT_POSITION: Vec3 = Vec3::new(0.0, 0.0, 15.0);
pub const CAMERA_TOP_POSITION: Vec3 = Vec3::new(0.0, 15.0, 0.0);
pub const CAMERA_START_POSITION: Vec3 = Vec3::new(9.0, 7.0, 15.0);

// Material colors
pub const COLOR_WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
pub const COLOR_PURPLE: Color = Color::srgb(0.8, 0.2, 1.0);
pub const COLOR_RED: Color = Color::srgb(1.0, 0.0, 0.0);
pub const COLOR_GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
pub const COLOR_BLUE: Color = Color::srgb(0.0, 0.0, 1.0);

// Lighting constants
pub const FRONT_LIGHT_ILLUMINANCE: f32 = 2000.0;
pub const BACK_LIGHT_ILLUMINANCE: f32 = 1500.0;

// UI constants
pub const UI_SIDEBAR_WIDTH_PERCENT: f32 = 15.0;
pub const UI_FONT_SIZE: f32 = 12.0;
pub const UI_PADDING: f32 = 10.0;
