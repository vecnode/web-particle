// systems/particles.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{Particle, Selected};
use crate::constants::{PARTICLE_RADIUS, COLOR_WHITE, COLOR_PURPLE};
use crate::components::ParticleSelectionState;

pub fn handle_particle_selection(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut particle_query: Query<(Entity, &Transform, &mut MeshMaterial3d<StandardMaterial>), (With<Particle>, Without<Selected>)>,
    mut selected_query: Query<(Entity, &Transform, &mut MeshMaterial3d<StandardMaterial>), (With<Particle>, With<Selected>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut selection_state: ResMut<ParticleSelectionState>,
) {
    // Only handle clicks when mouse button is just released (not during camera drag)
    // This ensures we don't interfere with camera rotation
    // Note: bevy_egui automatically filters out input when cursor is over Egui UI
    if !mouse_button_input.just_released(MouseButton::Left) {
        return;
    }
    
    let Ok(window) = windows.single() else { return };
    
    // Find camera whose viewport contains the cursor
    let cursor_pos = window.cursor_position().unwrap_or_default();
    let cursor_physical = cursor_pos * window.scale_factor() as f32;
    
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
        }
    }
    
    let Some((camera, camera_transform)) = selected_camera else { return };
    
    // Get mouse position relative to viewport
    let Some(cursor_pos) = window.cursor_position() else { return };
    
    // Convert screen position to world ray - viewport_to_world handles viewport offset automatically
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else { return };
    
    // Find closest particle hit by ray
    let closest_hit = find_closest_particle_hit(ray, &particle_query, &selected_query);
    
    // Toggle selection of closest hit particle
    if let Some(entity) = closest_hit {
        toggle_particle_selection(
            entity,
            &mut particle_query,
            &mut selected_query,
            &mut materials,
            &mut commands,
            &mut selection_state,
        );
    }
}

fn find_closest_particle_hit(
    ray: impl std::borrow::Borrow<bevy::math::Ray3d>,
    particle_query: &Query<(Entity, &Transform, &mut MeshMaterial3d<StandardMaterial>), (With<Particle>, Without<Selected>)>,
    selected_query: &Query<(Entity, &Transform, &mut MeshMaterial3d<StandardMaterial>), (With<Particle>, With<Selected>)>,
) -> Option<Entity> {
    let ray = ray.borrow();
    let mut closest_hit: Option<(Entity, f32)> = None;
    let ray_dir = *ray.direction;
    
    // Check unselected particles
    for (entity, transform, _) in particle_query.iter() {
        if let Some(distance) = raycast_particle(ray, transform, ray_dir) {
            if let Some((_, closest_dist)) = closest_hit {
                if distance < closest_dist {
                    closest_hit = Some((entity, distance));
                }
            } else {
                closest_hit = Some((entity, distance));
            }
        }
    }
    
    // Check selected particles
    for (entity, transform, _) in selected_query.iter() {
        if let Some(distance) = raycast_particle(ray, transform, ray_dir) {
            if let Some((_, closest_dist)) = closest_hit {
                if distance < closest_dist {
                    closest_hit = Some((entity, distance));
                }
            } else {
                closest_hit = Some((entity, distance));
            }
        }
    }
    
    closest_hit.map(|(entity, _)| entity)
}

fn raycast_particle(
    ray: &bevy::math::Ray3d,
    transform: &Transform,
    ray_dir: Vec3,
) -> Option<f32> {
    let particle_pos = transform.translation;
    let to_particle = particle_pos - ray.origin;
    let projection = to_particle.dot(ray_dir);
    
    // Only check particles in front of camera
    if projection < 0.0 {
        return None;
    }
    
    // Find closest point on ray to particle center
    let closest_point = ray.origin + ray_dir * projection;
    let distance_to_ray = (closest_point - particle_pos).length();
    
    // Check if ray intersects particle sphere
    if distance_to_ray < PARTICLE_RADIUS {
        Some(projection)
    } else {
        None
    }
}

fn toggle_particle_selection(
    entity: Entity,
    particle_query: &mut Query<(Entity, &Transform, &mut MeshMaterial3d<StandardMaterial>), (With<Particle>, Without<Selected>)>,
    selected_query: &mut Query<(Entity, &Transform, &mut MeshMaterial3d<StandardMaterial>), (With<Particle>, With<Selected>)>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    commands: &mut Commands,
    selection_state: &mut ResMut<ParticleSelectionState>,
) {
    if let Ok((_, _, mut material)) = selected_query.get_mut(entity) {
        // Deselect: change to white
        material.0 = materials.add(COLOR_WHITE);
        commands.entity(entity).remove::<Selected>();
        selection_state.selected_particles.remove(&entity);
    } else if let Ok((_, _, mut material)) = particle_query.get_mut(entity) {
        // Select: change to purple
        material.0 = materials.add(COLOR_PURPLE);
        commands.entity(entity).insert(Selected);
        selection_state.selected_particles.insert(entity);
    }
}
