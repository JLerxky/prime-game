use bevy::prelude::*;
use protocol::data::control_data::ControlData;

use crate::engine::plugin::network::NetWorkState;

pub struct ControlEventPlugin;

impl Plugin for ControlEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ControlLastOne {
            control: ControlData {
                uid: 0,
                direction: (0., 0.),
                action: 0,
            },
        })
        .add_event::<ControlEvent>()
        .add_system(event_listener_system.system());
    }
}

#[derive(Debug)]
pub struct ControlEvent {
    //方向 模拟输入
    pub direction: (f32, f32),
    // 动作 0停止, 1移动, 2跳跃
    pub action: u8,
}

pub struct ControlLastOne {
    control: ControlData,
}

fn event_listener_system(
    mut control_event_reader: Local<EventReader<ControlEvent>>,
    control_events: Res<Events<ControlEvent>>,
    net_state: ResMut<NetWorkState>,
    mut control_last_one: ResMut<ControlLastOne>,
    // player_state: Res<PlayerState>,
) {
    for control_event in control_event_reader.iter(&control_events) {
        if let Ok(mut control_queue) = net_state.control_queue.lock() {
            if control_last_one.control.direction == control_event.direction
                && control_last_one.control.action == control_event.action
            {
                continue;
            }
            control_queue.push(ControlData {
                uid: 0,
                direction: control_event.direction,
                action: control_event.action,
            });
            control_last_one.control = ControlData {
                uid: 0,
                direction: control_event.direction,
                action: control_event.action,
            };
            // println!("收到控制事件: {:?}", control_event);
        }
    }
}
