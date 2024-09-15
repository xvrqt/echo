use bevy::core_pipeline::bloom::BloomSettings;
use bevy::math::vec3;
use bevy::pbr::add_clusters;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::character::Player;

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

const CAMERA_DECAY_RATE: f32 = 200.0;
fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera.translation = camera
        .translation
        .move_towards(direction, CAMERA_DECAY_RATE * time.delta_seconds());
}
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera);
    }
}
