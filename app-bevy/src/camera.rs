use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;

pub struct CameraPlugin;
fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle {
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    };
    commands.spawn((camera, BloomSettings::NATURAL));
}
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
