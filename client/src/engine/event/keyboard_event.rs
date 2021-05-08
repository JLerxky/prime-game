use bevy::prelude::*;
use protocol::data::skill_data::SkillType;

use crate::engine::plugin::ui_plugin::UIState;

use super::{control_event::ControlEvent, skill_event::SkillEvent};

pub struct KeyboardEventPlugin;

impl Plugin for KeyboardEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_event_system.system());
    }
}

fn keyboard_event_system(
    mut control_events: EventWriter<ControlEvent>,
    mut skill_events: EventWriter<SkillEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mourse_input: Res<Input<MouseButton>>,
    mut ui_state: ResMut<UIState>,
    windows: Res<Windows>,
) {
    // 控制移动
    let x_axis = -(keyboard_input.pressed(KeyCode::A) as i8) as f32
        + (keyboard_input.pressed(KeyCode::D) as i8) as f32;
    let y_axis = -(keyboard_input.pressed(KeyCode::S) as i8) as f32
        + (keyboard_input.pressed(KeyCode::W) as i8) as f32;
    let action = 1u8 + (keyboard_input.pressed(KeyCode::LControl) as u8);
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
    // 控制菜单
    if keyboard_input.just_released(KeyCode::Escape) {
        ui_state.windows_enabled[1] = !ui_state.windows_enabled[1];
    }
    // 释放技能
    if mourse_input.just_pressed(MouseButton::Left) || keyboard_input.just_released(KeyCode::Space) {
        if let Some(window) = windows.get_primary() {
            let center_point = Vec2::new(window.width() / 2., window.height() / 2.);
            // println!("camera_point: {}", &center_point);
            if let Some(cursor_point) = window.cursor_position() {
                // println!("cursor_point: {}", &cursor_point);
                let direction = (cursor_point - center_point).normalize();
                // println!("direction: {}", &direction);
                skill_events.send(SkillEvent {
                    direction: (direction.x, direction.y),
                    skill_type: SkillType::Shot,
                });
            }
        }
    }
}
