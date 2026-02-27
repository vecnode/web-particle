// systems/egui_ui.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_plot::{Plot, PlotPoints, Line};
use crate::components::{ParticleSelectionState, Motion1State, TrajectoryState, CameraViewChanged, CameraProjectionState, EguiLayoutState, GridState, ParticleBoundsState, ParticleGroupState, StreamsPanelState, ParticleCreationState, ParticlePlacementMode, InMotion, SelectionTransformState};
use crate::constants::{CAMERA_FRONT_POSITION, CAMERA_TOP_POSITION, EGUI_TOP_BAR_HEIGHT, EGUI_SECOND_TOP_BAR_HEIGHT, EGUI_LEFT_PANEL_WIDTH};

pub fn egui_controls_ui(
    mut contexts: EguiContexts,
    selection_state: Res<ParticleSelectionState>,
    mut motion1_state: ResMut<Motion1State>,
    mut trajectory_state: ResMut<TrajectoryState>,
    mut camera_changed: ResMut<CameraViewChanged>,
    mut projection_state: ResMut<CameraProjectionState>,
    mut layout_state: ResMut<EguiLayoutState>,
    mut grid_state: ResMut<GridState>,
    mut _particle_bounds_state: ResMut<ParticleBoundsState>,
    mut _particle_group_state: ResMut<ParticleGroupState>,
    mut streams_panel_state: ResMut<StreamsPanelState>,
    mut creation_state: ResMut<ParticleCreationState>,
    mut selection_transform_state: ResMut<SelectionTransformState>,
    mut commands: Commands,
    mut queries: ParamSet<(
        Query<(Entity, &mut Transform, &mut GlobalTransform, &mut Projection), (With<bevy::prelude::Camera3d>, With<crate::plugins::viewport_constrained_camera::ViewportConstrainedCamera>, With<crate::components::RightCamera>)>,
        Query<&Transform, With<crate::components::Particle>>,
        Query<Entity, (With<crate::components::Particle>, With<InMotion>)>,
        Query<Entity, With<crate::components::Particle>>,
    )>,
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
                    ui.label("Web-Particle System");
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
                // Measure actual content area width
                let left_panel_content_width = ui.available_width();
                layout_state.left_panel_content_width = left_panel_content_width;
                
                // Add scroll area for vertical overflow
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Controls");
                            ui.separator();
                    
                    // Camera controls section
                    ui.label("Camera Controls");
                    
                    // Camera position display (one line)
                    if let Ok((_, transform, _, _)) = queries.p0().single() {
                        let pos = transform.translation;
                        ui.label(format!("Camera Position: ({}, {}, {})", 
                            pos.x.round() as i32, 
                            pos.y.round() as i32, 
                            pos.z.round() as i32));
                    }
                    
                    if ui.button("Camera Front").clicked() {
                        if let Ok((entity, mut transform, mut global_transform, _)) = queries.p0().single_mut() {
                            transform.translation = CAMERA_FRONT_POSITION;
                            transform.look_at(Vec3::ZERO, Vec3::Y);
                            *global_transform = GlobalTransform::from(*transform);
                            camera_changed.needs_reset = true;
                            camera_changed.entity = Some(entity);
                        }
                    }
                    
                    if ui.button("Camera Top").clicked() {
                        if let Ok((entity, mut transform, mut global_transform, _)) = queries.p0().single_mut() {
                            transform.translation = CAMERA_TOP_POSITION;
                            transform.look_at(Vec3::ZERO, Vec3::Z);
                            *global_transform = GlobalTransform::from(*transform);
                            camera_changed.needs_reset = true;
                            camera_changed.entity = Some(entity);
                        }
                    }

                    // Display projection mode label
                    ui.label("Perspective Camera");
                    
                    ui.separator();
                    
                    // Camera projection info and controls
                    if let Ok((_, _, _, mut projection)) = queries.p0().single_mut() {
                        // Update stored FOV if currently in perspective mode
                        if let Projection::Perspective(ref persp) = *projection {
                            projection_state.last_perspective_fov = persp.fov;
                        }
                        
                        
                        // FOV control for Perspective projection
                        if let Projection::Perspective(ref mut persp) = *projection {
                            ui.label("Field of View (FOV)");
                            
                            // Convert to degrees for user-friendly display
                            let mut fov_degrees = persp.fov.to_degrees();
                            if ui.add(egui::Slider::new(&mut fov_degrees, 30.0..=120.0)
                                .text("FOV (degrees)")
                                .step_by(1.0)).changed() {
                                persp.fov = fov_degrees.to_radians();
                            }
                            
                        }
                    }
                    
                    ui.separator();
                    ui.label(format!("Particles Selected: {}", selection_state.selected_particles.len()));

                    // Particle Creation section
                    ui.separator();
                    ui.label("Particle Creation");
                    
                    // Placement mode selection
                    ui.horizontal(|ui| {
                        ui.label("Mode:");
                        ui.radio_value(&mut creation_state.placement_mode, ParticlePlacementMode::Random, "Random");
                        ui.radio_value(&mut creation_state.placement_mode, ParticlePlacementMode::Ball, "Ball");
                        ui.radio_value(&mut creation_state.placement_mode, ParticlePlacementMode::Cube, "Cube");
                    });
                    
                    // Batch count
                    ui.horizontal(|ui| {
                        ui.label("Count:");
                        if ui.add(egui::DragValue::new(&mut creation_state.batch_count)
                            .range(1..=100)
                            .speed(1)).changed() {
                            // Value updated
                        }
                    });
                    
                    // Create button
                    if ui.button("Create Particles").clicked() {
                        creation_state.create_requested = true;
                    }
                    
                    // Remove buttons
                    ui.horizontal(|ui| {
                        let has_selected = !selection_state.selected_particles.is_empty();
                        if ui.add_enabled(has_selected, egui::Button::new("Remove Selected")).clicked() {
                            creation_state.remove_selected_requested = true;
                        }
                        if ui.button("Remove All").clicked() {
                            creation_state.remove_all_requested = true;
                        }
                    });
                    
                    // Ball mode parameters
                    if creation_state.placement_mode == ParticlePlacementMode::Ball {
                        ui.separator();
                        ui.label("Ball Parameters");
                        
                        ui.horizontal(|ui| {
                            ui.label("Center X:");
                            if ui.add(egui::DragValue::new(&mut creation_state.ball_center.x)
                                .range(-50.0..=50.0)
                                .speed(0.1)).changed() {}
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Center Y:");
                            if ui.add(egui::DragValue::new(&mut creation_state.ball_center.y)
                                .range(0.0..=20.0)
                                .speed(0.1)).changed() {}
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Center Z:");
                            if ui.add(egui::DragValue::new(&mut creation_state.ball_center.z)
                                .range(-50.0..=50.0)
                                .speed(0.1)).changed() {}
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Radius:");
                            if ui.add(egui::DragValue::new(&mut creation_state.ball_radius)
                                .range(0.1..=10.0)
                                .speed(0.1)
                                .suffix(" m")).changed() {}
                        });
                    }
                    
                    // Cube mode parameters
                    if creation_state.placement_mode == ParticlePlacementMode::Cube {
                        ui.separator();
                        ui.label("Cube Parameters");
                        
                        ui.horizontal(|ui| {
                            ui.label("Center X:");
                            if ui.add(egui::DragValue::new(&mut creation_state.cube_center.x)
                                .range(-50.0..=50.0)
                                .speed(0.1)).changed() {}
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Center Y:");
                            if ui.add(egui::DragValue::new(&mut creation_state.cube_center.y)
                                .range(0.0..=20.0)
                                .speed(0.1)).changed() {}
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Center Z:");
                            if ui.add(egui::DragValue::new(&mut creation_state.cube_center.z)
                                .range(-50.0..=50.0)
                                .speed(0.1)).changed() {}
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Size X:");
                            if ui.add(egui::DragValue::new(&mut creation_state.cube_size.x)
                                .range(0.1..=20.0)
                                .speed(0.1)
                                .suffix(" m")).changed() {}
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Size Y:");
                            if ui.add(egui::DragValue::new(&mut creation_state.cube_size.y)
                                .range(0.1..=20.0)
                                .speed(0.1)
                                .suffix(" m")).changed() {}
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Size Z:");
                            if ui.add(egui::DragValue::new(&mut creation_state.cube_size.z)
                                .range(0.1..=20.0)
                                .speed(0.1)
                                .suffix(" m")).changed() {}
                        });
                    }

                    // Grid controls section
                    ui.label("Grid Size (meters)");
                    
                    // X dimension input
                    let mut size_x = grid_state.size_x;
                    if ui.add(egui::DragValue::new(&mut size_x)
                        .range(1..=100)
                        .speed(1)
                        .prefix("X: ")
                        .suffix(" m")).changed() {
                        grid_state.size_x = size_x;
                    }
                    
                    // Z dimension input
                    let mut size_z = grid_state.size_z;
                    if ui.add(egui::DragValue::new(&mut size_z)
                        .range(1..=100)
                        .speed(1)
                        .prefix("Z: ")
                        .suffix(" m")).changed() {
                        grid_state.size_z = size_z;
                    }
                    
                    
                    
                    // Selection position offset controls section
                    ui.label("Selection Distribution Area (meters)");
                    
                    // Position offset X
                    let mut offset_x = selection_transform_state.position_offset.x;
                    if ui.add(egui::DragValue::new(&mut offset_x)
                        .range(-100.0..=100.0)
                        .speed(0.5)
                        .prefix("X: ")
                        .suffix(" m")).changed() {
                        selection_transform_state.position_offset.x = offset_x;
                    }
                    
                    // Position offset Y
                    let mut offset_y = selection_transform_state.position_offset.y;
                    if ui.add(egui::DragValue::new(&mut offset_y)
                        .range(-100.0..=100.0)
                        .speed(0.5)
                        .prefix("Y: ")
                        .suffix(" m")).changed() {
                        selection_transform_state.position_offset.y = offset_y;
                    }
                    
                    // Position offset Z
                    let mut offset_z = selection_transform_state.position_offset.z;
                    if ui.add(egui::DragValue::new(&mut offset_z)
                        .range(-100.0..=100.0)
                        .speed(0.5)
                        .prefix("Z: ")
                        .suffix(" m")).changed() {
                        selection_transform_state.position_offset.z = offset_z;
                    }
                    
                    
                    // Selection scale controls section
                    ui.label("Selection Transform");
                    
                    // Scale X (normal distribution)
                    let mut scale_x = selection_transform_state.scale.x;
                    if ui.add(egui::Slider::new(&mut scale_x, 0.1..=5.0)
                        .text("Scale X")
                        .step_by(0.1)).changed() {
                        selection_transform_state.scale.x = scale_x;
                    }
                    
                    // Scale Y (normal distribution)
                    let mut scale_y = selection_transform_state.scale.y;
                    if ui.add(egui::Slider::new(&mut scale_y, 0.1..=5.0)
                        .text("Scale Y")
                        .step_by(0.1)).changed() {
                        selection_transform_state.scale.y = scale_y;
                    }
                    
                    // Scale Z (normal distribution)
                    let mut scale_z = selection_transform_state.scale.z;
                    if ui.add(egui::Slider::new(&mut scale_z, 0.1..=5.0)
                        .text("Scale Z")
                        .step_by(0.1)).changed() {
                        selection_transform_state.scale.z = scale_z;
                    }
                    
                   
                    ui.separator();
                    
                    // Motion 1 button
                    // Check if any particles are in motion to determine button label
                    let particles_with_motion: std::collections::HashSet<Entity> = queries.p2().iter().collect();
                    let particles_in_motion = particles_with_motion.len();
                    let motion1_label = if particles_in_motion > 0 { 
                        format!("Motion 1 (Active: {})", particles_in_motion) 
                    } else { 
                        "Motion 1".to_string() 
                    };
                    if ui.button(motion1_label).clicked() {
                        // If particles are selected, toggle motion only for selected particles
                        // If no particles are selected, toggle motion for all particles
                        if !selection_state.selected_particles.is_empty() {
                            // Toggle InMotion component for selected particles only
                            for entity in selection_state.selected_particles.iter() {
                                if particles_with_motion.contains(entity) {
                                    // Remove InMotion component (stop motion)
                                    commands.entity(*entity).remove::<InMotion>();
                                } else {
                                    // Add InMotion component (start motion)
                                    commands.entity(*entity).insert(InMotion);
                                }
                            }
                            // Update is_active: true if any particles (selected or not) will be in motion after toggle
                            let selected_in_motion_before = selection_state.selected_particles.iter()
                                .filter(|e| particles_with_motion.contains(e))
                                .count();
                            let non_selected_in_motion = particles_with_motion.len() - selected_in_motion_before;
                            let selected_in_motion_after = selection_state.selected_particles.len() - selected_in_motion_before;
                            motion1_state.is_active = (non_selected_in_motion + selected_in_motion_after) > 0;
                        } else {
                            // No selection: toggle motion for all particles
                            let all_particles: Vec<Entity> = queries.p3().iter().collect();
                            
                            if particles_with_motion.len() == all_particles.len() {
                                // All particles are in motion: stop all
                                for entity in all_particles {
                                    commands.entity(entity).remove::<InMotion>();
                                }
                                motion1_state.is_active = false;
                            } else {
                                // Not all particles are in motion: start all
                                for entity in all_particles {
                                    if !particles_with_motion.contains(&entity) {
                                        commands.entity(entity).insert(InMotion);
                                    }
                                }
                                motion1_state.is_active = true;
                            }
                        }
                    }
                    
                    // Show Trajectory button
                    let trajectory_label = if trajectory_state.is_visible { "Hide Trajectory" } else { "Show Trajectory" };
                    if ui.button(trajectory_label).clicked() {
                        trajectory_state.is_visible = !trajectory_state.is_visible;
                    }
                    
                        }); // Close vertical layout
                    }); // Close ScrollArea
            }); // Close SidePanel
        
        // Second top bar (starts at x=200, fills to right panel, right under first top bar)
        // SOLUTION: After SidePanels are shown, available_rect() gives the content area (excluding panels)
        // available_rect().left() gives us the ACTUAL position where the left panel ends (includes frame borders)
        // For the right panel, we mirror the left panel's total width (including borders) for symmetry
        let available_rect = ctx.available_rect(); // Content area after panels
        let viewport_rect = ctx.viewport_rect();
        
        // Get the actual left panel end position (includes frame borders, ~38px extra)
        let left_panel_end_x = available_rect.left(); // Actual position where left panel ends
        
        // Calculate right panel start position: mirror the left panel's total width
        // If left panel ends at 238.03 (200px content + 38.03px borders), 
        // right panel should start at viewport_right - 238.03 for symmetry
        let left_panel_total_width = left_panel_end_x; // Total width from 0 to left panel end
        let calculated_right_panel_start = viewport_rect.right() - left_panel_total_width;
        
        // Store actual panel positions for camera viewport calculation
        layout_state.left_panel_end_x = left_panel_end_x; // Actual position where left panel ends (includes frame borders)
        layout_state.right_panel_start_x = calculated_right_panel_start; // Right panel starts here (mirrors left panel width)
        layout_state.top_bars_height = EGUI_TOP_BAR_HEIGHT + EGUI_SECOND_TOP_BAR_HEIGHT;
        layout_state.bottom_bar_height = EGUI_SECOND_TOP_BAR_HEIGHT; // Bottom bar height
        
        // Calculate exact width: from left panel end to right edge of window (for testing)
        // Extended to the right side of the window, not stopping at inspector panel
        let second_bar_width = (viewport_rect.right() - left_panel_end_x).max(0.0);
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
                    ui.spacing_mut().button_padding = egui::vec2(4.0, 0.0); // Reduced vertical padding to make button 4px smaller
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0); // No spacing at all
                    ui.spacing_mut().window_margin = egui::Margin::ZERO; // No window margin
                    ui.spacing_mut().menu_margin = egui::Margin::ZERO; // No menu margin
                    
                    // Add 2px vertical offset to push button down
                    ui.add_space(2.0);
                    // Use the same layout approach as the first top bar - no margins
                    ui.horizontal(|ui| {
                        // Add 5px left margin for the button
                        ui.add_space(5.0);
                        // Button with normal frame to make it visible (not frame(false))
                        if ui.button("Workspace").clicked() {
                            streams_panel_state.is_visible = false;
                        }
                        // Add spacing between buttons
                        ui.add_space(5.0);
                        // Streams button with same style
                        if ui.button("Streams").clicked() {
                            streams_panel_state.is_visible = true;
                        }
                    });
                });
            });
        
        // Bottom bar - positioned at the bottom, between the two sidebars, under the 3D world
        let viewport_rect_for_bottom = ctx.viewport_rect();
        let bottom_bar_height = EGUI_SECOND_TOP_BAR_HEIGHT; // Same height as second top bar
        let bottom_bar_y = viewport_rect_for_bottom.bottom() - bottom_bar_height;
        
        let bottom_bar_rect = egui::Rect::from_min_size(
            egui::pos2(left_panel_end_x, bottom_bar_y),
            egui::vec2(second_bar_width, bottom_bar_height)
        );
        
        egui::Area::new(egui::Id::new("bottom_bar"))
            .fixed_pos(bottom_bar_rect.min)
            .constrain(true)
            .show(ctx, |ui| {
                // Allocate rect to intercept clicks and block 3D world input
                let _response = ui.allocate_rect(bottom_bar_rect, egui::Sense::click());
                
                // Paint the background directly to match panel fill (exact size, no Frame expansion)
                ui.painter().rect_filled(bottom_bar_rect, 0.0, ui.style().visuals.panel_fill);
                
                // Set clip rect to hard-constrain content to exactly the bar height
                ui.set_clip_rect(bottom_bar_rect);
                
                // Allocate UI at the exact rect position to constrain content within the bar
                #[allow(deprecated)]
                ui.allocate_ui_at_rect(bottom_bar_rect, |ui| {
                    // Constrain height to exactly match the bar height
                    ui.set_max_height(bottom_bar_height);
                    ui.set_min_height(bottom_bar_height);
                    
                    // Remove ALL margins, padding, and spacing inside the bar
                    ui.spacing_mut().button_padding = egui::vec2(4.0, 0.0); // Reduced vertical padding to make button 4px smaller
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0); // No spacing at all
                    ui.spacing_mut().window_margin = egui::Margin::ZERO; // No window margin
                    ui.spacing_mut().menu_margin = egui::Margin::ZERO; // No menu margin
                    
                    // Add Inspector toggle button - positioned like "3D Viewer" button but a couple pixels down
                    ui.add_space(2.0); // Add 2px vertical offset (couple pixels down)
                    // Use the same layout approach as the second top bar - no margins
                    ui.horizontal(|ui| {
                    // Add 5px left margin for the button
                    // Add spacing between buttons
                    ui.add_space(5.0);
                    // 3D Viewer toggle button
                    if ui.button("3D Viewer").clicked() {
                        layout_state.d3_viewer_visible = !layout_state.d3_viewer_visible;
                    }
                    ui.add_space(5.0);
                    // Left Panel toggle button
                    if ui.button("Middle-Left Panel").clicked() {
                        layout_state.left_half_panel_collapsed = !layout_state.left_half_panel_collapsed;
                    }
                        ui.add_space(5.0);
                        // Button with normal frame to make it visible (not frame(false))
                        if ui.button("Inspector").clicked() {
                            layout_state.inspector_collapsed = !layout_state.inspector_collapsed;
                        }
                        
                    });
                });
            });
        
        // Inspector panel on the right side - rendered AFTER bars as Area to appear on top
        // Set width to match left panel's total width (including borders) for symmetry
        // Only show if not collapsed (toggled by button in bottom bar)
        if !layout_state.inspector_collapsed {
            let viewport_rect = ctx.viewport_rect();
            let inspector_width = left_panel_total_width;
            let inspector_x = viewport_rect.right() - inspector_width;
            let inspector_y = 22.0; // Start 22px from top (below top bars)
            let inspector_height = viewport_rect.height() - inspector_y;
            
            let inspector_rect = egui::Rect::from_min_size(
                egui::pos2(inspector_x, inspector_y),
                egui::vec2(inspector_width, inspector_height)
            );
            
            egui::Area::new(egui::Id::new("inspector_panel"))
                .fixed_pos(inspector_rect.min)
                .constrain(true)
                .order(egui::Order::Foreground) // Ensure it renders on top
                .show(ctx, |ui| {
                    // Allocate rect to intercept clicks
                    let _response = ui.allocate_rect(inspector_rect, egui::Sense::click());
                    
                    // Paint the background
                    ui.painter().rect_filled(inspector_rect, 0.0, ui.style().visuals.panel_fill);
                    
                    // Draw left border to match the left panel's border
                    let border_stroke = ui.style().visuals.widgets.noninteractive.bg_stroke;
                    let left_edge_start = egui::pos2(inspector_rect.left(), inspector_rect.top());
                    let left_edge_end = egui::pos2(inspector_rect.left(), inspector_rect.bottom());
                    ui.painter().line_segment([left_edge_start, left_edge_end], border_stroke);
                    
                    // Set clip rect to constrain content
                    ui.set_clip_rect(inspector_rect);
                    
                    // Allocate UI at the exact rect position
                    #[allow(deprecated)]
                    ui.allocate_ui_at_rect(inspector_rect, |ui| {
                        // Measure actual content area width (accounting for frame)
                        let right_panel_content_width = ui.available_width();
                        layout_state.right_panel_content_width = right_panel_content_width;
                        
                        // Add left padding to match SidePanel's default padding
                        //ui.add_space(8.0); // Small left padding similar to SidePanel
                        
                        ui.vertical(|ui| {
                            ui.heading("Inspector");
                            ui.separator();
                        });
                    });
                });
        }
        
        // Left half panel - divides the 3D world space vertically (left 50%, full height)
        // When 3D viewer is hidden, takes full width instead of 50%
        // Only show if not collapsed (toggled by button in bottom bar)
        if !layout_state.left_half_panel_collapsed {
            let viewport_rect = ctx.viewport_rect();
            let left_panel_end_x = layout_state.left_panel_end_x;
            let viewport_right_edge = if layout_state.inspector_collapsed {
                viewport_rect.right() // Extend to right edge when inspector is hidden
            } else {
                layout_state.right_panel_start_x // Stop at inspector when visible
            };
            
            // Calculate total available width
            let total_viewport_width = viewport_right_edge - left_panel_end_x;
            // If 3D viewer is hidden, panel takes full width; otherwise takes 50%
            let panel_width = if !layout_state.d3_viewer_visible {
                total_viewport_width // Full width when 3D viewer is hidden
            } else {
                total_viewport_width / 2.0 // Half width when 3D viewer is visible
            };
            let panel_y = layout_state.top_bars_height; // Start below top bars
            let panel_height = viewport_rect.height() - layout_state.top_bars_height - layout_state.bottom_bar_height; // Full height minus bars
            
            let left_half_panel_rect = egui::Rect::from_min_size(
                egui::pos2(left_panel_end_x, panel_y),
                egui::vec2(panel_width, panel_height)
            );
            
            egui::Area::new(egui::Id::new("left_half_panel"))
                .fixed_pos(left_half_panel_rect.min)
                .constrain(true)
                .show(ctx, |ui| {
                    // Allocate rect to intercept clicks
                    let _response = ui.allocate_rect(left_half_panel_rect, egui::Sense::click());
                    
                    // Paint the background
                    ui.painter().rect_filled(left_half_panel_rect, 0.0, ui.style().visuals.panel_fill);
                    
                    // Set clip rect to constrain content
                    ui.set_clip_rect(left_half_panel_rect);
                    
                    // Allocate UI at the exact rect position
                    #[allow(deprecated)]
                    ui.allocate_ui_at_rect(left_half_panel_rect, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Middle-Left Panel");
                            ui.separator();
                            
                            // Center Axis button
                            if ui.button("Center Axis").clicked() {
                                layout_state.plot_center_axes = !layout_state.plot_center_axes;
                            }
                            
                            // Calculate available height for the plot (reserve space for heading, separator, and button)
                            let plot_height = ui.available_height().max(200.0); // Minimum 200px height
                            
                            // Create a simple example plot (sine wave)
                            let points: PlotPoints = (0..100)
                                .map(|i| {
                                    let x = i as f64 * 0.1;
                                    [x, x.sin()]
                                })
                                .collect();
                            
                            // Calculate grid bounds for axis centering
                            // Use full grid size: if grid_size = 10, show from -10 to +10 (centered at 0)
                            let grid_size_x = grid_state.size_x as f64;
                            let grid_size_z = grid_state.size_z as f64;
                            
                            // Build plot with conditional axis bounds
                            let mut plot = Plot::new("middle_left_plot").height(plot_height);
                            
                            // If center axes is enabled, set axis bounds to match grid dimensions
                            // Grid is mirrored (symmetric around 0,0), so axes should show -grid_size to +grid_size
                            // This centers 0 in the middle and shows equal positive and negative ranges
                            if layout_state.plot_center_axes {
                                // Include both negative and positive bounds to center at 0,0
                                // For grid_size = 10, this shows from -10 to +10
                                plot = plot
                                    .include_x(-grid_size_x)  // Negative X bound: -10 for size 10
                                    .include_x(grid_size_x)   // Positive X bound: +10 for size 10
                                    .include_y(-grid_size_z) // Negative Y bound: -10 for size 10
                                    .include_y(grid_size_z);  // Positive Y bound: +10 for size 10
                                
                                // Also include the origin (0,0) to ensure it's visible and centered
                                plot = plot.include_x(0.0).include_y(0.0);
                            }
                            
                            plot.show(ui, |plot_ui| {
                                plot_ui.line(Line::new("Sine Wave", points));
                            });
                        });
                    });
                });
        }
        
        // Streams panel - covers the 3D viewport when visible
        if streams_panel_state.is_visible {
            let viewport_rect = ctx.viewport_rect();
            let viewport_x = layout_state.left_panel_end_x;
            let viewport_y = layout_state.top_bars_height;
            // Adjust width based on inspector visibility: extend to right edge if inspector is hidden
            let viewport_right_edge = if layout_state.inspector_collapsed {
                viewport_rect.right() // Extend to right edge of window when inspector is hidden
            } else {
                layout_state.right_panel_start_x // Stop at inspector when visible
            };
            let viewport_width = viewport_right_edge - layout_state.left_panel_end_x;
            let viewport_height = viewport_rect.height() - layout_state.top_bars_height;
            
            let streams_panel_rect = egui::Rect::from_min_size(
                egui::pos2(viewport_x, viewport_y),
                egui::vec2(viewport_width.max(0.0), viewport_height.max(0.0))
            );
            
            egui::Area::new(egui::Id::new("streams_panel"))
                .fixed_pos(streams_panel_rect.min)
                .constrain(true)
                .interactable(true)
                .order(egui::Order::Foreground) // Render on top instantly, no transitions
                .show(ctx, |ui| {
                    // Allocate rect to intercept clicks and block 3D world input
                    let _response = ui.allocate_rect(streams_panel_rect, egui::Sense::click());
                    
                    // Paint background to fully cover the 3D viewport - use fixed color for instant appearance
                    let panel_color = ui.style().visuals.panel_fill;
                    ui.painter().rect_filled(streams_panel_rect, 0.0, panel_color);
                    
                    // Set clip rect to constrain content
                    ui.set_clip_rect(streams_panel_rect);
                    
                    // Allocate UI at the exact rect position
                    #[allow(deprecated)]
                    ui.allocate_ui_at_rect(streams_panel_rect, |ui| {
                        ui.vertical(|ui| {
                            ui.heading("Streams Panel");
                            ui.separator();
                            ui.label("This panel covers the 3D viewport.");
                            ui.label("Click '3D Viewer' to return to the 3D world.");
                        });
                    });
                });
        }
    }
}
