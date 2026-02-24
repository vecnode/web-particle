// systems/grid.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{GridState, GridLine};

pub fn update_grid_dimensions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid_state: ResMut<GridState>,
    grid_line_query: Query<Entity, With<GridLine>>,
) {
    // Check if grid dimensions changed
    if grid_state.size_x != grid_state.previous_size_x || grid_state.size_z != grid_state.previous_size_z {
        // Despawn all existing grid lines
        for entity in grid_line_query.iter() {
            commands.entity(entity).despawn();
        }
        
        // Update previous values
        grid_state.previous_size_x = grid_state.size_x;
        grid_state.previous_size_z = grid_state.size_z;
        
        // Respawn grid with new dimensions
        // Extract values from grid_state
        let size_x = grid_state.size_x;
        let size_z = grid_state.size_z;
        
        // Spawn grid lines directly here instead of calling spawn_grid
        let size_x_f = size_x as f32;
        let size_z_f = size_z as f32;
        let half_size_x = size_x_f / 2.0;
        let half_size_z = size_z_f / 2.0;
        let num_lines_x = size_x + 1;
        let num_lines_z = size_z + 1;
        
        use crate::constants::{GRID_LINE_RADIUS, GRID_COLOR, GRID_SPACING};
        
        // Create grid lines along X axis (parallel to Z) - these lines span the X direction
        for i in 0..num_lines_z {
            let z = -half_size_z + (i as f32 * GRID_SPACING);
            commands.spawn((
                Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_x_f))),
                MeshMaterial3d(materials.add(GRID_COLOR)),
                Transform::from_translation(Vec3::new(0.0, 0.0, z))
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                GridLine,
            ));
        }
        
        // Create grid lines along Z axis (parallel to X) - these lines span the Z direction
        for i in 0..num_lines_x {
            let x = -half_size_x + (i as f32 * GRID_SPACING);
            commands.spawn((
                Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, size_z_f))),
                MeshMaterial3d(materials.add(GRID_COLOR)),
                Transform::from_translation(Vec3::new(x, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                GridLine,
            ));
        }
    }
}
