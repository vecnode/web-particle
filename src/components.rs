// components.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct FixCameraButton;

#[derive(Component)]
pub struct CameraTopButton;

#[derive(Component)]
pub struct CameraPositionText;

#[derive(Resource, Default)]
pub struct CameraViewChanged {
    pub needs_reset: bool,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct Motion1Button;

#[derive(Component)]
pub struct ShowTrajectoryButton;

#[derive(Component)]
pub struct TrajectoryCircle {
    pub particle_entity: Entity,
}

#[derive(Resource, Default)]
pub struct ParticleSelectionState {
    pub selected_particles: std::collections::HashSet<Entity>,
}

#[derive(Resource, Default)]
pub struct ParticlePositions {
    pub positions: std::collections::HashMap<Entity, Vec3>,
}

#[derive(Resource)]
pub struct Motion1State {
    pub is_active: bool,
    pub rotation_speed: f32, // radians per second
}

impl Default for Motion1State {
    fn default() -> Self {
        Self {
            is_active: false,
            rotation_speed: 1.0, // 1 radian per second (about 57 degrees per second)
        }
    }
}

#[derive(Resource, Default)]
pub struct TrajectoryState {
    pub is_visible: bool,
}

#[derive(Component)]
pub struct SelectionBox;

#[derive(Resource, Default)]
pub struct SelectionBoxState {
    pub is_active: bool,
    pub start_position: Option<Vec2>,
    pub current_position: Option<Vec2>,
}
