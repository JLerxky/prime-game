use bevy::prelude::*;

pub struct MapEventPlugin;

impl Plugin for MapEventPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<MapEvent>();
    }
}
pub enum MapEvent {
    // Clean,
    Add,
}
