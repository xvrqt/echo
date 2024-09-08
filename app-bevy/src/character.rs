use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin},
};

use crate::physics::Physics;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    render: MaterialMesh2dBundle<ColorMaterial>,
    physics: Physics,
}

#[derive(Component)]
struct Player;

// Spawn a character at the origin
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(Circle { radius: 50.0 }));
    let color = Color::hsl(0.5, 0.95, 0.8);
    let material = materials.add(color);
    let transform = Transform::from_xyz(0.0, 100.0, 0.0);
    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh,
            material,
            transform,
            ..default()
        },
        Physics::default(),
    ));
}
pub struct CharacterPlugin;
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Wireframe2dPlugin)
            .add_systems(Startup, spawn_player);
    }
}
