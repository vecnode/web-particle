// components.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Selected;

#[derive(Resource, Default)]
pub struct CameraViewChanged {
    pub needs_reset: bool,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct TrajectoryCircle {
    pub particle_entity: Entity,
}

#[derive(Resource, Default)]
pub struct ParticleSelectionState {
    pub selected_particles: std::collections::HashSet<Entity>,
}

#[derive(Resource, Default)]
pub struct ParticlePositions {
    pub base_positions: std::collections::HashMap<Entity, Vec3>, // Original spawn positions (normalized relative to bounds)
    pub current_positions: std::collections::HashMap<Entity, Vec3>, // Current world positions
}

#[derive(Resource)]
pub struct ParticleBoundsState {
    pub bounds_x: f32,  // Total size in X direction (meters) - diameter, not half-width
    pub bounds_z: f32,  // Total size in Z direction (meters) - diameter, not half-width
    pub bounds_y_height: f32,  // Y height/range starting from 1.0 (meters)
    pub previous_bounds_x: f32,
    pub previous_bounds_z: f32,
    pub previous_bounds_y_height: f32,
}

impl Default for ParticleBoundsState {
    fn default() -> Self {
        Self {
            bounds_x: 10.0,  // Total size: 10m (matches grid)
            bounds_z: 10.0,  // Total size: 10m (matches grid)
            bounds_y_height: 1.0,  // Height: 1.0m (Y range: 1.0 to 2.0)
            previous_bounds_x: 10.0,
            previous_bounds_z: 10.0,
            previous_bounds_y_height: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct ParticleGroupState {
    pub offset: Vec3,  // Global offset for all particles (for moving as group)
    pub scale: f32,    // Scale factor for resizing (1.0 = no scaling)
    pub previous_offset: Vec3,  // For change detection
    pub previous_scale: f32,  // For change detection
}

impl Default for ParticleGroupState {
    fn default() -> Self {
        Self {
            offset: Vec3::ZERO,
            scale: 1.0,
            previous_offset: Vec3::ZERO,
            previous_scale: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct Motion1State {
    pub is_active: bool,
    pub rotation_speed: f32, // radians per second
}

impl Default for Motion1State {
    fn default() -> Self {
        Self {
            is_active: false,
            rotation_speed: 1.0, // 1 radian per second (about 57 degrees per second)
        }
    }
}

#[derive(Resource, Default)]
pub struct TrajectoryState {
    pub is_visible: bool,
}

#[derive(Component)]
pub struct SelectionBox;

#[derive(Resource, Default)]
pub struct SelectionBoxState {
    pub is_active: bool,
    pub start_position: Option<Vec2>,
    pub current_position: Option<Vec2>,
}

#[derive(Resource, Default)]
pub struct MouseButtonState {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub left_was_pressed: bool,
    pub right_was_pressed: bool,
}

#[derive(Component)]
pub struct RightCamera;

#[derive(Resource)]
pub struct CameraProjectionState {
    pub last_perspective_fov: f32, // Store FOV for camera projection state
}

impl Default for CameraProjectionState {
    fn default() -> Self {
        Self {
            last_perspective_fov: 1.047, // Default ~60 degrees, will be updated from actual camera
        }
    }
}

#[derive(Resource, Default)]
pub struct EguiLayoutState {
    pub left_panel_end_x: f32, // Actual x position where left panel ends (in logical pixels)
    pub right_panel_start_x: f32, // Actual x position where right panel starts (in logical pixels)
    pub top_bars_height: f32, // Combined height of both top bars (in logical pixels)
}

#[derive(Component)]
pub struct GridLine;

#[derive(Resource)]
pub struct GridState {
    pub size_x: i32, // Grid size in X direction (meters)
    pub size_z: i32, // Grid size in Z direction (meters)
    pub previous_size_x: i32,
    pub previous_size_z: i32,
}

impl Default for GridState {
    fn default() -> Self {
        Self {
            size_x: 10,
            size_z: 10,
            previous_size_x: 10,
            previous_size_z: 10,
        }
    }
}
