use bevy::prelude::*;

use crate::engine::plugin::ui_plugin::UIState;

use super::control_event::ControlEvent;

pub struct KeyboardEventPlugin;

impl Plugin for KeyboardEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_event_system.system());
    }
}

fn keyboard_event_system(
    mut control_events: EventWriter<ControlEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_state: ResMut<UIState>,
) {
    let x_axis = -(keyboard_input.pressed(KeyCode::A) as i8) as f32
        + (keyboard_input.pressed(KeyCode::D) as i8) as f32;
    let y_axis = -(keyboard_input.pressed(KeyCode::S) as i8) as f32
        + (keyboard_input.pressed(KeyCode::W) as i8) as f32;
    let action = 1u8;
    if keyboard_input.just_released(KeyCode::Escape) {
        ui_state.windows_enabled[1] = !ui_state.windows_enabled[1];
    }
    // if y_axis != 0f32 {
    //     action = 2u8;
    // }
    if x_axis != 0f32 || y_axis != 0f32 {
        control_events.send(ControlEvent {
            direction: (x_axis, y_axis),
            action,
        });
    } else {
        control_events.send(ControlEvent {
            direction: (x_axis, y_axis),
            action: 0u8,
        });
    }
}
