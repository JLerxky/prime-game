use bevy::{input::keyboard::KeyboardInput, prelude::*};

use crate::engine::plugin::{camera_ctrl::CameraCtrl, player::Player};

pub struct MoveEventPlugin;

impl Plugin for MoveEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MoveEvent>()
            .add_system(keyboard_event_system.system())
            .add_system(event_listener_system.system());
    }
}

#[derive(Debug)]
enum MoveEvent {
    UP(bool),
    DOWN(bool),
    LEFT(bool),
    RIGHT(bool),
}

fn event_listener_system(
    mut move_event_reader: Local<EventReader<MoveEvent>>,
    move_events: Res<Events<MoveEvent>>,
    mut camera_transform_query: Query<&mut Transform, With<CameraCtrl>>,
    mut player_query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let mut camera_transform = camera_transform_query.iter_mut().next().unwrap();
    let (mut player_transform, mut player) = player_query.iter_mut().next().unwrap();
    for move_event in move_event_reader.iter(&move_events) {
        match move_event {
            MoveEvent::UP(state) => {
                if *state {
                    // camera_transform.translation += Vec3::new(0f32, 10f32, 0f32);
                    // player_transform.translation += Vec3::new(0f32, 10f32, 0f32);
                    player.velocity.y = 1f32;
                } else {
                    if player.velocity.y > 0f32 {
                        player.velocity.y -= 1f32;
                    }
                }
            }
            MoveEvent::DOWN(state) => {
                if *state {
                    // camera_transform.translation += Vec3::new(0f32, -10f32, 0f32);
                    // player_transform.translation += Vec3::new(0f32, -10f32, 0f32);
                    player.velocity.y = -1f32;
                } else {
                    if player.velocity.y < 0f32 {
                        player.velocity.y -= -1f32;
                    }
                }
            }
            MoveEvent::LEFT(state) => {
                if *state {
                    // camera_transform.translation += Vec3::new(-10f32, 0f32, 0f32);
                    // player_transform.translation += Vec3::new(-10f32, 0f32, 0f32);
                    player.velocity.x = -1f32;
                } else {
                    if player.velocity.x < 0f32 {
                        player.velocity.x -= -1f32;
                    }
                }
            }
            MoveEvent::RIGHT(state) => {
                if *state {
                    // camera_transform.translation += Vec3::new(10f32, 0f32, 0f32);
                    // player_transform.translation += Vec3::new(10f32, 0f32, 0f32);
                    player.velocity.x = 1f32;
                } else {
                    if player.velocity.x > 0f32 {
                        player.velocity.x -= 1f32;
                    }
                }
            }
        }
    }
}

fn keyboard_event_system(
    mut keyboard_event_reader: Local<EventReader<KeyboardInput>>,
    keyboard_events: Res<Events<KeyboardInput>>,
    mut move_events: ResMut<Events<MoveEvent>>,
) {
    for event in keyboard_event_reader.iter(&keyboard_events) {
        match event.key_code {
            Some(KeyCode::W) => match event.state {
                bevy::input::ElementState::Pressed => {
                    move_events.send(MoveEvent::UP(true));
                }
                bevy::input::ElementState::Released => {
                    move_events.send(MoveEvent::UP(false));
                }
            },
            Some(KeyCode::S) => match event.state {
                bevy::input::ElementState::Pressed => {
                    move_events.send(MoveEvent::DOWN(true));
                }
                bevy::input::ElementState::Released => {
                    move_events.send(MoveEvent::DOWN(false));
                }
            },
            Some(KeyCode::A) => match event.state {
                bevy::input::ElementState::Pressed => {
                    move_events.send(MoveEvent::LEFT(true));
                }
                bevy::input::ElementState::Released => {
                    move_events.send(MoveEvent::LEFT(false));
                }
            },
            Some(KeyCode::D) => match event.state {
                bevy::input::ElementState::Pressed => {
                    move_events.send(MoveEvent::RIGHT(true));
                }
                bevy::input::ElementState::Released => {
                    move_events.send(MoveEvent::RIGHT(false));
                }
            },
            Some(_) => {}
            None => {}
        }
    }
}
