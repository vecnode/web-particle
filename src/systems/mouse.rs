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
    
    // Read current state directly from ButtonInput
    // ButtonInput.pressed() reflects actual physical button state even when Egui absorbs input
    // This ensures we track state correctly regardless of where cursor is
    button_state.left_pressed = mouse_button_input.pressed(MouseButton::Left);
    button_state.right_pressed = mouse_button_input.pressed(MouseButton::Right);
}

pub fn cleanup_mouse_button_state(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut button_state: ResMut<MouseButtonState>,
    mut selection_box_state: ResMut<crate::components::SelectionBoxState>,
) {
    // Safety: If state says pressed but button is not actually pressed, force release
    if button_state.left_pressed && !mouse_button_input.pressed(MouseButton::Left) {
        button_state.left_pressed = false;
    }
    
    // Handle right button release transitions unconditionally (even when over Egui)
    // This ensures releases are always processed, regardless of where cursor is
    if button_state.right_was_pressed && !button_state.right_pressed {
        // Button was released - always deactivate selection box
        if selection_box_state.is_active {
            selection_box_state.is_active = false;
        }
    }
    
    // Safety: If state says pressed but button is not actually pressed, force release
    if button_state.right_pressed && !mouse_button_input.pressed(MouseButton::Right) {
        button_state.right_pressed = false;
        // Also release selection box if right button was stuck
        if selection_box_state.is_active {
            selection_box_state.is_active = false;
        }
    }
}
