//! The collection of all game [`System`]s
//!
//! [`System`]: https://docs.amethyst.rs/stable/specs/trait.System.html

mod camera_rotation;

pub(crate) use self::camera_rotation::CameraRotationSystemDesc;
