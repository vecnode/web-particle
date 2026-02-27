// systems/selection_bounds.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{ParticleSelectionState, SelectionBoundingBox, Particle};

const SELECTION_BOX_LINE_RADIUS: f32 = 0.01;
const SELECTION_BOX_COLOR: Color = Color::srgb(0.7, 0.7, 0.7); // Light gray

/// System to update the selection bounding box wireframe
pub fn update_selection_bounding_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selection_state: Res<ParticleSelectionState>,
    particle_query: Query<&Transform, (With<Particle>, Without<SelectionBoundingBox>)>,
    bounding_box_query: Query<Entity, With<SelectionBoundingBox>>,
) {
    // Remove existing bounding box if no particles are selected
    if selection_state.selected_particles.is_empty() {
        for entity in bounding_box_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }
    
    // Calculate bounding box from selected particles
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    let mut min_z = f32::MAX;
    let mut max_z = f32::MIN;
    
    let mut has_particles = false;
    for entity in selection_state.selected_particles.iter() {
        if let Ok(transform) = particle_query.get(*entity) {
            let pos = transform.translation;
            min_x = min_x.min(pos.x);
            max_x = max_x.max(pos.x);
            min_y = min_y.min(pos.y);
            max_y = max_y.max(pos.y);
            min_z = min_z.min(pos.z);
            max_z = max_z.max(pos.z);
            has_particles = true;
        }
    }
    
    if !has_particles {
        // Remove bounding box if no valid particles found
        for entity in bounding_box_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }
    
    // Add padding to the bounding box
    let padding = 0.1;
    min_x -= padding;
    max_x += padding;
    min_y -= padding;
    max_y += padding;
    min_z -= padding;
    max_z += padding;
    
    // Calculate box dimensions
    let width = max_x - min_x;
    let height = max_y - min_y;
    let depth = max_z - min_z;
    let center = Vec3::new(
        (min_x + max_x) * 0.5,
        (min_y + max_y) * 0.5,
        (min_z + max_z) * 0.5,
    );
    
    // Remove existing bounding box before creating new one
    for entity in bounding_box_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Create wireframe material
    let wireframe_material = materials.add(StandardMaterial {
        base_color: SELECTION_BOX_COLOR,
        unlit: true, // Make it visible regardless of lighting
        ..default()
    });
    
    // Create 12 edges of the bounding box using thin cylinders
    // Bottom face (z = min_z): 4 edges
    // Front edge (X direction, at y = min_y)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, width))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(center.x, min_y, min_z))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        SelectionBoundingBox,
    ));
    // Back edge (X direction, at y = max_y)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, width))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(center.x, max_y, min_z))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        SelectionBoundingBox,
    ));
    // Left edge (Y direction, at x = min_x) - connects front-left to back-left
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, height))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(min_x, center.y, min_z)),
        SelectionBoundingBox,
    ));
    // Right edge (Y direction, at x = max_x) - connects front-right to back-right
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, height))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(max_x, center.y, min_z)),
        SelectionBoundingBox,
    ));
    
    // Top face (z = max_z): 4 edges
    // Front edge (X direction, at y = min_y)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, width))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(center.x, min_y, max_z))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        SelectionBoundingBox,
    ));
    // Back edge (X direction, at y = max_y)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, width))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(center.x, max_y, max_z))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        SelectionBoundingBox,
    ));
    // Left edge (Y direction, at x = min_x) - connects front-left to back-left
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, height))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(min_x, center.y, max_z)),
        SelectionBoundingBox,
    ));
    // Right edge (Y direction, at x = max_x) - connects front-right to back-right
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, height))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(max_x, center.y, max_z)),
        SelectionBoundingBox,
    ));
    
    // Vertical edges (4 edges connecting bottom to top)
    // Front-left (Z direction, at x = min_x, y = min_y)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, depth))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(min_x, min_y, center.z))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        SelectionBoundingBox,
    ));
    // Front-right (Z direction, at x = max_x, y = min_y)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, depth))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(max_x, min_y, center.z))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        SelectionBoundingBox,
    ));
    // Back-left (Z direction, at x = min_x, y = max_y)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, depth))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(min_x, max_y, center.z))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        SelectionBoundingBox,
    ));
    // Back-right (Z direction, at x = max_x, y = max_y)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(SELECTION_BOX_LINE_RADIUS, depth))),
        MeshMaterial3d(wireframe_material.clone()),
        Transform::from_translation(Vec3::new(max_x, max_y, center.z))
            .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        SelectionBoundingBox,
    ));
}
