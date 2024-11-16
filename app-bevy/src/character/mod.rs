use avian2d::{math::*, prelude::*};
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};

// Character Movement & Physics
mod controller;
pub use controller::{CharacterControllerBundle, CharacterControllerPlugin};

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    render: MaterialMesh2dBundle<ColorMaterial>,
    controller: CharacterControllerBundle,
}

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // 20px is 1 "meter" for simulation purposes
        let physics = PhysicsPlugins::default().with_length_unit(20.0);
        let cm = Material2dPlugin::<CoolMaterial>::default();
        app.add_plugins(physics)
            .add_plugins(cm)
            .add_plugins(CharacterControllerPlugin)
            .add_systems(Startup, spawn_player);
    }
}

#[derive(AsBindGroup, Clone, Debug, Asset, TypePath)]
pub struct CoolMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for CoolMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cool.wgsl".into()
    }
}

impl Default for CoolMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::new(1.0, 0.05, 0.9, 1.0),
        }
    }
}

// Spawn a character 5m in the air
fn spawn_player(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<CoolMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(Circle { radius: 10.0 }));
    // let color = Color::hsl(0.5, 0.95, 0.8);
    let cool_material = CoolMaterial::default();
    let material = materials.add(cool_material);
    let transform = Transform::from_xyz(0.0, 100.0, 0.0);

    let collider = Collider::circle(10.0);
    let character_controller = CharacterControllerBundle::new(collider).with_movement(
        0.92,
        400.0,
        1250.0,
        (30.0 as Scalar).to_radians(),
    );
    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh,
            material,
            transform,
            ..default()
        },
        character_controller,
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ColliderDensity(2.0),
        GravityScale(1.5),
    ));
}
