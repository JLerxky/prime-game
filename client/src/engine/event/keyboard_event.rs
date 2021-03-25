use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::engine::plugin::player::Player;

pub struct KeyboardEventPlugin;

impl Plugin for KeyboardEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_event_system.system());
    }
}

fn keyboard_event_system(
    mut keyboard_event_reader: Local<EventReader<KeyboardInput>>,
    keyboard_events: Res<Events<KeyboardInput>>,
    mut player_info: Query<&mut Player>,
) {
    let mut player = player_info.iter_mut().next().unwrap();
    for event in keyboard_event_reader.iter(&keyboard_events) {
        match event.key_code {
            Some(KeyCode::Space) => match event.state {
                bevy::input::ElementState::Pressed => {}
                bevy::input::ElementState::Released => {
                    player.jumped = false;
                }
            },
            Some(_) => {}
            None => {}
        }
    }
}
