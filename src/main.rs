// main.rs
// Copyright (C) 2026 vecnode

mod components;
mod constants;
mod setup;
mod systems;

use bevy::prelude::*;
use bevy::camera::Viewport;
use bevy::camera_controller::free_camera::FreeCameraPlugin;
use bevy_egui::{EguiPlugin, EguiGlobalSettings, PrimaryEguiContext, EguiPrimaryContextPass, input::egui_wants_any_pointer_input};

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
        .add_plugins(FreeCameraPlugin)
        .insert_resource(EguiGlobalSettings {
            auto_create_primary_context: false,
            enable_absorb_bevy_input_system: true,
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
        .add_systems(
            Startup,
            (
                spawn_particles,
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
                handle_particle_selection.run_if(not(egui_wants_any_pointer_input)),
                animate_motion1_particles,
                update_trajectory_visualization,
                handle_right_mouse_button.run_if(not(egui_wants_any_pointer_input)),
                update_selection_box_visual.run_if(not(egui_wants_any_pointer_input)),
                process_selection_box.run_if(not(egui_wants_any_pointer_input)),
            ),
        )
        .add_systems(
            PostUpdate,
            reset_free_camera_after_view_change,
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
        bevy::camera_controller::free_camera::FreeCamera::default(),
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
    let right_panel_start_physical = (layout_state.right_panel_start_x * scale_factor) as u32;
    let top_bars_height_physical = (layout_state.top_bars_height * scale_factor) as u32;
    let bottom_bar_height_physical = (layout_state.bottom_bar_height * scale_factor) as u32;
    
    // Calculate viewport size: from left panel end to right panel start
    // Height: from below top bars to above bottom bar
    let viewport_width = right_panel_start_physical.saturating_sub(left_panel_end_physical);
    let viewport_height = physical_size.y.saturating_sub(top_bars_height_physical).saturating_sub(bottom_bar_height_physical);
    
    // Camera viewport takes remaining space (center, below both top bars, between left and right panels)
    if let Ok(mut camera) = right_camera.single_mut() {
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(left_panel_end_physical, top_bars_height_physical),
            physical_size: UVec2::new(viewport_width, viewport_height),
            ..default()
        });
    }
}
