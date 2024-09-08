use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod character;
use character::CharacterPlugin;

mod camera;
use camera::CameraPlugin;

mod physics;
use physics::PhysicsPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(PhysicsPlugin);

    // Debug Builds Only
    #[cfg(debug_assertions)]
    {
        use bevy::diagnostic::LogDiagnosticsPlugin;
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_plugins(LogDiagnosticsPlugin::default());
    }
    app.run();
}
