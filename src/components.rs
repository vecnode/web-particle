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
