use std::time::SystemTime;

use bevy::prelude::*;

use crate::engine::plugin::ui_plugin::UIState;

pub struct HeartBeatEventPlugin;

impl Plugin for HeartBeatEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<HeartBeatEvent>()
            .add_system(event_listener_system.system());
    }
}

pub struct HeartBeatEvent {
    pub time: u128,
}

fn event_listener_system(
    mut hb_event_reader: EventReader<HeartBeatEvent>,
    // _hb_event_writer: EventWriter<HeartBeatEvent>,
    mut ui_state: ResMut<UIState>,
) {
    for heart_beat_event in hb_event_reader.iter() {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            - heart_beat_event.time;
        ui_state.ping = time as f32;
    }
}
