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
use components::{CameraViewChanged, ParticleSelectionState, ParticlePositions, Motion1State, TrajectoryState, SelectionBoxState};
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
) {
    let Ok(window) = window.single() else { return };
    let physical_size = window.physical_size();
    let scale_factor = window.scale_factor() as f32;
    
    // Egui panel width (left side) and top bar height in physical pixels
    let egui_panel_width = (200.0 * scale_factor) as u32;
    let egui_top_bar_height = (30.0 * scale_factor) as u32;
    
    // Camera viewport takes remaining space (right side, below top bar)
    if let Ok(mut camera) = right_camera.single_mut() {
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(egui_panel_width, egui_top_bar_height),
            physical_size: UVec2::new(
                physical_size.x.saturating_sub(egui_panel_width),
                physical_size.y.saturating_sub(egui_top_bar_height),
            ),
            ..default()
        });
    }
}
