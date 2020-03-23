use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::transform::bundle::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        RenderingBundle,
        types::DefaultBackend,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::audio::Music;
use crate::pong::Pong;
use crate::systems::{BounceBallSystem, MoveBallSystem, PaddleSystem, WinnerSystem};

mod pong;
mod systems;
mod audio;

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
        .with(MoveBallSystem, "move_ball_system", &[])
        .with(BounceBallSystem, "bounce_ball_system", &["paddle_system", "move_ball_system"])
        .with(WinnerSystem, "winner_system", &["bounce_ball_system"])
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(RenderToWindow::from_config_path(display_config_path).with_clear([0.00196, 0.23726, 0.21765, 1.0]))
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()))?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(DjSystemDesc::new(|music: &mut Music| music.music.next()), "dj_system", &[]);

    let mut game = Application::new(assets_dir, Pong::default(), game_data_builder)?;
    game.run();

    Ok(())
}
