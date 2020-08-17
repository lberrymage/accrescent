//! The [`System`] responsible for fly movement
//!
//! Makes a given [`Camera`] with a [`Transform`] and [`FlyControlTag`] a fly
//! camera. The mouse directs which way "forward" is. However, y-axis movement
//! is not affected by location of the mouse, allowing the camera to rotate
//! freely in its local x-axis without affecting its direction.
//!
//! [`Camera`]: https://docs.amethyst.rs/stable/amethyst_rendy/struct.Camera.html
//! [`FlyControlTag`]: https://docs.amethyst.rs/stable/amethyst_controls/struct.FlyControlTag.html
//! [`System`]: https://docs.amethyst.rs/stable/specs/trait.System.html
//! [`Transform`]: https://docs.amethyst.rs/stable/amethyst_core/transform/components/struct.Transform.html

use amethyst::{
    controls::FlyControlTag,
    core::{
        math::{Unit, UnitQuaternion, Vector3},
        transform::Transform,
    },
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use lazy_static::lazy_static;

use crate::input::{IngameAxisBinding, IngameBindings};

/// The camera's movement speed in units per 1/60 second.
const MOVEMENT_SPEED: f32 = 0.05;

lazy_static! {
    static ref BACKWARD: Vector3<f32> = Vector3::z();
    static ref FORWARD: Vector3<f32> = -Vector3::z();
    static ref X_AXIS: Unit<Vector3<f32>> = Vector3::x_axis();
}

/// The struct implementing fly movement
pub(crate) struct FlyMovementSystem;

impl<'a> System<'a> for FlyMovementSystem {
    type SystemData = (
        Read<'a, InputHandler<IngameBindings>>,
        ReadStorage<'a, FlyControlTag>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (input, tag, mut transform): Self::SystemData) {
        for (transform, _) in (&mut transform, &tag).join() {
            let x = input.axis_value(&IngameAxisBinding::X).unwrap_or(0.0);
            let y = input.axis_value(&IngameAxisBinding::Y).unwrap_or(0.0);
            let z = input.axis_value(&IngameAxisBinding::Z).unwrap_or(0.0);

            if x == 0.0 && y == 0.0 && z == 0.0 {
                break;
            }

            let mut local_xz = transform.rotation() * Vector3::new(x, 0.0, -z);

            // Undo x rotation on local_xz
            let forward = transform.rotation() * *FORWARD;
            let flattened = Vector3::new(forward.x, 0.0, forward.z);
            let local_x_axis = transform.rotation() * Vector3::x_axis();
            let fixing_rotation = UnitQuaternion::from_axis_angle(
                &local_x_axis,
                if forward.y < 0.0 {
                    forward.angle(&flattened)
                } else {
                    -forward.angle(&flattened)
                },
            );
            local_xz = fixing_rotation * local_xz;

            transform.prepend_translation_along(
                Unit::new_normalize(Vector3::new(local_xz.x, y, local_xz.z)),
                MOVEMENT_SPEED,
            );
        }
    }
}
