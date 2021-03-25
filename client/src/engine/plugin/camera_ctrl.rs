use bevy::prelude::*;
pub struct CameraCtrl;

impl Plugin for CameraCtrl {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(camera_ctrl_system.system());
    }
}

fn setup(commands: &mut Commands) {
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .with(CameraCtrl)
        .spawn(CameraUiBundle::default())
        .with(CameraCtrl);
}

fn camera_ctrl_system(// diagnostics: Res<Diagnostics>,
    // mut query: Query<&mut Camera2dBundle, With<CameraCtrl>>,
) {
}
