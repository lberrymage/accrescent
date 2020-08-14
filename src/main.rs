//! Accrescent is an open-world, modular sandbox game built upon the [Amethyst
//! game engine][ge]. It aims to be mod-driven through its pubilc API, allowing
//! anyone to easily create and distribute personal modifications in source or
//! binary form.
//!
//! [ge]: https://amethyst.rs

mod states;

use amethyst::{
    controls::{CursorHideSystemDesc, MouseFocusUpdateSystemDesc},
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        types::DefaultBackend,
        RenderShaded3D,
        RenderSkybox,
        RenderToWindow,
        RenderingBundle,
    },
    utils,
    LoggerConfig,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());

    let app_root = utils::application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderSkybox::default())
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?),
        )?
        .with_bundle(TransformBundle::new())?
        .with_system_desc(MouseFocusUpdateSystemDesc::default(), "mouse_focus", &[])
        .with_system_desc(
            CursorHideSystemDesc::default(),
            "cursor_hide",
            &["mouse_focus"],
        );

    let mut game = Application::new(assets_dir, states::Ingame, game_data)?;
    game.run();

    Ok(())
}
