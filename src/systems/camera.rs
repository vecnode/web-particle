// systems/camera.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::CameraViewChanged;
use crate::plugins::viewport_constrained_camera::ViewportConstrainedCameraState;

// System to reset ViewportConstrainedCamera after camera view change
// Resets the camera state so it reinitializes with the new transform's rotation
pub fn reset_viewport_constrained_camera_after_view_change(
    mut commands: Commands,
    mut camera_changed: ResMut<CameraViewChanged>,
) {
    if camera_changed.needs_reset {
        if let Some(entity) = camera_changed.entity {
            // Remove ViewportConstrainedCameraState to force reinitialization
            // The state will be re-initialized from the transform's rotation on next frame
            commands.entity(entity).remove::<ViewportConstrainedCameraState>();
            
            // Re-add ViewportConstrainedCameraState with initialized=false
            // This will cause initialize_viewport_constrained_camera_state to read the new rotation
            commands.entity(entity).insert(ViewportConstrainedCameraState {
                pitch: 0.0,
                yaw: 0.0,
                initialized: false,
            });
            
            // Reset the flag
            camera_changed.needs_reset = false;
            camera_changed.entity = None;
        }
    }
}
