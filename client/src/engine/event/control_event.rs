use std::sync::{Arc, Mutex};

use bevy::prelude::*;
use protocol::data::control_data::ControlData;

pub struct ControlEventPlugin;

impl Plugin for ControlEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let control_queue: Vec<ControlData> = Vec::new();
        let control_queue = Arc::new(Mutex::new(control_queue));

        app.add_resource(ControlState { control_queue })
            .add_event::<ControlEvent>()
            .add_system(event_listener_system.system());
    }
}

pub struct ControlEvent {
    //方向 模拟输入
    pub direction: (f32, f32),
    // 动作 0停止, 1移动, 2跳跃
    pub action: u8,
}

pub struct ControlState {
    control_queue: Arc<Mutex<Vec<ControlData>>>,
}

fn event_listener_system(
    mut control_event_reader: Local<EventReader<ControlEvent>>,
    control_events: Res<Events<ControlEvent>>,
) {
    for control_event in control_event_reader.iter(&control_events) {
        println!("{}", control_event.action);
    }
}
