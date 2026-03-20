use bevy::prelude::*;

use crate::aircraft::ControlInput;

/// Read keyboard state and write to the ControlInput component.
///
/// Key mapping:
/// - W / S: Pitch (W = nose up, S = nose down)
/// - A / D: Roll (A = roll left, D = roll right)
/// - Q / E: Yaw (Q = yaw left, E = yaw right)
/// - Shift: Increase throttle
/// - Ctrl: Decrease throttle
pub fn handle_keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ControlInput>,
) {
    for mut input in query.iter_mut() {
        // Reset inputs each frame
        input.pitch = 0.0;
        input.roll = 0.0;
        input.yaw = 0.0;
        input.throttle_change = 0.0;

        // Pitch: W = nose up (positive pitch), S = nose down (negative pitch)
        if keys.pressed(KeyCode::KeyW) {
            input.pitch += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            input.pitch -= 1.0;
        }

        // Roll: A = roll left (negative), D = roll right (positive)
        if keys.pressed(KeyCode::KeyA) {
            input.roll -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            input.roll += 1.0;
        }

        // Yaw: Q = yaw left (positive Y rotation), E = yaw right (negative)
        if keys.pressed(KeyCode::KeyQ) {
            input.yaw += 1.0;
        }
        if keys.pressed(KeyCode::KeyE) {
            input.yaw -= 1.0;
        }

        // Throttle: Shift = increase, Ctrl = decrease
        if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            input.throttle_change += 1.0;
        }
        if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) {
            input.throttle_change -= 1.0;
        }
    }
}
