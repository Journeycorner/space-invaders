mod space_invaders;
mod systems;

use crate::space_invaders::SpaceInvaders;
use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // display
    let display_path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&display_path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            // black
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    // input
    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(&binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(
            systems::SpacecraftSystem,
            "spacecraft_sytem",
            &["input_system"],
        )
        .with(systems::AliensSystem { x_moves: 0 }, "aliens_system", &[]);

    Application::new("./", SpaceInvaders, game_data)?.run();
    Ok(())
}
