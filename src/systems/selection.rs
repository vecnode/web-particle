// systems/selection.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{SelectionBox, SelectionBoxState, Particle, Selected, ParticleSelectionState};
use crate::constants::{SELECTION_BOX_COLOR, COLOR_PURPLE, COLOR_WHITE};

pub fn handle_right_mouse_button(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut selection_box_state: ResMut<SelectionBoxState>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        let Ok(window) = windows.single() else { return };
        
        // Only start selection if cursor is over the camera viewport (not over Egui panels)
        let Some(cursor_pos) = window.cursor_position() else { return };
        let cursor_physical = cursor_pos * window.scale_factor() as f32;
        
        // Check if cursor is within camera viewport
        let mut is_in_viewport = false;
        for (camera, _) in camera_query.iter() {
            if let Some(viewport) = &camera.viewport {
                let viewport_start = viewport.physical_position.as_vec2();
                let viewport_end = viewport_start + viewport.physical_size.as_vec2();
                if cursor_physical.x >= viewport_start.x && cursor_physical.x < viewport_end.x &&
                   cursor_physical.y >= viewport_start.y && cursor_physical.y < viewport_end.y {
                    is_in_viewport = true;
                    break;
                }
            }
        }
        
        if is_in_viewport {
            selection_box_state.is_active = true;
            selection_box_state.start_position = Some(cursor_pos);
            selection_box_state.current_position = Some(cursor_pos);
        }
    }
    
    if mouse_button_input.just_released(MouseButton::Right) {
        selection_box_state.is_active = false;
    }
}

pub fn update_selection_box_visual(
    windows: Query<&Window>,
    mut selection_box_state: ResMut<SelectionBoxState>,
    mut selection_box_query: Query<(Entity, &mut Node), With<SelectionBox>>,
    mut commands: Commands,
) {
    let Ok(window) = windows.single() else { return };
    
    if selection_box_state.is_active {
        if let Some(cursor_pos) = window.cursor_position() {
            selection_box_state.current_position = Some(cursor_pos);
        }
        
        if let (Some(start), Some(current)) = (selection_box_state.start_position, selection_box_state.current_position) {
            let left = start.x.min(current.x);
            let top = start.y.min(current.y);
            let width = (current.x - start.x).abs();
            let height = (current.y - start.y).abs();
            
            if width > 1.0 && height > 1.0 {
                if selection_box_query.is_empty() {
                    commands.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(left),
                            top: Val::Px(top),
                            width: Val::Px(width),
                            height: Val::Px(height),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BackgroundColor(SELECTION_BOX_COLOR),
                        SelectionBox,
                    ));
                } else {
                    for (_, mut node) in selection_box_query.iter_mut() {
                        node.left = Val::Px(left);
                        node.top = Val::Px(top);
                        node.width = Val::Px(width);
                        node.height = Val::Px(height);
                    }
                }
            }
        }
    } else {
        for (entity, _) in selection_box_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn process_selection_box(
    mut selection_box_state: ResMut<SelectionBoxState>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    particle_query: Query<(Entity, &Transform), With<Particle>>,
    mut selected_query: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>), (With<Particle>, With<Selected>)>,
    mut unselected_query: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>), (With<Particle>, Without<Selected>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut particle_selection_state: ResMut<ParticleSelectionState>,
) {
    if selection_box_state.is_active {
        return;
    }
    
    let (Some(start), Some(end)) = (selection_box_state.start_position.take(), selection_box_state.current_position.take()) else {
        return;
    };
    
    let drag_distance = (end - start).length();
    const MIN_DRAG_DISTANCE: f32 = 5.0;
    
    if drag_distance < MIN_DRAG_DISTANCE {
        for entity in particle_selection_state.selected_particles.clone() {
            if let Ok((_, mut material)) = selected_query.get_mut(entity) {
                material.0 = materials.add(COLOR_WHITE);
                commands.entity(entity).remove::<Selected>();
                particle_selection_state.selected_particles.remove(&entity);
            }
        }
        return;
    }
    
    let Ok(window) = windows.single() else { return };
    
    // Find camera whose viewport contains the selection box center
    let box_center = (start + end) * 0.5;
    let cursor_physical = box_center * window.scale_factor() as f32;
    
    let mut selected_camera = None;
    for (camera, camera_transform) in camera_query.iter() {
        if let Some(viewport) = &camera.viewport {
            let viewport_start = viewport.physical_position.as_vec2();
            let viewport_end = viewport_start + viewport.physical_size.as_vec2();
            if cursor_physical.x >= viewport_start.x && cursor_physical.x < viewport_end.x &&
               cursor_physical.y >= viewport_start.y && cursor_physical.y < viewport_end.y {
                selected_camera = Some((camera, camera_transform));
                break;
            }
        } else {
            // If no viewport, use this camera (fallback)
            selected_camera = Some((camera, camera_transform));
            break;
        }
    }
    
    let Some((camera, camera_transform)) = selected_camera else { return };
    
    // Get viewport information for coordinate conversion
    let viewport = camera.viewport.as_ref().expect("Camera should have viewport");
    let viewport_physical_start = viewport.physical_position.as_vec2();
    let viewport_physical_size = viewport.physical_size.as_vec2();
    let scale_factor = window.scale_factor() as f32;
    
    // Convert selection box coordinates from logical to physical, then to viewport-relative
    let start_physical = start * scale_factor;
    let end_physical = end * scale_factor;
    
    // Make coordinates relative to viewport
    let left_physical = (start_physical.x.min(end_physical.x) - viewport_physical_start.x).max(0.0);
    let right_physical = (start_physical.x.max(end_physical.x) - viewport_physical_start.x).min(viewport_physical_size.x);
    let top_physical = (start_physical.y.min(end_physical.y) - viewport_physical_start.y).max(0.0);
    let bottom_physical = (start_physical.y.max(end_physical.y) - viewport_physical_start.y).min(viewport_physical_size.y);
    
    for (entity, transform) in particle_query.iter() {
        let world_pos = transform.translation;
        
        let Some(ndc) = camera.world_to_ndc(camera_transform, world_pos) else { continue };
        
        // Convert NDC to viewport-relative screen coordinates
        // NDC: -1 to 1, where (0,0) is center, (-1,-1) is bottom-left, (1,1) is top-right
        // Screen: 0 to viewport_size, where (0,0) is top-left
        let screen_x = (ndc.x * 0.5 + 0.5) * viewport_physical_size.x;
        let screen_y = (1.0 - (ndc.y * 0.5 + 0.5)) * viewport_physical_size.y;
        
        // Check if particle is within selection box (in viewport coordinates)
        if screen_x >= left_physical && screen_x <= right_physical &&
           screen_y >= top_physical && screen_y <= bottom_physical {
            if !particle_selection_state.selected_particles.contains(&entity) {
                if let Ok((_, mut material)) = unselected_query.get_mut(entity) {
                    material.0 = materials.add(COLOR_PURPLE);
                    commands.entity(entity).insert(Selected);
                    particle_selection_state.selected_particles.insert(entity);
                }
            }
        }
    }
}
