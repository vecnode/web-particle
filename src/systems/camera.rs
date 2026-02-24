// systems/camera.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraState};
use crate::components::CameraViewChanged;

// System to reset FreeCamera after camera view change
// Based on FreeCamera source code analysis:
// - FreeCameraState stores pitch and yaw values
// - When !state.initialized, it reads yaw/pitch from transform.rotation
// - Mouse motion updates state.pitch/yaw, then applies to transform
// - The problem: After changing camera view, FreeCameraState still has old pitch/yaw
// - Solution: Reset FreeCameraState so it reinitializes with new transform rotation
pub fn reset_free_camera_after_view_change(
    mut commands: Commands,
    mut camera_changed: ResMut<CameraViewChanged>,
) {
    if camera_changed.needs_reset {
        if let Some(entity) = camera_changed.entity {
            // Remove both FreeCamera and FreeCameraState components
            // This forces FreeCameraState to reinitialize with the new transform's rotation
            // When FreeCameraState is re-added, it will read yaw/pitch from the updated transform
            commands.entity(entity).remove::<FreeCamera>();
            commands.entity(entity).remove::<FreeCameraState>();
            
            // Re-add FreeCamera (FreeCameraState will be automatically added via #[require])
            // On next update, FreeCameraState will see !initialized and read the new transform's rotation
            commands.entity(entity).insert(FreeCamera::default());
            
            // Reset the flag
            camera_changed.needs_reset = false;
            camera_changed.entity = None;
        }
    }
}

// Keep FreeCamera always present, but block mouse input when cursor is outside viewport
// This allows keyboard and camera buttons to work regardless of cursor position
// Only mouse clicks/rotation are blocked when cursor is over egui panels
pub fn constrain_free_camera_mouse_to_viewport(
    windows: Query<&Window>,
    camera_query: Query<&Camera, With<crate::components::RightCamera>>,
    mut mouse_blocked: ResMut<crate::components::CameraMouseInputBlocked>,
) {
    let Ok(window) = windows.single() else { return };
    let Ok(camera) = camera_query.single() else { return };
    
    // Check if cursor is within camera viewport
    let mut is_in_viewport = false;
    if let Some(viewport) = &camera.viewport {
        if let Some(cursor_pos) = window.cursor_position() {
            let cursor_physical = cursor_pos * window.scale_factor() as f32;
            let viewport_start = viewport.physical_position.as_vec2();
            let viewport_end = viewport_start + viewport.physical_size.as_vec2();
            
            if cursor_physical.x >= viewport_start.x && cursor_physical.x < viewport_end.x &&
               cursor_physical.y >= viewport_start.y && cursor_physical.y < viewport_end.y {
                is_in_viewport = true;
            }
        }
    }
    
    // Block mouse input when cursor is outside viewport
    mouse_blocked.is_blocked = !is_in_viewport;
}

// Store camera rotation before FreeCamera processes input
// This allows us to restore it if mouse input was blocked
pub fn block_camera_mouse_input_before_freecamera(
    camera_query: Query<(Entity, &Transform), (With<FreeCamera>, With<crate::components::RightCamera>)>,
    mouse_blocked: Res<crate::components::CameraMouseInputBlocked>,
    mut previous_rotations: Local<std::collections::HashMap<Entity, Quat>>,
) {
    if mouse_blocked.is_blocked {
        for (entity, transform) in camera_query.iter() {
            // Store rotation before FreeCamera processes input
            previous_rotations.insert(entity, transform.rotation);
        }
    } else {
        // Clear stored rotations when mouse is not blocked
        previous_rotations.clear();
    }
}

// Restore camera rotation if mouse input was blocked (undoes mouse rotation)
// This preserves keyboard movement but prevents mouse rotation
// Runs in PostUpdate after FreeCameraPlugin processes input
pub fn restore_camera_after_blocked_mouse(
    mut camera_query: Query<(Entity, &mut Transform), (With<FreeCamera>, With<crate::components::RightCamera>)>,
    mouse_blocked: Res<crate::components::CameraMouseInputBlocked>,
    previous_rotations: Local<std::collections::HashMap<Entity, Quat>>,
) {
    if mouse_blocked.is_blocked {
        for (entity, mut transform) in camera_query.iter_mut() {
            if let Some(previous_rotation) = previous_rotations.get(&entity) {
                // Restore previous rotation to undo any mouse rotation
                // Position changes from keyboard are preserved
                transform.rotation = *previous_rotation;
            }
        }
    }
}
