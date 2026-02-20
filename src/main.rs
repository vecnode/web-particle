// web-particle - main.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin};

#[derive(Component)]
struct Particle {
    angle: f32,
    speed: f32,
    radius: f32,
    normal: Vec3,
}

fn spawn_particles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    for i in 0..50 {
        let t = i as f32 * 0.1;
        let radius = 3.0 + (t * 2.0).sin() * 2.0;
        let normal = Vec3::new(t.sin(), t.cos(), (t * 2.0).sin() * 0.5).normalize();
        let right = normal.cross(Vec3::Y).normalize();
        let initial_pos = right * radius;
        
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.05))),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_translation(initial_pos),
            Particle {
                angle: 0.0,
                speed: 0.01 + (t * 0.5).sin() * 0.01,
                radius,
                normal,
            },
        ));
    }
}

fn spawn_axes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let length = 5.0;
    let radius = 0.01;
    
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(radius, length))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_translation(Vec3::X * length / 2.0).with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
    ));
    
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(radius, length))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_translation(Vec3::Y * length / 2.0),
    ));
    
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(radius, length))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_translation(Vec3::Z * length / 2.0).with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

fn spawn_grid(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let grid_size = 10.0; // Total size of the grid
    let grid_spacing = 1.0; // Spacing between grid lines
    let line_radius = 0.005; // Very thin lines
    let grid_color = Color::srgb(0.3, 0.3, 0.3); // Dark grey
    
    let half_size = grid_size / 2.0;
    let num_lines = (grid_size / grid_spacing) as i32 + 1;
    
    // Create grid lines along X axis (parallel to Z)
    for i in 0..num_lines {
        let z = -half_size + (i as f32 * grid_spacing);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(line_radius, grid_size))),
            MeshMaterial3d(materials.add(grid_color)),
            Transform::from_translation(Vec3::new(0.0, 0.0, z))
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ));
    }
    
    // Create grid lines along Z axis (parallel to X)
    for i in 0..num_lines {
        let x = -half_size + (i as f32 * grid_spacing);
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(line_radius, grid_size))),
            MeshMaterial3d(materials.add(grid_color)),
            Transform::from_translation(Vec3::new(x, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ));
    }
}

fn update_particles(mut query: Query<(&mut Transform, &mut Particle)>) {
    for (mut transform, mut particle) in query.iter_mut() {
        particle.angle += particle.speed;
        let right = particle.normal.cross(Vec3::Y).normalize();
        let up = right.cross(particle.normal).normalize();
        transform.translation = right * particle.radius * particle.angle.cos() + 
                               up * particle.radius * particle.angle.sin();
    }
}

fn spawn_ui(mut commands: Commands) {
    // Left sidebar (15%)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(15.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        }, 
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Controls\n\nWASD - Move\nMouse (Left) - Look\n\nParticles: 50"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ));
        
        parent.spawn((
            Node {
                margin: UiRect::top(Val::Px(20.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
            Interaction::default(),
            FixCameraButton,
        )).with_children(|button_parent| {
            button_parent.spawn((
                Text::new("Fix Camera 1"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
            ));
        });
        
        // Camera position display
        parent.spawn((
            Text::new("X: 0\nY: 0\nZ: 0"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
            CameraPositionText,
        ));
        
    });
}

#[derive(Component)]
struct FixCameraButton;

#[derive(Component)]
struct CameraPositionText;

fn handle_camera_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<FixCameraButton>)>,
    mut camera_query: Query<(&mut Transform, &mut FreeCamera), With<Camera3d>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok((mut transform, mut free_camera)) = camera_query.single_mut() {
                transform.translation = Vec3::new(0.0, 0.0, 15.0);
                transform.look_at(Vec3::ZERO, Vec3::Y);
                *free_camera = FreeCamera::default();
            }
        }
    }
}

fn update_camera_position_text(
    camera_query: Query<&Transform, With<Camera3d>>,
    mut text_query: Query<&mut Text, With<CameraPositionText>>,
) {
    if let Ok(transform) = camera_query.single() {
        let pos = transform.translation;
        let x = pos.x.round() as i32;
        let y = pos.y.round() as i32;
        let z = pos.z.round() as i32;
        
        if let Ok(mut text) = text_query.single_mut() {
            *text = Text::new(format!("X: {}\nY: {}\nZ: {}", x, y, z));
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, (spawn_particles, spawn_axes, spawn_grid, spawn_ui, |mut commands: Commands| {
            commands.spawn((
                Camera3d::default(),
                Transform::from_xyz(9.0, 7.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
                FreeCamera::default(),
            ));
            commands.spawn(DirectionalLight {
                illuminance: 3000.0,
                ..default()
            });
        }))
        .add_systems(Update, (update_particles, handle_camera_button, update_camera_position_text))
        .run();
}

