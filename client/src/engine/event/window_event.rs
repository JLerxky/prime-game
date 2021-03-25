use bevy::{app::AppExit, prelude::*};

pub struct WindowEventPlugin;

impl Plugin for WindowEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<WindowEvent>()
            .add_system(keyboard_event_system.system())
            .add_system(event_listener_system.system());
    }
}

#[derive(Debug)]
enum WindowEvent {
    _QUIT,
}

fn event_listener_system(
    mut window_event_reader: Local<EventReader<WindowEvent>>,
    window_events: Res<Events<WindowEvent>>,
) {
    for window_event in window_event_reader.iter(&window_events) {
        println!("{:?}", window_event);
        match window_event {
            WindowEvent::_QUIT => {}
        }
    }
}

fn keyboard_event_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    let window = windows.get_primary_mut().unwrap();

    // 鼠标显示
    if keyboard_input.just_pressed(KeyCode::LAlt) {
        window.set_cursor_position(Vec2::new(window.width() / 2f32, window.height() / 2f32));
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
    if keyboard_input.just_released(KeyCode::LAlt) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    // ESC退出游戏
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit);
    }
}
