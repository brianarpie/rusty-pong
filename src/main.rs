use amethyst::{
    prelude::*,
    core::transform::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod pong;
use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    // enable basic logging to console
    amethyst::start_logger(Default::default());

    // load display.ron to modify window size/title w/o recompiling
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // basic application setup
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it.
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities witha 'SpriteRender' component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?;


    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;

    game.run();
    
    Ok(())
}