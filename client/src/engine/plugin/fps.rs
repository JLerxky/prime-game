use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
pub struct Fps;

impl Plugin for Fps {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            // .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
            .add_startup_system(add_fps_system.system())
            .add_system(change_fps_system.system());
    }
}

fn add_fps_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/YouZai.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "0 fps".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(Fps);
}

fn change_fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<Fps>>) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }

        text.sections[0].value = format!("{:.0} fps", fps);
    }
}
