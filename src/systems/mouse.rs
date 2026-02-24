// systems/mouse.rs
// Copyright (C) 2026 vecnode

use bevy::prelude::*;
use crate::components::MouseButtonState;

pub fn track_mouse_button_state(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut button_state: ResMut<MouseButtonState>,
) {
    // Update previous state BEFORE reading current state
    button_state.left_was_pressed = button_state.left_pressed;
    button_state.right_was_pressed = button_state.right_pressed;
    
    // Use pressed() as source of truth - always sync with ButtonInput
    // This ensures state matches actual button state, even if events are missed
    button_state.left_pressed = mouse_button_input.pressed(MouseButton::Left);
    button_state.right_pressed = mouse_button_input.pressed(MouseButton::Right);
}

pub fn cleanup_mouse_button_state(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut button_state: ResMut<MouseButtonState>,
    mut selection_box_state: ResMut<crate::components::SelectionBoxState>,
) {
    // Always sync with ButtonInput - this is the source of truth
    // If button is not pressed, clear state immediately
    if !mouse_button_input.pressed(MouseButton::Left) {
        button_state.left_pressed = false;
    }
    
    if !mouse_button_input.pressed(MouseButton::Right) {
        button_state.right_pressed = false;
        if selection_box_state.is_active {
            selection_box_state.is_active = false;
        }
    }
}
