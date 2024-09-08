use bevy::math::vec3;
use bevy::prelude::*;
use float_ord::FloatOrd;

const GRAVITY_ACCEL: Vec3 = Vec3::new(0.0, -1.0, 0.0);

#[derive(Component)]
struct Velocity(Vec3);
#[derive(Component)]
struct Acceleration(Vec3);
#[derive(Component)]
struct EnableGravity(bool);

#[derive(Bundle)]
pub struct DynamicObject {
    physics: Physics,
    transform: TransformBundle,
}

impl Default for DynamicObject {
    fn default() -> Self {
        DynamicObject {
            transform: TransformBundle::default(),
            physics: Physics {
                velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
                acceleration: Acceleration(Vec3::new(0.0, 0.0, 0.0)),
                affected_by_gravity: EnableGravity(false),
            },
        }
    }
}

#[derive(Component)]
pub struct Physics {
    velocity: Velocity,
    acceleration: Acceleration,
    affected_by_gravity: EnableGravity,
}

impl Default for Physics {
    fn default() -> Self {
        Physics {
            velocity: Velocity(Vec3::new(0.0, 0.0, 0.0)),
            acceleration: Acceleration(Vec3::new(0.0, 0.0, 0.0)),
            affected_by_gravity: EnableGravity(true),
        }
    }
}

pub struct PhysicsPlugin;
fn update_positions(mut dynamic_objects: Query<(&mut Transform, &mut Physics)>, time: Res<Time>) {
    for (mut transform, mut physics) in &mut dynamic_objects {
        // Update position
        transform.translation += physics.velocity.0;
        transform.translation.y = std::cmp::max(FloatOrd(0.0), FloatOrd(transform.translation.y)).0;
        // Update velocity
        let acceleration = physics.acceleration.0;
        physics.velocity.0 += acceleration * time.delta_seconds();
        // Update acceleration
        if physics.affected_by_gravity.0 {
            physics.velocity.0 += GRAVITY_ACCEL * 0.05;
        }
    }
}
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_positions);
    }
}
