//! The [`System`] responsible for camera rotation
//!
//! Causes any given [`Camera`] with a [`Transform`] to rotate with the mouse.
//! Rotation is clamped between -90 and 90 degrees along the local X axis, and
//! will not occur at all if the game window is not in focus.
//!
//! [`Camera`]: https://docs.amethyst.rs/stable/amethyst_rendy/struct.Camera.html
//! [`System`]: https://docs.amethyst.rs/stable/specs/trait.System.html
//! [`Transform`]: https://docs.amethyst.rs/stable/amethyst_core/transform/components/struct.Transform.html

use amethyst::{
    controls::{HideCursor, WindowFocus},
    core::{
        math::{self, Vector3},
        transform::Transform,
    },
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::Camera,
    shrev::{EventChannel, ReaderId},
    winit::{DeviceEvent, Event},
};
use derive_new::new;
use lazy_static::lazy_static;

/// The mouse sensitivity
const SENSITIVITY: f32 = 0.25;

lazy_static! {
    static ref DOWN: Vector3<f32> = -Vector3::y();
    static ref FORWARD: Vector3<f32> = -Vector3::z();
    static ref UP: Vector3<f32> = Vector3::y();
}

/// The struct implementing camera rotation
#[derive(new, SystemDesc)]
#[system_desc(name(CameraRotationSystemDesc))]
pub(crate) struct CameraRotationSystem {
    /// Default event reader
    #[system_desc(event_channel_reader)]
    event_reader: ReaderId<Event>,
}

impl<'a> System<'a> for CameraRotationSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, EventChannel<Event>>,
        Read<'a, HideCursor>,
        Read<'a, WindowFocus>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Transform>,
    );

    #[allow(clippy::as_conversions)]
    fn run(&mut self, (events, hide, focus, camera, mut transform): Self::SystemData) {
        for event in events.read(&mut self.event_reader) {
            if focus.is_focused && hide.hide {
                if let Event::DeviceEvent { ref event, .. } = *event {
                    if let DeviceEvent::MouseMotion { delta: (x, y) } = *event {
                        for (transform, _) in (&mut transform, &camera).join() {
                            let forward = transform.rotation() * *FORWARD;

                            // Treat `MouseMotion` delta as rotation in degrees
                            transform.append_rotation_x_axis(-math::clamp(
                                y.to_radians() as f32 * SENSITIVITY,
                                -forward.angle(&UP),
                                forward.angle(&DOWN),
                            ));
                            transform
                                .prepend_rotation_y_axis((-x as f32).to_radians() * SENSITIVITY);
                        }
                    }
                }
            }
        }
    }
}
