// setup.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy::camera_controller::free_camera::FreeCamera;
use crate::constants::*;
use crate::components::Particle;

pub fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let white_material = materials.add(COLOR_WHITE);
    
    for i in 0..PARTICLE_COUNT {
        // Simple pseudo-random distribution based on index
        let x = ((i * 17 + 7) % 100) as f32 / 100.0 * GRID_BOUNDS * 2.0 - GRID_BOUNDS;
        let z = ((i * 23 + 11) % 100) as f32 / 100.0 * GRID_BOUNDS * 2.0 - GRID_BOUNDS;
        let y = ((i * 13 + 3) % 100) as f32 / 100.0 * 2.0; // Random height 0-2
        
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(PARTICLE_RADIUS))),
            MeshMaterial3d(white_material.clone()),
            Transform::from_translation(Vec3::new(x, y, z)),
            Particle,
        ));
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
) {
    let half_size = GRID_SIZE / 2.0;
    let num_lines = (GRID_SIZE / GRID_SPACING) as i32 + 1;
    
    // Create grid lines along X axis (parallel to Z)
    for i in 0..num_lines {
        let z = -half_size + (i as f32 * GRID_SPACING);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, GRID_SIZE))),
            MeshMaterial3d(materials.add(GRID_COLOR)),
            Transform::from_translation(Vec3::new(0.0, 0.0, z))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ));
    }
    
    // Create grid lines along Z axis (parallel to X)
    for i in 0..num_lines {
        let x = -half_size + (i as f32 * GRID_SPACING);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(GRID_LINE_RADIUS, GRID_SIZE))),
            MeshMaterial3d(materials.add(GRID_COLOR)),
            Transform::from_translation(Vec3::new(x, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ));
    }
}

pub fn setup_camera_and_lights(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(CAMERA_START_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
    ));
    
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
