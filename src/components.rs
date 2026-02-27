// components.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct InMotion;

#[derive(Component)]
pub struct SelectionBoundingBox;

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
pub struct SelectionTransformState {
    pub position_offset: Vec3,  // XYZ position offset for selected particles
    pub scale: Vec3,  // XYZ scale for selected particles (normal distribution)
    pub previous_position_offset: Vec3,
    pub previous_scale: Vec3,
    pub original_selection_positions: std::collections::HashMap<Entity, Vec3>,  // Store original positions when selection changes
    pub previous_selection_hash: u64,  // Hash of selection to detect changes
}

impl Default for SelectionTransformState {
    fn default() -> Self {
        Self {
            position_offset: Vec3::ZERO,
            scale: Vec3::ONE,
            previous_position_offset: Vec3::ZERO,
            previous_scale: Vec3::ONE,
            original_selection_positions: std::collections::HashMap::new(),
            previous_selection_hash: 0,
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

#[derive(Resource)]
pub struct EguiLayoutState {
    pub left_panel_end_x: f32, // Actual x position where left panel ends (in logical pixels)
    pub right_panel_start_x: f32, // Actual x position where right panel starts (in logical pixels)
    pub top_bars_height: f32, // Combined height of both top bars (in logical pixels)
    pub bottom_bar_height: f32, // Height of the bottom bar (in logical pixels)
    pub left_panel_content_width: f32, // Actual content width inside left panel (in logical pixels)
    pub right_panel_content_width: f32, // Actual content width inside right panel (in logical pixels)
    pub inspector_collapsed: bool, // Whether the inspector panel is collapsed
    pub left_half_panel_collapsed: bool, // Whether the left half panel (middle) is collapsed
    pub d3_viewer_visible: bool, // Whether the 3D viewer is visible (default: true)
    pub plot_center_axes: bool, // Whether to center plot axes to grid dimensions (default: false)
}

impl Default for EguiLayoutState {
    fn default() -> Self {
        Self {
            left_panel_end_x: 0.0,
            right_panel_start_x: 0.0,
            top_bars_height: 0.0,
            bottom_bar_height: 0.0,
            left_panel_content_width: 0.0,
            right_panel_content_width: 0.0,
            inspector_collapsed: false,
            left_half_panel_collapsed: true, // Start with left panel hidden
            d3_viewer_visible: true, // 3D viewer is visible by default
            plot_center_axes: false, // Start with auto-fit axes
        }
    }
}

#[derive(Resource, Default)]
pub struct StreamsPanelState {
    pub is_visible: bool,
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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParticlePlacementMode {
    Random,
    Ball,
    Cube,
}

#[derive(Resource)]
pub struct ParticleCreationState {
    pub placement_mode: ParticlePlacementMode,
    pub batch_count: usize,
    pub ball_center: Vec3,
    pub ball_radius: f32,
    pub cube_center: Vec3,
    pub cube_size: Vec3,
    pub y_min: f32,
    pub create_requested: bool,
    pub remove_selected_requested: bool,
    pub remove_all_requested: bool,
}

impl Default for ParticleCreationState {
    fn default() -> Self {
        Self {
            placement_mode: ParticlePlacementMode::Random,
            batch_count: 10,
            ball_center: Vec3::new(0.0, 1.5, 0.0),
            ball_radius: 2.0,
            cube_center: Vec3::new(0.0, 1.5, 0.0),
            cube_size: Vec3::new(2.0, 1.0, 2.0),
            y_min: 1.0,
            create_requested: false,
            remove_selected_requested: false,
            remove_all_requested: false,
        }
    }
}
