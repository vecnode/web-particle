// main.rs
// Copyright (C) 2026 vecnode

mod components;
mod constants;
mod setup;
mod systems;

use bevy::prelude::*;
use bevy::camera_controller::free_camera::FreeCameraPlugin;

use setup::*;
use systems::*;
use components::{CameraViewChanged, ParticleSelectionState, ParticlePositions, Motion1State, TrajectoryState, SelectionBoxState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FreeCameraPlugin)
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
                spawn_ui,
                setup_camera_and_lights,
            ),
        )
        .add_systems(
            Update,
            (
                handle_camera_button,
                handle_camera_top_button,
                update_camera_position_text,
                handle_particle_selection,
                handle_motion1_button,
                animate_motion1_particles,
                handle_show_trajectory_button,
                update_trajectory_visualization,
                handle_right_mouse_button,
                update_selection_box_visual,
                process_selection_box,
            ),
        )
        .add_systems(
            PostUpdate,
            reset_free_camera_after_view_change,
        )
        .run();
}

