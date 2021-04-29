use bevy::prelude::*;
pub struct Ping;

impl Plugin for Ping {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(PingState { ping: 999f32 })
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            // .add_plugin(bevy::diagnostic::PrintDiagnosticsPlugin::default())
            .add_startup_system(add_ping_system.system())
            .add_system(change_ping_system.system());
    }
}

pub struct PingState {
    pub ping: f32,
}

fn add_ping_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                "0 ping".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(Ping);
}

fn change_ping_system(ping_state: Res<PingState>, mut query: Query<&mut Text, With<Ping>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:.0} ping", ping_state.ping);
    }
}
