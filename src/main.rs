use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        RenderingBundle,
        types::DefaultBackend,
    },
    utils::application_root_dir,
};

struct Pong;

impl SimpleState for Pong {}

fn main() -> amethyst::Result<()>
{
    amethyst::start_logger(Default::default());
    let root_dir = application_root_dir()?;
    let display_config_path = root_dir.join("config").join("display.ron");
    let assets_dir = root_dir.join("assets");

    let game_data_builder = GameDataBuilder::new()
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_config_path).with_clear([0.00196, 0.23726, 0.21765, 1.0]))
            .with_plugin(RenderFlat2D::default()))?;

    let mut game = Application::new(assets_dir, Pong, game_data_builder)?;
    game.run();

    Ok(())
}
