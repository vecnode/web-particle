// main.rs
// Copyright (C) 2026 vecnode

mod components;
mod constants;
mod plugins;
mod setup;
mod systems;

use bevy::prelude::*;
use bevy::camera::Viewport;
use bevy_egui::{EguiPlugin, EguiGlobalSettings, PrimaryEguiContext, EguiPrimaryContextPass};

use setup::*;
use systems::*;
use components::{CameraViewChanged, ParticleSelectionState, ParticlePositions, Motion1State, TrajectoryState, SelectionBoxState, EguiLayoutState};
use constants::WORLD_BACKGROUND_COLOR;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(EguiPlugin::default())
        .add_plugins(plugins::viewport_constrained_camera::ViewportConstrainedCameraPlugin)
        .insert_resource(EguiGlobalSettings {
            auto_create_primary_context: false,
            ..default()
        })
        .insert_resource(ClearColor(WORLD_BACKGROUND_COLOR))
        .init_resource::<CameraViewChanged>()
        .init_resource::<ParticleSelectionState>()
        .init_resource::<ParticlePositions>()
        .init_resource::<Motion1State>()
        .init_resource::<TrajectoryState>()
        .init_resource::<SelectionBoxState>()
        .init_resource::<components::MouseButtonState>()
        .init_resource::<components::CameraProjectionState>()
        .init_resource::<components::EguiLayoutState>()
        .init_resource::<components::GridState>()
        .init_resource::<components::ParticleBoundsState>()
        .init_resource::<components::ParticleGroupState>()
        .init_resource::<components::StreamsPanelState>()
        .init_resource::<components::ParticleCreationState>()
        .add_systems(
            Startup,
            (
                spawn_axes,
                spawn_grid,
                setup_camera_and_lights,
                setup_split_screen_cameras,
            ),
        )
        .add_systems(
            Update,
            (
                track_mouse_button_state,
                cleanup_mouse_button_state,
                update_grid_dimensions,
                update_particle_bounds,
                update_particle_group_transform,
                handle_particle_selection,
                animate_motion1_particles,
                update_trajectory_visualization,
                handle_right_mouse_button,
                update_selection_box_visual,
                process_selection_box,
                handle_particle_creation,
                handle_particle_removal,
            ),
        )
        .add_systems(
            PostUpdate,
            reset_viewport_constrained_camera_after_view_change,
        )
        .add_systems(
            Update,
            update_camera_viewports,
        )
        .add_systems(
            EguiPrimaryContextPass,
            egui_controls_ui,
        )
        .run();
}

fn setup_split_screen_cameras(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    // Disable auto-create primary context
    egui_global_settings.auto_create_primary_context = false;
    
    // Single camera for 3D world (will take remaining space on right)
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 60.0_f32.to_radians(), // 60 degrees FOV
            ..default()
        }),
        Transform::from_translation(crate::constants::CAMERA_START_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
        plugins::viewport_constrained_camera::ViewportConstrainedCamera::default(),
        plugins::viewport_constrained_camera::ViewportConstrainedCameraState {
            pitch: 0.0,
            yaw: 0.0,
            initialized: false,
        },
        crate::components::RightCamera,
    ));
    
    // Primary Egui context camera (renders UI on top)
    commands.spawn((
        PrimaryEguiContext,
        Camera2d::default(),
        Camera {
            order: 10,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
    ));
}

fn update_camera_viewports(
    window: Query<&Window>,
    mut right_camera: Query<&mut Camera, With<crate::components::RightCamera>>,
    layout_state: Res<EguiLayoutState>,
) {
    let Ok(window) = window.single() else { return };
    let physical_size = window.physical_size();
    let scale_factor = window.scale_factor() as f32;
    
    // Use actual panel positions from Egui layout (in logical pixels, convert to physical)
    let left_panel_end_physical = (layout_state.left_panel_end_x * scale_factor) as u32;
    let top_bars_height_physical = (layout_state.top_bars_height * scale_factor) as u32;
    let bottom_bar_height_physical = (layout_state.bottom_bar_height * scale_factor) as u32;
    
    // Calculate viewport width: extend to right edge if inspector is collapsed, otherwise stop at inspector
    let viewport_right_edge = if layout_state.inspector_collapsed {
        physical_size.x // Extend to right edge of window when inspector is hidden
    } else {
        (layout_state.right_panel_start_x * scale_factor) as u32 // Stop at inspector when visible
    };
    
    // Calculate total available space: from left panel end to right edge (inspector or window edge)
    // Height: from below top bars to above bottom bar
    let total_viewport_width = viewport_right_edge.saturating_sub(left_panel_end_physical);
    let viewport_height = physical_size.y.saturating_sub(top_bars_height_physical).saturating_sub(bottom_bar_height_physical);
    
    // Calculate camera viewport: if 3D viewer is hidden, set size to 0; otherwise calculate based on left panel
    if let Ok(mut camera) = right_camera.single_mut() {
        if !layout_state.d3_viewer_visible {
            // Hide 3D viewer by setting viewport size to 0
            camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(0, 0),
                ..default()
            });
        } else {
            // Calculate camera viewport: if left half panel is visible, use right 50%, otherwise use full width
            let (camera_viewport_x, camera_viewport_width) = if layout_state.left_half_panel_collapsed {
                // Left panel is hidden: 3D world uses full width
                (left_panel_end_physical, total_viewport_width)
            } else {
                // Left panel is visible: 3D world uses right half (50% width)
                let half_width = total_viewport_width / 2;
                (left_panel_end_physical + half_width, half_width)
            };
            
            // Camera viewport (right half when left panel visible, full width when left panel hidden)
            camera.viewport = Some(Viewport {
                physical_position: UVec2::new(camera_viewport_x, top_bars_height_physical),
                physical_size: UVec2::new(camera_viewport_width, viewport_height),
                ..default()
            });
        }
    }
}
