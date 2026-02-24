// setup.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::constants::*;
use crate::components::{Particle, ParticlePositions};

pub fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut particle_positions: ResMut<ParticlePositions>,
    bounds_state: Option<Res<crate::components::ParticleBoundsState>>,
) {
    let white_material = materials.add(COLOR_WHITE);
    
    // Get bounds from resource or use defaults
    let bounds_x = bounds_state.as_ref().map(|bs| bs.bounds_x).unwrap_or(PARTICLE_GRID_BOUNDS);
    let bounds_z = bounds_state.as_ref().map(|bs| bs.bounds_z).unwrap_or(PARTICLE_GRID_BOUNDS);
    let bounds_y_height = bounds_state.as_ref().map(|bs| bs.bounds_y_height).unwrap_or(1.0);
    let bounds_y_min = 1.0;  // Always starts at 1.0
    
    for i in 0..PARTICLE_COUNT {
        // Simple pseudo-random distribution based on index
        // Generate normalized positions (0-1 range)
        let normalized_x = ((i * 17 + 7) % 100) as f32 / 100.0;
        let normalized_z = ((i * 23 + 11) % 100) as f32 / 100.0;
        let normalized_y = ((i * 13 + 3) % 100) as f32 / 100.0;
        
        // Convert normalized to world coordinates using current bounds
        // bounds_x is now total size (diameter), so calculate: (normalized - 0.5) * total_size
        let x = (normalized_x - 0.5) * bounds_x;
        let z = (normalized_z - 0.5) * bounds_z;
        let y = bounds_y_min + normalized_y * bounds_y_height;
        
        let position = Vec3::new(x, y, z);
        let entity = commands.spawn((
            Mesh3d(meshes.add(Sphere::new(PARTICLE_RADIUS))),
            MeshMaterial3d(white_material.clone()),
            Transform::from_translation(position),
            Particle,
        )).id();
        
        // Store normalized base position (for resizing) and current world position
        let base_position = Vec3::new(normalized_x, normalized_y, normalized_z);
        particle_positions.base_positions.insert(entity, base_position);
        particle_positions.current_positions.insert(entity, position);
    }
}

pub fn spawn_axes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // X axis (red)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(AXIS_RADIUS, AXIS_LENGTH))),
        MeshMaterial3d(materials.add(COLOR_RED)),
        Transform::from_translation(Vec3::X * AXIS_LENGTH / 2.0)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
    ));
    
    // Y axis (green)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(AXIS_RADIUS, AXIS_LENGTH))),
        MeshMaterial3d(materials.add(COLOR_GREEN)),
        Transform::from_translation(Vec3::Y * AXIS_LENGTH / 2.0),
    ));
    
    // Z axis (blue)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(AXIS_RADIUS, AXIS_LENGTH))),
        MeshMaterial3d(materials.add(COLOR_BLUE)),
        Transform::from_translation(Vec3::Z * AXIS_LENGTH / 2.0)
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

pub fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    grid_state: Option<Res<crate::components::GridState>>,
) {
    // Use grid_state if available, otherwise use default values (10x10)
    let size_x = grid_state.as_ref().map(|gs| gs.size_x).unwrap_or(10) as f32;
    let size_z = grid_state.as_ref().map(|gs| gs.size_z).unwrap_or(10) as f32;
    let half_size_x = size_x / 2.0;
    let half_size_z = size_z / 2.0;
    let num_lines_x = grid_state.as_ref().map(|gs| gs.size_x).unwrap_or(10) + 1;
    let num_lines_z = grid_state.as_ref().map(|gs| gs.size_z).unwrap_or(10) + 1;
    
    // Create grid lines along X axis (parallel to Z) - these lines span the X direction
    for i in 0..num_lines_z {
        let z = -half_size_z + (i as f32 * GRID_SPACING);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_x))),
            MeshMaterial3d(materials.add(GRID_COLOR)),
            Transform::from_translation(Vec3::new(0.0, 0.0, z))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            crate::components::GridLine,
        ));
    }
    
    // Create grid lines along Z axis (parallel to X) - these lines span the Z direction
    for i in 0..num_lines_x {
        let x = -half_size_x + (i as f32 * GRID_SPACING);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_z))),
            MeshMaterial3d(materials.add(GRID_COLOR)),
            Transform::from_translation(Vec3::new(x, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            crate::components::GridLine,
        ));
    }
}

pub fn setup_camera_and_lights(mut commands: Commands) {
    // Front light
    commands.spawn(DirectionalLight {
        illuminance: FRONT_LIGHT_ILLUMINANCE,
        ..default()
    });
    
    // Back light (from behind)
    commands.spawn((
        DirectionalLight {
            illuminance: BACK_LIGHT_ILLUMINANCE,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
    ));
}
