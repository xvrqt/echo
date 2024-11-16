use avian2d::{math::*, prelude::*};
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::{render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

mod camera;
use camera::CameraPlugin;

mod character;
use character::PlayerPlugin;

mod platforms;
use platforms::PlatformPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PlatformPlugin)
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
