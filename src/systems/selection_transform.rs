// systems/selection_transform.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{ParticleSelectionState, SelectionTransformState, Particle, ParticlePositions};

/// System to update original positions when selection changes and reset transforms
pub fn update_selection_original_positions(
    mut transform_state: ResMut<SelectionTransformState>,
    selection_state: Res<ParticleSelectionState>,
    particle_query: Query<(Entity, &Transform), With<Particle>>,
    mut particle_positions: ResMut<ParticlePositions>,
) {
    // Create a hash of the current selection to detect changes
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    let mut sorted_entities: Vec<Entity> = selection_state.selected_particles.iter().copied().collect();
    sorted_entities.sort();
    sorted_entities.hash(&mut hasher);
    let current_selection_hash = hasher.finish();
    
    // Check if selection changed
    if current_selection_hash != transform_state.previous_selection_hash {
        // Selection changed - update original positions and reset transforms
        transform_state.original_selection_positions.clear();
        transform_state.previous_selection_hash = current_selection_hash;
        
        // Reset transform values to defaults when selection changes
        transform_state.position_offset = Vec3::ZERO;
        transform_state.scale = Vec3::ONE;
        transform_state.previous_position_offset = Vec3::ZERO;
        transform_state.previous_scale = Vec3::ONE;
        
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
/// This always runs to ensure transforms are applied whenever selection or values change
pub fn update_selection_transform(
    mut particle_query: Query<(Entity, &mut Transform), With<Particle>>,
    mut transform_state: ResMut<SelectionTransformState>,
    mut particle_positions: ResMut<ParticlePositions>,
    selection_state: Res<ParticleSelectionState>,
) {
    // Always apply transforms if there are selected particles with stored original positions
    if !selection_state.selected_particles.is_empty() && !transform_state.original_selection_positions.is_empty() {
        // Update previous values if they changed (for change detection)
        if transform_state.position_offset != transform_state.previous_position_offset ||
           transform_state.scale != transform_state.previous_scale {
            transform_state.previous_position_offset = transform_state.position_offset;
            transform_state.previous_scale = transform_state.scale;
        }
        
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
