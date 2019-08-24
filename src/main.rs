use std::path::PathBuf;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::RenderingBundle;
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::input::{InputBundle, StringBindings};

mod components;
mod resources;
mod states;
mod systems;

use crate::systems::*;

const CLEAR_COLOUR : [f32; 4] = [0.2, 0.3, 0.4, 1.0];

fn main() -> amethyst::Result<()> {
	println!("Starting...");
    amethyst::start_logger(Default::default());
	
	let app_root = amethyst::utils::application_root_dir()?;
	let config_dir = app_root.join("config");

	println!("Building game data...");
    let game_data = GameDataBuilder::default()
        .with_bundle(create_rendering_bundle(&config_dir)?)?
		.with_bundle(create_input_bundle(&config_dir)?)?
        .with_bundle(TransformBundle::new())?
		
		.with(CameraControlSystem, "camera_control_system", &["input_system"])
		.with(CullingSystem::default(), "culling_system", &["camera_control_system"])
		
		.with(MovementSystem, "movement_roaming_system", &[])
		;

	println!("Initialising game...");
    let mut game = Application::new("assets/", states::GameState, game_data)?;
    game.run();

    Ok(())
}

fn create_rendering_bundle(config_dir : &PathBuf) -> amethyst::Result<RenderingBundle<DefaultBackend>> {
	let display_config = config_dir.join("display.ron");

	Ok(RenderingBundle::<DefaultBackend>::new()
		.with_plugin(RenderToWindow::from_config_path(display_config).with_clear(CLEAR_COLOUR))
		.with_plugin(RenderFlat2D::default()))
}

fn create_input_bundle(config_dir : &PathBuf) -> amethyst::Result<InputBundle<StringBindings>> {
	let binding_config = config_dir.join("bindings.ron");
	
	Ok(InputBundle::<StringBindings>::new().with_bindings_from_file(binding_config)?)
}
