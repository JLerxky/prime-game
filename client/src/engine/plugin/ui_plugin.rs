use bevy::prelude::*;

use super::ping_plugin::PingState;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_egui::EguiPlugin)
            .add_startup_system(setup.system())
            .add_system(ui_system.system());
    }
}

fn setup(mut _commands: Commands) {}

fn ui_system(egui_context: ResMut<bevy_egui::EguiContext>, ping_state: Res<PingState>) {
    bevy_egui::egui::Window::new("性能监控").show(egui_context.ctx(), |ui| {
        ui.label(format!("{:.0} ping", ping_state.ping));
        ui.fonts();
    });
}
