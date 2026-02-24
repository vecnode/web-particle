// systems/egui_ui.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::{ParticleSelectionState, Motion1State, TrajectoryState, CameraViewChanged, CameraProjectionState, EguiLayoutState, GridState, ParticleBoundsState, ParticleGroupState, StreamsPanelState};
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
    mut particle_bounds_state: ResMut<ParticleBoundsState>,
    mut particle_group_state: ResMut<ParticleGroupState>,
    mut streams_panel_state: ResMut<StreamsPanelState>,
    mut queries: ParamSet<(
        Query<(Entity, &mut Transform, &mut GlobalTransform, &mut Projection), (With<bevy::prelude::Camera3d>, With<crate::plugins::viewport_constrained_camera::ViewportConstrainedCamera>, With<crate::components::RightCamera>)>,
        Query<&Transform, With<crate::components::Particle>>,
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
                    
                    
                    
                    // Particle bounds controls section
                    ui.label("Particle Distribution Area (meters)");
                    
                    // X bounds input
                    let mut bounds_x = particle_bounds_state.bounds_x;
                    if ui.add(egui::DragValue::new(&mut bounds_x)
                        .range(1.0..=100.0)
                        .speed(0.5)
                        .prefix("X: ")
                        .suffix(" m")).changed() {
                        particle_bounds_state.bounds_x = bounds_x.max(0.1);
                    }
                    
                    // Z bounds input
                    let mut bounds_z = particle_bounds_state.bounds_z;
                    if ui.add(egui::DragValue::new(&mut bounds_z)
                        .range(1.0..=100.0)
                        .speed(0.5)
                        .prefix("Z: ")
                        .suffix(" m")).changed() {
                        particle_bounds_state.bounds_z = bounds_z.max(0.1);
                    }
                    
                    // Y Height input (starts at 1.0, extends upward)
                    let mut bounds_y_height = particle_bounds_state.bounds_y_height;
                    if ui.add(egui::DragValue::new(&mut bounds_y_height)
                        .range(0.1..=50.0)
                        .speed(0.1)
                        .prefix("Y: ")
                        .suffix(" m")).changed() {
                        particle_bounds_state.bounds_y_height = bounds_y_height.max(0.1);
                    }
                    
                    
                    // Particle group controls section
                    ui.label("Particle Transform All");
                    
                    // Group offset inputs
                    let mut offset_x = particle_group_state.offset.x;
                    if ui.add(egui::DragValue::new(&mut offset_x)
                        .range(-100.0..=100.0)
                        .speed(0.5)
                        .prefix("Offset X: ")
                        .suffix(" m")).changed() {
                        particle_group_state.offset.x = offset_x;
                    }
                    
                    let mut offset_y = particle_group_state.offset.y;
                    if ui.add(egui::DragValue::new(&mut offset_y)
                        .range(-100.0..=100.0)
                        .speed(0.5)
                        .prefix("Offset Y: ")
                        .suffix(" m")).changed() {
                        particle_group_state.offset.y = offset_y;
                    }
                    
                    let mut offset_z = particle_group_state.offset.z;
                    if ui.add(egui::DragValue::new(&mut offset_z)
                        .range(-100.0..=100.0)
                        .speed(0.5)
                        .prefix("Offset Z: ")
                        .suffix(" m")).changed() {
                        particle_group_state.offset.z = offset_z;
                    }
                    
                    // Group scale input
                    let mut scale = particle_group_state.scale;
                    if ui.add(egui::Slider::new(&mut scale, 0.1..=5.0)
                        .text("Scale")
                        .step_by(0.1)).changed() {
                        particle_group_state.scale = scale;
                    }
                    
                   
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
                    
                });
            });
        
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
                        if ui.button("3D Viewer").clicked() {
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
                        ui.add_space(5.0);
                        // Button with normal frame to make it visible (not frame(false))
                        if ui.button("Inspector").clicked() {
                            layout_state.inspector_collapsed = !layout_state.inspector_collapsed;
                        }
                        // Add spacing between buttons
                        ui.add_space(5.0);
                        // Left Panel toggle button
                        if ui.button("Left Panel").clicked() {
                            layout_state.left_half_panel_collapsed = !layout_state.left_half_panel_collapsed;
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
                    
                    // Set clip rect to constrain content
                    ui.set_clip_rect(inspector_rect);
                    
                    // Allocate UI at the exact rect position
                    #[allow(deprecated)]
                    ui.allocate_ui_at_rect(inspector_rect, |ui| {
                        // Measure actual content area width (accounting for frame)
                        let right_panel_content_width = ui.available_width();
                        layout_state.right_panel_content_width = right_panel_content_width;
                        
                        ui.vertical(|ui| {
                            ui.heading("Inspector");
                        });
                    });
                });
        }
        
        // Left half panel - divides the 3D world space vertically (left 50%, full height)
        // Only show if not collapsed (toggled by button in bottom bar)
        if !layout_state.left_half_panel_collapsed {
            let viewport_rect = ctx.viewport_rect();
            let left_panel_end_x = layout_state.left_panel_end_x;
            let viewport_right_edge = if layout_state.inspector_collapsed {
                viewport_rect.right() // Extend to right edge when inspector is hidden
            } else {
                layout_state.right_panel_start_x // Stop at inspector when visible
            };
            
            // Calculate total available width and divide in half
            let total_viewport_width = viewport_right_edge - left_panel_end_x;
            let half_width = total_viewport_width / 2.0;
            let panel_y = layout_state.top_bars_height; // Start below top bars
            let panel_height = viewport_rect.height() - layout_state.top_bars_height - layout_state.bottom_bar_height; // Full height minus bars
            
            let left_half_panel_rect = egui::Rect::from_min_size(
                egui::pos2(left_panel_end_x, panel_y),
                egui::vec2(half_width, panel_height)
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
                            ui.heading("Left Panel");
                            ui.separator();
                            ui.label("This panel occupies the left half of the 3D world space.");
                            ui.label("The 3D view is on the right half.");
                        });
                    });
                });
        }
        
        // Streams panel - covers the 3D viewport when visible
        if streams_panel_state.is_visible {
            let viewport_rect = ctx.viewport_rect();
            let viewport_x = layout_state.left_panel_end_x;
            let viewport_y = layout_state.top_bars_height;
            let viewport_width = layout_state.right_panel_start_x - layout_state.left_panel_end_x;
            let viewport_height = viewport_rect.height() - layout_state.top_bars_height;
            
            let streams_panel_rect = egui::Rect::from_min_size(
                egui::pos2(viewport_x, viewport_y),
                egui::vec2(viewport_width.max(0.0), viewport_height.max(0.0))
            );
            
            egui::Area::new(egui::Id::new("streams_panel"))
                .fixed_pos(streams_panel_rect.min)
                .constrain(true)
                .interactable(true)
                .show(ctx, |ui| {
                    // Allocate rect to intercept clicks and block 3D world input
                    let _response = ui.allocate_rect(streams_panel_rect, egui::Sense::click());
                    
                    // Paint background to fully cover the 3D viewport
                    ui.painter().rect_filled(streams_panel_rect, 0.0, ui.style().visuals.panel_fill);
                    
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
