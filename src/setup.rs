// setup.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::constants::*;

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
