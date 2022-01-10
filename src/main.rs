use amethyst::{
    input::{InputBundle,StringBindings},
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod catvolleyball;
use crate::catvolleyball::CatVolleyball;

mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let binding_path = app_root.join("resources")
                .join("bindings_config.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
                .with_bindings_from_file(binding_path)?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[]);


    let mut game = Application::new(assets_dir, CatVolleyball, game_data)?;
    game.run();

    Ok(())
}
