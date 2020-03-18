use amethyst::{
    core::transform::bundle::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        RenderingBundle,
        types::DefaultBackend,
    },
    utils::application_root_dir,
};

use crate::pong::Pong;
use crate::systems::{MoveBallSystem, PaddleSystem};

mod pong;
mod systems;

fn main() -> amethyst::Result<()>
{
    amethyst::start_logger(Default::default());
    let root_dir = application_root_dir()?;
    let config_dir = root_dir.join("config");
    let display_config_path = config_dir.join("display.ron");
    let assets_dir = root_dir.join("assets");

    let game_data_builder = GameDataBuilder::new()
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(config_dir.join("bindings.ron"))?)?
        .with(PaddleSystem, "paddle_system", &["input_system"])
        .with(MoveBallSystem, "ball_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_config_path).with_clear([0.00196, 0.23726, 0.21765, 1.0]))
            .with_plugin(RenderFlat2D::default()))?;

    let mut game = Application::new(assets_dir, Pong, game_data_builder)?;
    game.run();

    Ok(())
}
