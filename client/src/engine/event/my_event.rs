use bevy::prelude::*;

pub struct MyEventPlugin;

impl Plugin for MyEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MyEvent>()
            .add_system(event_listener_system.system());
    }
}

pub struct MyEvent {
    pub message: String,
}

fn event_listener_system(
    mut my_event_reader: EventReader<MyEvent>,
    //mut my_event_writer: EventWriter<MyEvent>,
) {
    for my_event in my_event_reader.iter() {
        println!("{}", my_event.message);
    }
}
