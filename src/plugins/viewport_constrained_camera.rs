// plugins/viewport_constrained_camera.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::RightCamera;

/// Marker component for viewport-constrained camera controller
#[derive(Component)]
pub struct ViewportConstrainedCamera {
    /// Mouse rotation sensitivity (radians per pixel)
    pub sensitivity: f32,
    /// Movement speed (units per second)
    pub speed: f32,
    /// Fast movement speed multiplier (when Shift is held)
    pub fast_speed_multiplier: f32,
}

impl Default for ViewportConstrainedCamera {
    fn default() -> Self {
        Self {
            sensitivity: 0.003,  // Increased from 0.0015 for faster mouse rotation
            speed: 5.0,          // Match FreeCamera default speed
            fast_speed_multiplier: 3.0, // Match FreeCamera default fast multiplier
        }
    }
}

/// Internal state for viewport-constrained camera
/// Stores pitch and yaw angles for smooth rotation
#[derive(Component)]
pub struct ViewportConstrainedCameraState {
    pub pitch: f32,        // Vertical rotation (radians, clamped to -89° to 89°)
    pub yaw: f32,         // Horizontal rotation (radians)
    pub initialized: bool, // Whether state has been initialized from transform
}

/// Resource to track cursor position relative to camera viewport
#[derive(Resource, Default)]
pub struct CameraViewportCursorState {
    pub is_cursor_in_viewport: bool,
}

/// Updates cursor position state relative to camera viewport
/// Runs in PreUpdate to ensure viewport state is available before camera processes input
pub fn update_viewport_cursor_state(
    windows: Query<&Window>,
    camera: Query<&Camera, With<RightCamera>>,
    mut cursor_state: ResMut<CameraViewportCursorState>,
) {
    let Ok(window) = windows.single() else {
        cursor_state.is_cursor_in_viewport = false;
        return;
    };

    let cursor_logical = window.cursor_position();
    if cursor_logical.is_none() {
        cursor_state.is_cursor_in_viewport = false;
        return;
    }

    let cursor_logical = cursor_logical.unwrap();
    let scale_factor = window.scale_factor() as f32;
    let cursor_physical = cursor_logical * scale_factor;

    let mut is_in_viewport = false;
    if let Ok(camera_ref) = camera.single() {
        if let Some(viewport) = &camera_ref.viewport {
            let viewport_start = viewport.physical_position.as_vec2();
            let viewport_end = viewport_start + viewport.physical_size.as_vec2();

            if cursor_physical.x >= viewport_start.x
                && cursor_physical.x < viewport_end.x
                && cursor_physical.y >= viewport_start.y
                && cursor_physical.y < viewport_end.y
            {
                is_in_viewport = true;
            }
        } else {
            // No viewport set yet, assume cursor is in viewport to avoid blocking
            is_in_viewport = true;
        }
    }

    cursor_state.is_cursor_in_viewport = is_in_viewport;
}

/// Initializes camera state from transform rotation
/// Runs on first frame or when camera is reset
pub fn initialize_viewport_constrained_camera_state(
    mut cameras: Query<
        (&Transform, &mut ViewportConstrainedCameraState),
        (With<ViewportConstrainedCamera>, With<RightCamera>),
    >,
) {
    for (transform, mut state) in cameras.iter_mut() {
        if !state.initialized {
            // Extract pitch and yaw from transform rotation
            // Transform rotation is a quaternion, we need to convert to Euler angles
            let (yaw, pitch, _roll) = transform.rotation.to_euler(bevy::math::EulerRot::YXZ);
            
            state.pitch = pitch;
            state.yaw = yaw;
            state.initialized = true;
        }
    }
}

