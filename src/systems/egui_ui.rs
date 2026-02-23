// systems/egui_ui.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::{ParticleSelectionState, Motion1State, TrajectoryState, CameraViewChanged};
use crate::constants::{CAMERA_FRONT_POSITION, CAMERA_TOP_POSITION};

pub fn egui_controls_ui(
    mut contexts: EguiContexts,
    selection_state: Res<ParticleSelectionState>,
    mut motion1_state: ResMut<Motion1State>,
    mut trajectory_state: ResMut<TrajectoryState>,
    mut camera_query: Query<(Entity, &mut Transform, &mut GlobalTransform), (With<bevy::prelude::Camera3d>, With<bevy::camera_controller::free_camera::FreeCamera>, With<crate::components::RightCamera>)>,
    mut camera_changed: ResMut<CameraViewChanged>,
) {
    if let Ok(ctx) = contexts.ctx_mut() {
        // Top bar
        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(30.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Web Particle System");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("Selected: {}", selection_state.selected_particles.len()));
                    });
                });
            });
        
        // Controls panel on the left side
        egui::SidePanel::left("controls_panel")
            .resizable(false)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Controls");
                    ui.separator();
                    
                    // Camera controls section
                    ui.label("Camera Controls");
                    if ui.button("Camera Front").clicked() {
                        if let Ok((entity, mut transform, mut global_transform)) = camera_query.single_mut() {
                            transform.translation = CAMERA_FRONT_POSITION;
                            transform.look_at(Vec3::ZERO, Vec3::Y);
                            *global_transform = GlobalTransform::from(*transform);
                            camera_changed.needs_reset = true;
                            camera_changed.entity = Some(entity);
                        }
                    }
                    
                    if ui.button("Camera Top").clicked() {
                        if let Ok((entity, mut transform, mut global_transform)) = camera_query.single_mut() {
                            transform.translation = CAMERA_TOP_POSITION;
                            transform.look_at(Vec3::ZERO, Vec3::Z);
                            *global_transform = GlobalTransform::from(*transform);
                            camera_changed.needs_reset = true;
                            camera_changed.entity = Some(entity);
                        }
                    }
                    
                    // Camera position display
                    if let Ok((_, transform, _)) = camera_query.single() {
                        let pos = transform.translation;
                        ui.separator();
                        ui.label("Camera Position:");
                        ui.label(format!("X: {}", pos.x.round() as i32));
                        ui.label(format!("Y: {}", pos.y.round() as i32));
                        ui.label(format!("Z: {}", pos.z.round() as i32));
                    }
                    
                    ui.separator();
                    
                    // Particle controls section
                    ui.heading("Particle Controls");
                    ui.label(format!("Selected: {}", selection_state.selected_particles.len()));
                    
                    ui.separator();
                    
                    // Motion 1 button
                    let motion1_label = if motion1_state.is_active { "Motion 1 (Active)" } else { "Motion 1" };
                    if ui.button(motion1_label).clicked() {
                        motion1_state.is_active = !motion1_state.is_active;
                    }
                    
                    // Show Trajectory button
                    let trajectory_label = if trajectory_state.is_visible { "Hide Trajectory" } else { "Show Trajectory" };
                    if ui.button(trajectory_label).clicked() {
                        trajectory_state.is_visible = !trajectory_state.is_visible;
                    }
                    
                    ui.separator();
                    ui.label("Instructions");
                    ui.label("Left click: Select particle");
                    ui.label("Right drag: Select area");
                    ui.label("Mouse drag: Rotate camera");
                    ui.label("Scroll: Zoom");
                });
            });
    }
}
