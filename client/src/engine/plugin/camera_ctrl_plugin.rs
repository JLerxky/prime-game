use bevy::prelude::*;
pub struct CameraCtrl;

impl Plugin for CameraCtrl {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(camera_ctrl_system.system());
    }
}

fn setup(mut commands: Commands) {
    commands
        // cameras
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraCtrl);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(CameraCtrl);
}

fn camera_ctrl_system(// diagnostics: Res<Diagnostics>,
    // mut query: Query<&mut Camera2dBundle, With<CameraCtrl>>,
) {
}
