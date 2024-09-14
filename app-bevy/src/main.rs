use avian2d::{math::*, prelude::*};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod character;
use character::PlayerPlugin;

mod camera;
use camera::CameraPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0));

    // Debug Builds Only
    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default());
    }
    app.run();
}