/// Handles mouse rotation with viewport constraints
/// Only processes mouse input when cursor is within camera viewport
/// Uses mouse position tracking instead of events for better compatibility
pub fn handle_viewport_constrained_mouse_rotation(
    mut cameras: Query<
        (&ViewportConstrainedCamera, &mut ViewportConstrainedCameraState, &mut Transform),
        (With<ViewportConstrainedCamera>, With<RightCamera>),
    >,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    cursor_state: Res<CameraViewportCursorState>,
    windows: Query<&Window>,
    mut last_mouse_pos: Local<Option<Vec2>>,
) {
    // Only process mouse rotation if left button is pressed AND cursor is in viewport
    let left_button_pressed = mouse_button_input.pressed(MouseButton::Left);
    if !left_button_pressed || !cursor_state.is_cursor_in_viewport {
        // Clear last position when button is released or cursor leaves viewport
        if !left_button_pressed {
            *last_mouse_pos = None;
        }
        return;
    }

    // Get current mouse position
    let Ok(window) = windows.single() else { return; };
    let Some(current_pos) = window.cursor_position() else {
        *last_mouse_pos = None;
        return;
    };

    // Calculate delta from last position
    let delta = if let Some(last_pos) = *last_mouse_pos {
        current_pos - last_pos
    } else {
        Vec2::ZERO // First frame, no delta
    };

    *last_mouse_pos = Some(current_pos);

    if delta.length_squared() < f32::EPSILON {
        return;
    }

    for (camera, mut state, mut transform) in cameras.iter_mut() {
        // Update yaw and pitch based on mouse movement
        state.yaw -= delta.x * camera.sensitivity;
        state.pitch -= delta.y * camera.sensitivity;

        // Clamp pitch to prevent gimbal lock (limit to -89° to 89°)
        const MAX_PITCH: f32 = std::f32::consts::PI / 2.0 - 0.01; // ~89 degrees
        state.pitch = state.pitch.clamp(-MAX_PITCH, MAX_PITCH);

        // Convert pitch and yaw to quaternion rotation
        // Using YXZ Euler order (yaw around Y, pitch around X, no roll)
        let rotation = Quat::from_euler(
            bevy::math::EulerRot::YXZ,
            state.yaw,
            state.pitch,
            0.0,
        );

        transform.rotation = rotation;
    }
}

/// Handles keyboard movement (WASD, QE, Shift)
/// Works regardless of cursor position (no viewport constraint for keyboard)
pub fn handle_viewport_constrained_keyboard_movement(
    mut cameras: Query<
        (&ViewportConstrainedCamera, &mut Transform),
        (With<ViewportConstrainedCamera>, With<RightCamera>),
    >,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (camera, mut transform) in cameras.iter_mut() {
        // Determine movement speed (fast if Shift is held)
        let speed = if keyboard_input.pressed(KeyCode::ShiftLeft)
            || keyboard_input.pressed(KeyCode::ShiftRight)
        {
            camera.speed * camera.fast_speed_multiplier
        } else {
            camera.speed
        };

        // Calculate movement direction based on camera rotation
        // forward(), right(), and up() return Dir3, convert to Vec3 by multiplying by 1.0
        let forward: Vec3 = transform.forward() * 1.0;
        let right: Vec3 = transform.right() * 1.0;
        let up: Vec3 = transform.up() * 1.0;

        let mut movement = Vec3::ZERO;

        // WASD movement
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement += right;
        }

        // QE for up/down movement
        if keyboard_input.pressed(KeyCode::KeyQ) {
            movement -= up;
        }
        if keyboard_input.pressed(KeyCode::KeyE) {
            movement += up;
        }

        // Normalize movement direction if moving in multiple directions
        if movement.length_squared() > 0.0 {
            movement = movement.normalize();
            transform.translation += movement * speed * delta_time;
        }
    }
}

/// Plugin that provides viewport-constrained camera controller
/// Replaces FreeCameraPlugin with built-in viewport constraint support
pub struct ViewportConstrainedCameraPlugin;

impl Plugin for ViewportConstrainedCameraPlugin {
    fn build(&self, app: &mut App) {
        // Initialize resources
        app.init_resource::<CameraViewportCursorState>();

        // Add systems
        // PreUpdate: Update cursor state before camera processes input
        app.add_systems(
            PreUpdate,
            update_viewport_cursor_state,
        );

        // Update: Initialize state, handle mouse rotation, handle keyboard movement
        // Systems run in order: initialization first, then mouse rotation, then keyboard
        app.add_systems(
            Update,
            initialize_viewport_constrained_camera_state,
        );
        app.add_systems(
            Update,
            handle_viewport_constrained_mouse_rotation.after(initialize_viewport_constrained_camera_state),
        );
        app.add_systems(
            Update,
            handle_viewport_constrained_keyboard_movement,
        );
    }
}
