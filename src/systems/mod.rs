// systems/mod.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;

pub mod camera;
pub mod particles;
pub mod selection;
pub mod egui_ui;
pub mod mouse;
pub mod grid;

pub use camera::reset_free_camera_after_view_change;
pub use particles::*;
pub use selection::*;
pub use egui_ui::egui_controls_ui;
pub use mouse::*;
pub use grid::update_grid_dimensions;

pub fn animate_motion1_particles(
    time: Res<Time>,
    motion1_state: Res<crate::components::Motion1State>,
    selection_state: Res<crate::components::ParticleSelectionState>,
    mut particle_query: Query<(Entity, &mut Transform), With<crate::components::Particle>>,
    mut particle_positions: ResMut<crate::components::ParticlePositions>,
) {
    if !motion1_state.is_active {
        return;
    }
    
    let delta_time = time.delta_secs();
    let rotation_delta = motion1_state.rotation_speed * delta_time;
    
    for entity in &selection_state.selected_particles {
        if let Ok((_, mut transform)) = particle_query.get_mut(*entity) {
            let current_pos = transform.translation;
            
            // Calculate radius in XZ plane (horizontal distance from center at Vec3::ZERO)
            let xz_pos = Vec3::new(current_pos.x, 0.0, current_pos.z);
            let radius = xz_pos.length();
            
            if radius > 0.001 {
                // Calculate current angle in XZ plane
                let current_angle = current_pos.z.atan2(current_pos.x);
                
                // Rotate clockwise (increase angle)
                let new_angle = current_angle + rotation_delta;
                
                // Calculate new XZ position maintaining radius
                let new_x = radius * new_angle.cos();
                let new_z = radius * new_angle.sin();
                
                // Update position maintaining Y height
                transform.translation = Vec3::new(new_x, current_pos.y, new_z);
                
                // Update global position state
                particle_positions.current_positions.insert(*entity, transform.translation);
            }
        }
    }
}

pub fn update_trajectory_visualization(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    trajectory_state: Res<crate::components::TrajectoryState>,
    selection_state: Res<crate::components::ParticleSelectionState>,
    particle_query: Query<&Transform, With<crate::components::Particle>>,
    trajectory_query: Query<(Entity, &crate::components::TrajectoryCircle)>,
) {
    use crate::constants::TRAJECTORY_CIRCLE_THICKNESS;
    use crate::constants::TRAJECTORY_COLOR;
    
    if trajectory_state.is_visible {
        // Spawn trajectory circles for selected particles that don't have one yet
        for particle_entity in &selection_state.selected_particles {
            // Check if trajectory already exists for this particle
            let has_trajectory = trajectory_query.iter().any(|(_, circle)| circle.particle_entity == *particle_entity);
            
            if !has_trajectory {
                if let Ok(transform) = particle_query.get(*particle_entity) {
                    let pos = transform.translation;
                    
                    // Calculate radius in XZ plane
                    let xz_pos = Vec3::new(pos.x, 0.0, pos.z);
                    let radius = xz_pos.length().max(0.1); // Minimum radius to avoid zero-size circles
                    
                    // Create a high-resolution torus (ring) for the trajectory circle
                    // Bevy's Torus primitive provides good quality by default
                    let torus = bevy::prelude::Torus {
                        major_radius: radius,
                        minor_radius: TRAJECTORY_CIRCLE_THICKNESS,
                    };
                    
                    let trajectory_material = materials.add(StandardMaterial {
                        base_color: TRAJECTORY_COLOR,
                        unlit: true, // Make it visible regardless of lighting
                        ..default()
                    });
                    
                    commands.spawn((
                        Mesh3d(meshes.add(torus)),
                        MeshMaterial3d(trajectory_material),
                        Transform::from_translation(Vec3::new(0.0, pos.y, 0.0)),
                        crate::components::TrajectoryCircle {
                            particle_entity: *particle_entity,
                        },
                    ));
                }
            }
        }
        
        // Remove trajectory circles for particles that are no longer selected
        for (trajectory_entity, circle) in trajectory_query.iter() {
            if !selection_state.selected_particles.contains(&circle.particle_entity) {
                commands.entity(trajectory_entity).despawn();
            }
        }
    } else {
        // Remove all trajectory circles when hidden
        for (trajectory_entity, _) in trajectory_query.iter() {
            commands.entity(trajectory_entity).despawn();
        }
    }
}


