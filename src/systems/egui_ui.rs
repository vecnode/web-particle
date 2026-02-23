// systems/egui_ui.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::{ParticleSelectionState, Motion1State, TrajectoryState, CameraViewChanged, CameraProjectionState};
use crate::constants::{CAMERA_FRONT_POSITION, CAMERA_TOP_POSITION, EGUI_TOP_BAR_HEIGHT, EGUI_SECOND_TOP_BAR_HEIGHT, EGUI_LEFT_PANEL_WIDTH, EGUI_RIGHT_PANEL_WIDTH};

pub fn egui_controls_ui(
    mut contexts: EguiContexts,
    selection_state: Res<ParticleSelectionState>,
    mut motion1_state: ResMut<Motion1State>,
    mut trajectory_state: ResMut<TrajectoryState>,
    mut camera_query: Query<(Entity, &mut Transform, &mut GlobalTransform, &mut Projection), (With<bevy::prelude::Camera3d>, With<bevy::camera_controller::free_camera::FreeCamera>, With<crate::components::RightCamera>)>,
    mut camera_changed: ResMut<CameraViewChanged>,
    mut projection_state: ResMut<CameraProjectionState>,
) {
    if let Ok(ctx) = contexts.ctx_mut() {
        // Top bar
        egui::TopBottomPanel::top("top_bar")
            .resizable(false)
            .default_height(EGUI_TOP_BAR_HEIGHT)
            .frame(egui::Frame::side_top_panel(&ctx.style())
                .corner_radius(0.0) // Squared corners
                .inner_margin(egui::Margin::ZERO) // Remove inner margin
                .outer_margin(egui::Margin::ZERO)) // Remove outer margin (including bottom)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Web Particle System");
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("Selected: {}", selection_state.selected_particles.len()));
                    });
                });
            });
        
        // Get the actual position where the first top bar ends
        // After TopBottomPanel is shown, available_rect() starts below it
        let available_after_top = ctx.available_rect();
        let first_top_bar_end_y = available_after_top.top(); // This is where the first bar actually ends
        
        // Controls panel on the left side
        egui::SidePanel::left("controls_panel")
            .resizable(false)
            .default_width(EGUI_LEFT_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Controls");
                    ui.separator();
                    
                    // Camera controls section
                    ui.label("Camera Controls");
                    if ui.button("Camera Front").clicked() {
                        if let Ok((entity, mut transform, mut global_transform, _)) = camera_query.single_mut() {
                            transform.translation = CAMERA_FRONT_POSITION;
                            transform.look_at(Vec3::ZERO, Vec3::Y);
                            *global_transform = GlobalTransform::from(*transform);
                            camera_changed.needs_reset = true;
                            camera_changed.entity = Some(entity);
                        }
                    }
                    
                    if ui.button("Camera Top").clicked() {
                        if let Ok((entity, mut transform, mut global_transform, _)) = camera_query.single_mut() {
                            transform.translation = CAMERA_TOP_POSITION;
                            transform.look_at(Vec3::ZERO, Vec3::Z);
                            *global_transform = GlobalTransform::from(*transform);
                            camera_changed.needs_reset = true;
                            camera_changed.entity = Some(entity);
                        }
                    }
                    
                    // Camera position display
                    if let Ok((_, transform, _, _)) = camera_query.single() {
                        let pos = transform.translation;
                        ui.separator();
                        ui.label("Camera Position:");
                        ui.label(format!("X: {}", pos.x.round() as i32));
                        ui.label(format!("Y: {}", pos.y.round() as i32));
                        ui.label(format!("Z: {}", pos.z.round() as i32));
                    }
                    
                    ui.separator();
                    
                    // Projection mode toggle and controls
                    if let Ok((_, transform, _, mut projection)) = camera_query.single_mut() {
                        // Update stored FOV if currently in perspective mode
                        if let Projection::Perspective(ref persp) = *projection {
                            projection_state.last_perspective_fov = persp.fov;
                        }
                        
                        let is_perspective = matches!(*projection, Projection::Perspective(_));
                        let projection_label = if is_perspective { "Perspective" } else { "Orthographic" };
                        
                        // Toggle button
                        if ui.button(format!("Switch to {}", if is_perspective { "Orthographic" } else { "Perspective" })).clicked() {
                            if is_perspective {
                                // Switch to orthographic projection
                                if let Projection::Perspective(ref persp) = *projection {
                                    // Calculate camera distance from origin (where particles are)
                                    let camera_pos = transform.translation;
                                    let camera_distance = camera_pos.length();
                                    
                                    // Calculate orthographic scale to match perspective view
                                    // Formula: ortho_scale = camera_distance * tan(fov / 2.0)
                                    // This ensures the visible area matches between perspective and orthographic
                                    // tan(fov/2) gives half the visible height ratio at distance 1
                                    // Multiply by distance to get actual visible half-height
                                    let ortho_scale = camera_distance * (persp.fov / 2.0).tan();
                                    
                                    let mut ortho = OrthographicProjection::default_3d();
                                    ortho.scale = ortho_scale.max(0.1); // Ensure minimum scale to avoid zero/negative
                                    *projection = Projection::Orthographic(ortho);
                                } else {
                                    // Fallback if somehow not perspective
                                    let mut ortho = OrthographicProjection::default_3d();
                                    ortho.scale = 5.0;
                                    *projection = Projection::Orthographic(ortho);
                                }
                            } else {
                                // Switch to perspective projection
                                // Restore the FOV that was stored
                                *projection = Projection::Perspective(PerspectiveProjection {
                                    fov: projection_state.last_perspective_fov,
                                    ..default()
                                });
                            }
                        }
                        
                        ui.label(format!("Current: {}", projection_label));
                        
                        // FOV control for Perspective projection
                        if let Projection::Perspective(ref mut persp) = *projection {
                            ui.separator();
                            ui.label("Field of View (FOV)");
                            
                            // Convert to degrees for user-friendly display
                            let mut fov_degrees = persp.fov.to_degrees();
                            if ui.add(egui::Slider::new(&mut fov_degrees, 30.0..=120.0)
                                .text("FOV (degrees)")
                                .step_by(1.0)).changed() {
                                persp.fov = fov_degrees.to_radians();
                            }
                            
                            // Integer input option (alternative)
                            let mut fov_degrees_int = fov_degrees.round() as i32;
                            if ui.add(egui::DragValue::new(&mut fov_degrees_int)
                                .range(30..=120)
                                .speed(1)
                                .prefix("FOV: ")
                                .suffix("°")).changed() {
                                fov_degrees = fov_degrees_int as f32;
                                persp.fov = fov_degrees.to_radians();
                            }
                        }
                        
                        // Orthographic size control for Orthographic projection
                        if let Projection::Orthographic(ref mut ortho) = *projection {
                            ui.separator();
                            ui.label("Orthographic Size");
                            
                            // Slider for continuous control
                            if ui.add(egui::Slider::new(&mut ortho.scale, 1.0..=20.0)
                                .text("Size")
                                .step_by(0.1)).changed() {
                                // Value updated automatically
                            }
                            
                            // Integer input for discrete control
                            let mut ortho_size_int = ortho.scale.round() as i32;
                            if ui.add(egui::DragValue::new(&mut ortho_size_int)
                                .range(1..=20)
                                .speed(1)
                                .prefix("Size: ")).changed() {
                                ortho.scale = ortho_size_int as f32;
                            }
                        }
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
        
        // Inspector panel on the right side
        egui::SidePanel::right("inspector_panel")
            .resizable(false)
            .default_width(EGUI_RIGHT_PANEL_WIDTH)
            .min_width(EGUI_RIGHT_PANEL_WIDTH)
            .max_width(EGUI_RIGHT_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Inspector");
                });
            });
        
        // Second top bar (starts at x=200, fills to right panel, right under first top bar)
        // SOLUTION: After SidePanels are shown, available_rect() gives the content area (excluding panels)
        // available_rect().left() gives us the ACTUAL position where the left panel ends
        // This is more reliable than using the constant, as it accounts for any panel borders/margins
        let available_rect = ctx.available_rect(); // Content area after panels
        let viewport_rect = ctx.viewport_rect(); // Full window size
        
        // Get the actual left panel end position from available_rect
        // This ensures we position correctly even if panel has borders or content affects layout
        let left_panel_end_x = available_rect.left(); // Actual position where left panel ends
        
        // Calculate exact width: from left panel end to right panel start
        // available_rect.right() gives us where the right panel starts
        let right_panel_start_x = available_rect.right();
        let second_bar_width = (right_panel_start_x - left_panel_end_x).max(0.0);
        let second_bar_height = EGUI_SECOND_TOP_BAR_HEIGHT; // Match first top bar height
        
        // Position the second bar exactly where the first bar ends (no gap)
        // Use the actual panel positions from available_rect for accurate positioning
        let second_bar_rect = egui::Rect::from_min_size(
            egui::pos2(left_panel_end_x, first_top_bar_end_y),
            egui::vec2(second_bar_width, second_bar_height)
        );
        
        egui::Area::new(egui::Id::new("second_top_bar"))
            .fixed_pos(second_bar_rect.min)
            .constrain(true)
            .show(ctx, |ui| {
                // Allocate rect to intercept clicks and block 3D world input
                let _response = ui.allocate_rect(second_bar_rect, egui::Sense::click());
                
                // Paint the background directly to match panel fill (exact size, no Frame expansion)
                ui.painter().rect_filled(second_bar_rect, 0.0, ui.style().visuals.panel_fill);
                
                // Set clip rect to hard-constrain content to exactly 30px height (prevents overflow)
                ui.set_clip_rect(second_bar_rect);
                
                // Allocate UI at the exact rect position to constrain content within the bar
                #[allow(deprecated)]
                ui.allocate_ui_at_rect(second_bar_rect, |ui| {
                    // Constrain height to exactly 30px to match first top bar
                    ui.set_max_height(second_bar_height);
                    ui.set_min_height(second_bar_height);
                    
                    // Remove ALL margins, padding, and spacing inside the bar
                    ui.spacing_mut().button_padding = egui::vec2(4.0, 0.0); // Zero vertical padding, minimal horizontal
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0); // No spacing at all
                    ui.spacing_mut().window_margin = egui::Margin::ZERO; // No window margin
                    ui.spacing_mut().menu_margin = egui::Margin::ZERO; // No menu margin
                    
                    // Use the same layout approach as the first top bar - no margins
                    ui.horizontal(|ui| {
                        // Add 5px left margin for the button
                        ui.add_space(5.0);
                        // Button with normal frame to make it visible (not frame(false))
                        if ui.button("File").clicked() {
                            // Debug: Print sizes to terminal
                            println!("=== Second Top Bar Debug ===");
                            println!("EGUI_TOP_BAR_HEIGHT: {}px", EGUI_TOP_BAR_HEIGHT);
                            println!("second_bar_height: {}px", second_bar_height);
                            println!("second_bar_rect size: {}x{}", second_bar_rect.width(), second_bar_rect.height());
                            println!("second_bar_rect position: ({}, {})", second_bar_rect.min.x, second_bar_rect.min.y);
                            println!("first_top_bar_end_y: {}", first_top_bar_end_y);
                            println!("viewport_rect size: {}x{}", viewport_rect.width(), viewport_rect.height());
                            println!("===========================");
                        }
                    });
                });
            });
    }
}
