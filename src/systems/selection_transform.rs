// systems/selection_transform.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{ParticleSelectionState, SelectionTransformState, Particle, ParticlePositions};

/// System to update original positions when selection changes
pub fn update_selection_original_positions(
    mut transform_state: ResMut<SelectionTransformState>,
    selection_state: Res<ParticleSelectionState>,
    particle_query: Query<(Entity, &Transform), With<Particle>>,
    mut particle_positions: ResMut<ParticlePositions>,
) {
    // Check if selection changed by comparing current selection with stored original positions
    let current_selection: std::collections::HashSet<Entity> = selection_state.selected_particles.iter().copied().collect();
    let stored_selection: std::collections::HashSet<Entity> = transform_state.original_selection_positions.keys().copied().collect();
    
    if current_selection != stored_selection {
        // Selection changed - update original positions
        transform_state.original_selection_positions.clear();
        
        for entity in selection_state.selected_particles.iter() {
            if let Ok((_, transform)) = particle_query.get(*entity) {
                // Store current position as original (before any transforms)
                let current_pos = transform.translation;
                transform_state.original_selection_positions.insert(*entity, current_pos);
                // Also update particle_positions to track this
                particle_positions.current_positions.insert(*entity, current_pos);
            }
        }
    }
}

/// System to apply position offset and scale to selected particles only
pub fn update_selection_transform(
    mut particle_query: Query<(Entity, &mut Transform), With<Particle>>,
    mut transform_state: ResMut<SelectionTransformState>,
    mut particle_positions: ResMut<ParticlePositions>,
    selection_state: Res<ParticleSelectionState>,
) {
    // Check if transform changed
    if transform_state.position_offset != transform_state.previous_position_offset ||
       transform_state.scale != transform_state.previous_scale {
        
        // Update previous values
        transform_state.previous_position_offset = transform_state.position_offset;
        transform_state.previous_scale = transform_state.scale;
        
        // Calculate center from original positions
        let mut center = Vec3::ZERO;
        let mut count = 0;
        
        for entity in selection_state.selected_particles.iter() {
            if let Some(&original_pos) = transform_state.original_selection_positions.get(entity) {
                center += original_pos;
                count += 1;
            }
        }
        
        if count > 0 {
            center /= count as f32;
        }
        
        // Apply transform only to selected particles
        for entity in selection_state.selected_particles.iter() {
            if let Ok((_, mut transform)) = particle_query.get_mut(*entity) {
                if let Some(&original_pos) = transform_state.original_selection_positions.get(entity) {
                    // Get position relative to original center
                    let relative_pos = original_pos - center;
                    
                    // Apply scale (normal distribution)
                    let scaled_relative = Vec3::new(
                        relative_pos.x * transform_state.scale.x,
                        relative_pos.y * transform_state.scale.y,
                        relative_pos.z * transform_state.scale.z,
                    );
                    
                    // Apply position offset and restore center
                    transform.translation = center + scaled_relative + transform_state.position_offset;
                    
                    // Update stored position
                    particle_positions.current_positions.insert(*entity, transform.translation);
                }
            }
        }
    }
}
