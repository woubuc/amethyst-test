use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture};
use amethyst::utils::fps_counter::FpsCounter;
use amethyst::window::ScreenDimensions;
use rand::{Rng, thread_rng};
use winit::{Event, WindowEvent};

use crate::components;
use crate::components::grid::Floor;
use crate::components::Movement;
use crate::resources;
use crate::resources::Display;
use crate::resources::pathfinding::{Pathfinder, Pos};
use crate::systems;

pub struct GameState;

impl SimpleState for GameState {
	fn on_start(&mut self, data : StateData<'_, GameData<'_, '_>>) {
		let world = data.world;
		
		let camera = create_camera(world);
		
		world.add_resource(Pathfinder::new(100, 100, 1));
		world.add_resource(Display::new(camera));
		world.register::<Floor>();
		
		let sprite = load_test(world);
		
		for x in 0..100 {
			for y in 0..100 {
				let mut transform = Transform::default();
				transform.set_translation_xyz(x as f32 * 32.0, y as f32 * 32.0, -1.0);
				
				world.create_entity()
					.with(Floor::default())
					.with(SpriteRender {
						sprite_sheet: sprite.clone(),
						sprite_number: 2,
					})
					.with(transform)
					.build();
			}
		}
		
		let mut rng = thread_rng();
		
		for i in 0..10 {
			let pos = Pos { x: rng.gen_range(0, 100), y: rng.gen_range(0, 100), z: 0 };
			world.create_entity()
				 .with(Movement::new(pos))
				 .with(SpriteRender {
					 sprite_sheet: sprite.clone(),
					 sprite_number: 0,
				 })
				 .with(pos.as_transform())
				 .build();
		}
		
	}
	
	fn update(&mut self, data : &mut StateData<GameData>) -> SimpleTrans {
		
		let mut display = data.world.write_resource::<Display>();
		display.update();
		
		Trans::None
	}
	
	fn handle_event(&mut self, data : StateData<'_, GameData<'_, '_>>, event : StateEvent) -> SimpleTrans {
		
		if let StateEvent::Window(event) = event {
			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => return Trans::Quit,
					WindowEvent::Resized(size) => Display::set_screen_size(data.world, size.width as f32, size.height as f32),
					_ => (),
				},
				_ => (),
			}
		}

		Trans::None
	}
}

fn create_camera(world: &mut World) -> Entity {
	let mut transform = Transform::default();
	transform.set_translation_xyz( 0.0, 0.0, 1.0);

	let (screen_width, screen_height) = {
		let dim = world.read_resource::<ScreenDimensions>();
		(dim.width(), dim.height())
	};
	
	world.create_entity()
		.with(Camera::standard_2d(screen_width, screen_height))
		.with(transform)
		.build()
}

fn load_test(world : &mut World) -> Handle<SpriteSheet> {
	let texture_handle = {
		let loader = world.read_resource::<Loader>();
		let storage = world.read_resource::<AssetStorage<Texture>>();
		loader.load("test.png", ImageFormat::default(), (), &storage)
	};
	
	let loader = world.read_resource::<Loader>();
	let storage = world.read_resource::<AssetStorage<SpriteSheet>>();
	loader.load("test.ron", SpriteSheetFormat(texture_handle), (), &storage)
}
