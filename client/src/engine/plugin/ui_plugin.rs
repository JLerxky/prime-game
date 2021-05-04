use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy_egui::{
    egui::{epaint::Shadow, Color32, FontDefinitions, Frame, Id, Stroke},
    EguiPlugin,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugin(EguiPlugin)
            .insert_resource(UIState { ping: 999f32 })
            .add_startup_system(setup.system())
            .add_system(ui_system.system());
    }
}

pub struct UIState {
    pub ping: f32,
}

fn get_default_fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let mut font_data: BTreeMap<String, std::borrow::Cow<'static, [u8]>> = BTreeMap::new();
    font_data.insert(
        "Ubuntu-Light".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../../../assets/fonts/YouZai.ttf")),
    );
    font_data.insert(
        "ProggyClean".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../../../assets/fonts/YouZai.ttf")),
    );
    font_data.insert(
        "Ubuntu-Light".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../../../assets/fonts/YouZai.ttf")),
    );
    font_data.insert(
        "NotoEmoji-Regular".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../../../assets/fonts/YouZai.ttf")),
    );
    font_data.insert(
        "emoji-icon-font".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../../../assets/fonts/YouZai.ttf")),
    );
    fonts.font_data = font_data;
    fonts
}

fn setup() {}

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
    bevy_egui::egui::Window::new("性能监控")
        // .title_bar(false)
        .id(Id::new(1))
        .resizable(false)
        .frame(Frame {
            margin: bevy_egui::egui::Vec2::new(5., 5.),
            corner_radius: 0.,
            shadow: Shadow {
                extrusion: 0.,
                color: Color32::from_rgb(0, 0, 0),
            },
            fill: Color32::from_rgba_premultiplied(100, 100, 100, 100),
            stroke: Stroke {
                width: 0.,
                color: Color32::from_rgb(0, 0, 0),
            },
        })
        .show(egui_context.ctx(), |ui| {
            egui_context.ctx().set_fonts(get_default_fonts());
            ui.label(format!("{:.0} ping", ui_state.ping));
            ui.label(format!("{:.0} fps", fps));
        });
}
