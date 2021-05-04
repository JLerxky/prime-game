use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy_egui::{
    egui::{epaint::Shadow, Color32, FontDefinitions, Frame, Id, Label, Stroke, TextureId},
    EguiPlugin,
};

const MAIN_MENU_TEXTURE_ID: u64 = 0;
const BUTTON_TEXTURE_ID: u64 = 1;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugin(EguiPlugin)
            .insert_resource(UIState {
                ping: 999f32,
                windows_enabled: [true, false],
            })
            .add_startup_system(setup.system())
            .add_system(ui_system.system());
    }
}

pub struct UIState {
    pub ping: f32,
    pub windows_enabled: [bool; 2],
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

fn setup(asset_server: Res<AssetServer>, mut egui_context: ResMut<bevy_egui::EguiContext>) {
    let texture_handle = asset_server.load("textures/prime/hub/box.png");
    egui_context.set_egui_texture(MAIN_MENU_TEXTURE_ID, texture_handle);
    let texture_handle = asset_server.load("textures/prime/hub/shuriken.png");
    egui_context.set_egui_texture(BUTTON_TEXTURE_ID, texture_handle);
}

fn ui_system(
    egui_context: ResMut<bevy_egui::EguiContext>,
    mut ui_state: ResMut<UIState>,
    diagnostics: Res<bevy::diagnostic::Diagnostics>,
    mut app_exit_events: EventWriter<bevy::app::AppExit>,
    window: Res<WindowDescriptor>,
) {
    if ui_state.windows_enabled[0] {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) =
            diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }
        bevy_egui::egui::Window::new("性能监控")
            .title_bar(false)
            .id(Id::new(1))
            .resizable(false)
            .fixed_pos((0., 0.))
            .enabled(ui_state.windows_enabled[0])
            .frame(Frame {
                margin: bevy_egui::egui::Vec2::new(5., 5.),
                corner_radius: 0.,
                shadow: Shadow {
                    extrusion: 0.,
                    color: Color32::from_rgb(0, 0, 0),
                },
                fill: Color32::from_rgba_unmultiplied(131, 106, 98, 255),
                stroke: Stroke {
                    width: 0.,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
                },
            })
            .show(egui_context.ctx(), |ui| {
                egui_context.ctx().set_fonts(get_default_fonts());
                ui.add(
                    Label::new(format!("{:.0} ping", ui_state.ping))
                        .text_style(bevy_egui::egui::TextStyle::Heading),
                );
                ui.add(
                    Label::new(format!("{:.0} fps", fps))
                        .text_style(bevy_egui::egui::TextStyle::Heading),
                );
            });
    }

    if ui_state.windows_enabled[1] {
        bevy_egui::egui::Window::new("主菜单")
            .title_bar(false)
            .id(Id::new(2))
            .resizable(false)
            // .default_pos((250., 75.))
            .enabled(ui_state.windows_enabled[1])
            .fixed_rect(bevy_egui::egui::Rect::from_center_size(
                bevy_egui::egui::Pos2::new(window.width / 2., window.height / 2.),
                bevy_egui::egui::Vec2::new(300., 300.),
            ))
            // .fixed_pos((250., 75.))
            .frame(Frame {
                margin: bevy_egui::egui::Vec2::new(0., 0.),
                corner_radius: 50.,
                shadow: Shadow {
                    extrusion: 0.,
                    color: Color32::from_rgb(0, 0, 0),
                },
                fill: Color32::from_rgba_unmultiplied(131, 106, 98, 255),
                stroke: Stroke {
                    width: 0.,
                    color: Color32::from_rgba_unmultiplied(0, 0, 0, 0),
                },
            })
            .show(egui_context.ctx(), |ui| {
                ui.image(TextureId::User(MAIN_MENU_TEXTURE_ID), (300., 300.));
                let widget_rect = bevy_egui::egui::Rect::from_center_size(
                    ui.min_rect().min + bevy_egui::egui::Vec2::new(150., 100.),
                    bevy_egui::egui::Vec2::new(100., 32.),
                );
                if ui
                    .put(
                        widget_rect,
                        bevy_egui::egui::Button::new("性能监控")
                            .fill(Some(Color32::from_rgba_unmultiplied(0, 0, 0, 0))),
                    )
                    .clicked()
                {
                    ui_state.windows_enabled[0] = !ui_state.windows_enabled[0];
                }
                let widget_rect = bevy_egui::egui::Rect::from_center_size(
                    ui.min_rect().min + bevy_egui::egui::Vec2::new(150., 150.),
                    bevy_egui::egui::Vec2::new(100., 32.),
                );
                if ui
                    .put(
                        widget_rect,
                        bevy_egui::egui::Button::new("退出游戏")
                            .fill(Some(Color32::from_rgba_unmultiplied(0, 0, 0, 0))),
                    )
                    .clicked()
                {
                    app_exit_events.send(bevy::app::AppExit);
                }
            });
    }
}
