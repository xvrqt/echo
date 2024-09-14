use avian2d::{math::*, prelude::*};
use bevy::{
    ecs::query::Has,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin},
};

// Coalesce movement inputs from different devices into a single event
#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    Jump,
}

// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

// The maximum angle a slope can have for a character controller
// to be able to climb and jump. If the slope is steeper than this angle,
// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    acceleration: MovementAcceleration,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        damping: Scalar,
        jump_impulse: Scalar,
        acceleration: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            acceleration: MovementAcceleration(acceleration),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        MovementBundle::new(0.9, 7.0, 30.0, PI * 0.45)
    }
}

// A bundle that contains the components needed for a basic
// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    movement: MovementBundle,
    collider: Collider,
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    ground_caster: ShapeCaster,
    character_controller: CharacterController,
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Sentinel type; used for queries
        let character_controller = CharacterController;
        // How a character jumps, slows, climbs slopes
        let movement = MovementBundle::default();

        // Character Controlled via external forces, can be knocked around
        let rigid_body = RigidBody::Dynamic;
        // Don't allow the character to rotate
        let locked_axes = LockedAxes::ROTATION_LOCKED;

        // Create a shape caster that shoots a version of the character 10 units "down"
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);
        let ground_caster = ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
            .with_max_time_of_impact(10.0);

        Self {
            movement,
            collider,
            rigid_body,
            locked_axes,
            ground_caster,
            character_controller,
        }
    }

    // Update the how a character moves
    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

// Sends a [MovementAction] based on keyboard_input
fn keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut movement_event_writer: EventWriter<MovementAction>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let spacebar = keyboard_input.just_pressed(KeyCode::Space);

    // -1 || 1 || 0 if the left, right, or both are pressed
    let direction = (right as i8 - left as i8) as Scalar;
    // Move the character in that direction
    if direction != 0.0 {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if spacebar {
        movement_event_writer.send(MovementAction::Jump);
    }
}

// Sends a [MovementAction] based on gamepad input
fn gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    gamepads: Res<Gamepads>,
    buttons: Res<ButtonInput<GamepadButton>>,
    mut movement_event_writer: EventWriter<MovementAction>,
) {
    for gamepad in gamepads.iter() {
        let axis_lx = GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        };
        if let Some(x) = axes.get(axis_lx) {
            movement_event_writer.send(MovementAction::Move(x as Scalar));
        }

        let jump_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };
        if buttons.just_pressed(jump_button) {
            movement_event_writer.send(MovementAction::Jump);
        }
    }
}

// Update if the character is "on the ground"
// Sets the [Grounded] status for CharacterControllers
fn update_grounded(
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
    mut commands: Commands,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // If our shape hits anything, we're above something solid
        let is_grounded = hits.iter().any(|hit| {
            // Check if we're sliding, falling, fading...
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        // Update the character controller
        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

// Responds to MovementAction events and updates the CharacterController accordingly
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>,
    )>,
) {
    // f32/f64 compatibility
    let delta_time = time.delta_seconds_f64().adjust_precision();
    for event in movement_event_reader.read() {
        for (movement_acceleration, jump_impulse, mut linear_velocity, is_grounded) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += *direction * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    if is_grounded {
                        linear_velocity.y = jump_impulse.0;
                    }
                }
            }
        }
    }
}

// Slows down movement in the x-direction
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        linear_velocity.x *= damping_factor.0;
    }
}

pub struct CharacterControllerPlugin;
impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        // Register the MovementAction
        app.add_event::<MovementAction>().add_systems(
            Update,
            (
                keyboard_input,
                gamepad_input,
                update_grounded,
                movement,
                apply_movement_damping,
            )
                .chain(),
        );
    }
}
