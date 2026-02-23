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

