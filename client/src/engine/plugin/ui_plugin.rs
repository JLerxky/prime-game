use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            .insert_resource(UIState { ping: 999f32 })
            .add_startup_system(setup.system())
            .add_system(ui_system.system());
    }
}

pub struct UIState {
    pub ping: f32,
}

fn setup(mut _commands: Commands) {}

fn ui_system(
    egui_context: ResMut<bevy_egui::EguiContext>,
    ui_state: Res<UIState>,
    diagnostics: Res<bevy::diagnostic::Diagnostics>,
) {
    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
    {
        if let Some(fps_avg) = fps_diagnostic.average() {
            fps = fps_avg;
        }
    }
    bevy_egui::egui::Window::new("性能监控").show(egui_context.ctx(), |ui| {
        ui.label(format!("{:.0} ping", ui_state.ping));
        ui.label(format!("{:.0} fps", fps));
    });
}
