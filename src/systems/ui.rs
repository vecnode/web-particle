// systems/ui.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::{FixCameraButton, CameraTopButton, CameraPositionText};
use crate::constants::{UI_SIDEBAR_WIDTH_PERCENT, UI_FONT_SIZE, UI_PADDING};

pub fn spawn_ui(mut commands: Commands) {
    // Left sidebar
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(UI_SIDEBAR_WIDTH_PERCENT),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(UI_PADDING)),
            flex_direction: FlexDirection::Column,
            ..default()
        }, 
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
    )).with_children(|parent| {
        // Controls text
        parent.spawn((
            Text::new("Controls\n\nWASD - Move\nMouse (Left) - Look\n\nParticles: 50"),
            TextFont {
                font_size: UI_FONT_SIZE,
                ..default()
            },
        ));
        
        // Camera Front button
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
                Text::new("Camera Front"),
                TextFont {
                    font_size: UI_FONT_SIZE,
                    ..default()
                },
            ));
        });
        
        // Camera Top button
        parent.spawn((
            Node {
                margin: UiRect::top(Val::Px(10.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
            Interaction::default(),
            CameraTopButton,
        )).with_children(|button_parent| {
            button_parent.spawn((
                Text::new("Camera Top"),
                TextFont {
                    font_size: UI_FONT_SIZE,
                    ..default()
                },
            ));
        });
        
        // Camera position display
        parent.spawn((
            Text::new("X: 0\nY: 0\nZ: 0"),
            TextFont {
                font_size: UI_FONT_SIZE,
                ..default()
            },
            CameraPositionText,
        ));
    });
}
