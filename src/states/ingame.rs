//! The main gameplay state

use amethyst::{
    controls::FlyControlTag,
    core::{
        math::{Point3, Translation3, UnitQuaternion, Vector3},
        transform::Transform,
    },
    prelude::*,
    renderer::{debug_drawing::DebugLinesComponent, palette::Srgba, Camera},
};
use lazy_static::lazy_static;
use static_assertions::const_assert_eq;

/// The width in units of debug grid. This number _must_ be divisible by 2.
const GRID_WIDTH: i16 = 16;
/// The depth in units of debug grid. This number _must_ be divisible by 2.
const GRID_DEPTH: i16 = 16;

const_assert_eq!(GRID_WIDTH % 2, 0);
const_assert_eq!(GRID_DEPTH % 2, 0);

lazy_static! {
    static ref GRID_COLOR: Srgba = Srgba::new(0.0, 0.4, 0.0, 1.0);
    static ref GRID_WIDTH_VEC: Vector3<f32> = Vector3::new(GRID_WIDTH.into(), 0.0, 0.0);
    static ref GRID_DEPTH_VEC: Vector3<f32> = Vector3::new(0.0, 0.0, GRID_DEPTH.into());
}

/// The struct implementing the main gameplay state
pub(crate) struct Ingame;

impl SimpleState for Ingame {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Create debug lines for testing camera movement. This will exist temporarily
        // until world generation is implemented.
        let mut debug_lines_component = DebugLinesComponent::with_capacity(256);

        // Lines along the y axis
        for x in (-GRID_WIDTH / 2)..=(GRID_WIDTH / 2) {
            debug_lines_component.add_direction(
                Point3::new(x.into(), 0.0, (-GRID_DEPTH / 2).into()),
                *GRID_DEPTH_VEC,
                *GRID_COLOR,
            );
        }

        // Lines along the x axis
        for z in (-GRID_DEPTH / 2)..=(GRID_DEPTH / 2) {
            debug_lines_component.add_direction(
                Point3::new((-GRID_WIDTH / 2).into(), 0.0, z.into()),
                *GRID_WIDTH_VEC,
                *GRID_COLOR,
            );
        }

        // "Forward" marker
        debug_lines_component.add_direction(
            Point3::new(0.0, 0.0, (-GRID_DEPTH / 2).into()),
            Vector3::new(0.0, 3.0, 0.0),
            *GRID_COLOR,
        );

        data.world
            .create_entity()
            .with(debug_lines_component)
            .build();

        data.world
            .create_entity()
            .with(Camera::standard_3d(500.0, 500.0))
            .with(FlyControlTag)
            .with(Transform::new(
                Translation3::new(0.0, 1.5, 0.0),
                UnitQuaternion::identity(),
                Vector3::from_element(1.0),
            ))
            .build();
    }
}
