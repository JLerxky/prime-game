use bevy::prelude::*;
use protocol::data::control_data::ControlData;

use crate::engine::plugin::network::NetWorkState;

pub struct ControlEventPlugin;

impl Plugin for ControlEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ControlEvent>()
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

fn event_listener_system(
    mut control_event_reader: Local<EventReader<ControlEvent>>,
    control_events: Res<Events<ControlEvent>>,
    net_state: ResMut<NetWorkState>,
    // player_state: Res<PlayerState>,
) {
    for control_event in control_event_reader.iter(&control_events) {
        if let Ok(mut control_queue) = net_state.control_queue.lock() {
            if let Some(control_data) = control_queue.last() {
                if control_data.direction == control_event.direction
                    && control_data.action == control_event.action
                {
                    continue;
                }
            }
            control_queue.push(ControlData {
                uid: 4721,
                direction: control_event.direction,
                action: control_event.action,
            });
            println!("收到控制事件: {:?}", control_event);
        }
    }
}
