// systems/particle_creation.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{Particle, ParticlePositions, ParticleCreationState, ParticlePlacementMode, ParticleSelectionState, ParticleBoundsState};
use crate::constants::{PARTICLE_RADIUS, COLOR_WHITE, PARTICLE_GRID_BOUNDS};
use rand::Rng;

/// Spawn a single particle at a specific position
pub fn spawn_single_particle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    particle_positions: &mut ParticlePositions,
    position: Vec3,
    white_material: &Handle<StandardMaterial>,
) -> Entity {
    let entity = commands.spawn((
        Mesh3d(meshes.add(Sphere::new(PARTICLE_RADIUS))),
        MeshMaterial3d(white_material.clone()),
        Transform::from_translation(position),
        Particle,
    )).id();
    
    // Store normalized base position (for resizing) and current world position
    // For manually created particles, we'll use the current position as base
    // This might need adjustment if bounds change, but for now we'll store it as-is
    let normalized_x = 0.5; // Default normalized position
    let normalized_y = 0.5;
    let normalized_z = 0.5;
    let base_position = Vec3::new(normalized_x, normalized_y, normalized_z);
    particle_positions.base_positions.insert(entity, base_position);
    particle_positions.current_positions.insert(entity, position);
    
    entity
}

/// Spawn particles randomly within bounds (original behavior)
pub fn spawn_particles_random(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    particle_positions: &mut ParticlePositions,
    bounds_state: Option<&ParticleBoundsState>,
    count: usize,
) {
    let white_material = materials.add(COLOR_WHITE);
    
    // Get bounds from resource or use defaults
    let bounds_x = bounds_state.map(|bs| bs.bounds_x).unwrap_or(PARTICLE_GRID_BOUNDS);
    let bounds_z = bounds_state.map(|bs| bs.bounds_z).unwrap_or(PARTICLE_GRID_BOUNDS);
    let bounds_y_height = bounds_state.map(|bs| bs.bounds_y_height).unwrap_or(1.0);
    let bounds_y_min = 1.0; // Always starts at 1.0
    
    let mut rng = rand::thread_rng();
    
    for _ in 0..count {
        // Generate random normalized positions (0-1 range)
        let normalized_x = rng.gen_range(0.0..=1.0);
        let normalized_z = rng.gen_range(0.0..=1.0);
        let normalized_y = rng.gen_range(0.0..=1.0);
        
        // Convert normalized to world coordinates using current bounds
        let x = (normalized_x - 0.5) * bounds_x;
        let z = (normalized_z - 0.5) * bounds_z;
        let y = bounds_y_min + normalized_y * bounds_y_height;
        
        let position = Vec3::new(x, y, z);
        spawn_single_particle(commands, meshes, materials, particle_positions, position, &white_material);
    }
}

/// Spawn particles randomly inside a sphere
pub fn spawn_particles_in_ball(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    particle_positions: &mut ParticlePositions,
    center: Vec3,
    radius: f32,
    y_min: f32,
    count: usize,
) {
    let white_material = materials.add(COLOR_WHITE);
    let mut rng = rand::thread_rng();
    
    for _ in 0..count {
        // Generate random point inside sphere using rejection sampling
        let mut position;
        loop {
            // Generate point in cube [-radius, radius]^3
            let x = rng.gen_range(-radius..=radius);
            let y = rng.gen_range(-radius..=radius);
            let z = rng.gen_range(-radius..=radius);
            
            // Check if point is inside sphere
            let distance_from_center = (x * x + y * y + z * z).sqrt();
            if distance_from_center <= radius {
                position = center + Vec3::new(x, y, z);
                // Ensure Y is at least y_min
                position.y = position.y.max(y_min);
                break;
            }
        }
        
        spawn_single_particle(commands, meshes, materials, particle_positions, position, &white_material);
    }
}

/// Spawn particles randomly inside a cube (axis-aligned box)
pub fn spawn_particles_in_cube(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    particle_positions: &mut ParticlePositions,
    center: Vec3,
    size: Vec3,
    y_min: f32,
    count: usize,
) {
    let white_material = materials.add(COLOR_WHITE);
    let mut rng = rand::thread_rng();
    
    let half_size = size * 0.5;
    
    for _ in 0..count {
        // Generate random point inside cube
        let x = rng.gen_range(-half_size.x..=half_size.x);
        let y = rng.gen_range(-half_size.y..=half_size.y);
        let z = rng.gen_range(-half_size.z..=half_size.z);
        
        let mut position = center + Vec3::new(x, y, z);
        // Ensure Y is at least y_min
        position.y = position.y.max(y_min);
        
        spawn_single_particle(commands, meshes, materials, particle_positions, position, &white_material);
    }
}

/// System to handle particle creation requests
pub fn handle_particle_creation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut particle_positions: ResMut<ParticlePositions>,
    mut creation_state: ResMut<ParticleCreationState>,
    bounds_state: Option<Res<ParticleBoundsState>>,
) {
    if creation_state.create_requested {
        creation_state.create_requested = false;
        
        match creation_state.placement_mode {
            ParticlePlacementMode::Random => {
                spawn_particles_random(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut particle_positions,
                    bounds_state.as_deref(),
                    creation_state.batch_count,
                );
            }
            ParticlePlacementMode::Ball => {
                spawn_particles_in_ball(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut particle_positions,
                    creation_state.ball_center,
                    creation_state.ball_radius,
                    creation_state.y_min,
                    creation_state.batch_count,
                );
            }
            ParticlePlacementMode::Cube => {
                spawn_particles_in_cube(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut particle_positions,
                    creation_state.cube_center,
                    creation_state.cube_size,
                    creation_state.y_min,
                    creation_state.batch_count,
                );
            }
        }
    }
}

/// System to handle particle removal requests
pub fn handle_particle_removal(
    mut commands: Commands,
    mut particle_positions: ResMut<ParticlePositions>,
    mut creation_state: ResMut<ParticleCreationState>,
    mut selection_state: ResMut<ParticleSelectionState>,
    particle_query: Query<Entity, With<Particle>>,
) {
    if creation_state.remove_all_requested {
        creation_state.remove_all_requested = false;
        
        // Remove all particles
        for entity in particle_query.iter() {
            commands.entity(entity).despawn();
            particle_positions.base_positions.remove(&entity);
            particle_positions.current_positions.remove(&entity);
        }
        
        // Clear selection after removing all particles
        selection_state.selected_particles.clear();
    } else if creation_state.remove_selected_requested {
        creation_state.remove_selected_requested = false;
        
        // Collect entities to remove (to avoid borrowing issues)
        let entities_to_remove: Vec<Entity> = selection_state.selected_particles.iter()
            .filter(|entity| particle_query.contains(**entity))
            .copied()
            .collect();
        
        // Remove selected particles
        for entity in entities_to_remove.iter() {
            commands.entity(*entity).despawn();
            particle_positions.base_positions.remove(entity);
            particle_positions.current_positions.remove(entity);
        }
        
        // Clear selection after removal
        selection_state.selected_particles.clear();
    }
}
