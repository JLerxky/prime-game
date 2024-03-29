use bevy::prelude::*;
extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub struct Clipboard;

impl Plugin for Clipboard {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(clipboard_system.system());
    }
}

fn clipboard_system(mut char_input_events: EventReader<ReceivedCharacter>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    for event in char_input_events.iter() {
        if event.char == "\u{3}".chars().next().unwrap() {
            ctx.set_contents("some string".to_owned()).unwrap();
        }
        if event.char == "\u{16}".chars().next().unwrap() {
            println!("{:?}", ctx.get_contents());
        }
    }
}
