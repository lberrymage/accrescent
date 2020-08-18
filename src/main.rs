//! Accrescent is an open-world, modular sandbox game built upon the [Amethyst
//! game engine][ge]. It aims to be mod-driven through its pubilc API, allowing
//! anyone to easily create and distribute personal modifications in source or
//! binary form.
//!
//! [ge]: https://amethyst.rs

mod input;
mod states;
mod systems;

use amethyst::{
    controls::{CursorHideSystemDesc, MouseFocusUpdateSystemDesc},
    core::transform::TransformBundle,
    input::InputBundle,
    renderer::{
        types::DefaultBackend,
        RenderDebugLines,
        RenderShaded3D,
        RenderSkybox,
        RenderToWindow,
        RenderingBundle,
    },
    utils,
    Application,
    GameDataBuilder,
    LoggerConfig,
};

use crate::{
    input::IngameBindings,
    states::Ingame,
    systems::{CameraRotationSystemDesc, FlyMovementSystem},
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());

    let app_root = utils::application_root_dir()?;
    let config_dir = app_root.join("config");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<IngameBindings>::new()
                .with_bindings_from_file(config_dir.join("ingame_bindings.ron"))?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderDebugLines::default())
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderSkybox::default())
                .with_plugin(RenderToWindow::from_config_path(
                    config_dir.join("display_config.ron"),
                )?),
        )?
        .with_bundle(TransformBundle::new())?
        .with(FlyMovementSystem, "fly_movement", &[])
        .with_system_desc(CameraRotationSystemDesc::default(), "camera_rotation", &[])
        .with_system_desc(MouseFocusUpdateSystemDesc::default(), "mouse_focus", &[])
        .with_system_desc(
            CursorHideSystemDesc::default(),
            "cursor_hide",
            &["mouse_focus"],
        );

    let mut game = Application::new(app_root.join("assets"), Ingame, game_data)?;
    game.run();

    Ok(())
}
