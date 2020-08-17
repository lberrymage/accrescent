//! The main gameplay state

use amethyst::{core::transform::Transform, prelude::*, renderer::Camera};

/// The struct implementing the main gameplay state
pub(crate) struct Ingame;

impl SimpleState for Ingame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world
            .create_entity()
            .with(Camera::standard_3d(500.0, 500.0))
            .with(Transform::default())
            .build();
    }
}
