// systems/camera.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraState};
use crate::components::{FixCameraButton, CameraTopButton, CameraPositionText, CameraViewChanged};
use crate::constants::{CAMERA_FRONT_POSITION, CAMERA_TOP_POSITION};

pub fn handle_camera_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<FixCameraButton>)>,
    mut camera_query: Query<(Entity, &mut Transform, &mut GlobalTransform), (With<Camera3d>, With<FreeCamera>)>,
    mut camera_changed: ResMut<CameraViewChanged>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for (entity, mut transform, mut global_transform) in camera_query.iter_mut() {
                update_camera_view(&mut transform, &mut global_transform, entity, &mut camera_changed, CAMERA_FRONT_POSITION, Vec3::ZERO, Vec3::Y);
            }
        }
    }
}

pub fn handle_camera_top_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CameraTopButton>)>,
    mut camera_query: Query<(Entity, &mut Transform, &mut GlobalTransform), (With<Camera3d>, With<FreeCamera>)>,
    mut camera_changed: ResMut<CameraViewChanged>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for (entity, mut transform, mut global_transform) in camera_query.iter_mut() {
                update_camera_view(&mut transform, &mut global_transform, entity, &mut camera_changed, CAMERA_TOP_POSITION, Vec3::ZERO, Vec3::Z);
            }
        }
    }
}

pub fn update_camera_view(
    transform: &mut Transform,
    global_transform: &mut GlobalTransform,
    entity: Entity,
    camera_changed: &mut ResMut<CameraViewChanged>,
    position: Vec3,
    target: Vec3,
    up: Vec3,
) {
    // Update camera transform first
    transform.translation = position;
    transform.look_at(target, up);
    *global_transform = GlobalTransform::from(*transform);
    
    // Mark that camera view changed - we'll reset FreeCamera immediately
    // This ensures FreeCamera reinitializes with the new transform
    camera_changed.needs_reset = true;
    camera_changed.entity = Some(entity);
}

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

pub fn update_camera_position_text(
    camera_query: Query<&Transform, (With<Camera3d>, With<crate::components::RightCamera>)>,
    mut text_query: Query<&mut Text, With<CameraPositionText>>,
) {
    if let Ok(transform) = camera_query.single() {
        let pos = transform.translation;
        let x = pos.x.round() as i32;
        let y = pos.y.round() as i32;
        let z = pos.z.round() as i32;
        
        if let Ok(mut text) = text_query.single_mut() {
            *text = Text::new(format!("X: {}\nY: {}\nZ: {}", x, y, z));
        }
    }
}
