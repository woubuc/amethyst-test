use std::mem;

use amethyst::core::Transform;
use amethyst::renderer::camera::Camera;
use amethyst::ecs::{Entity, World};

pub struct Display {
	pub camera : Entity,
	
	pub z_level : isize,
	
	pub window_width : f32,
	pub window_height : f32,
	pub window_size_changed : bool,
	
	pub camera_x : f32,
	pub camera_y : f32,
	pub camera_position_changed : bool,
}

impl Display {
	pub fn new(camera : Entity) -> Self {
		Display {
			camera,
			
			z_level: 0,
			
			window_width: 0.0,
			window_height: 0.0,
			window_size_changed: false,
			
			camera_x: 0.0,
			camera_y: 0.0,
			camera_position_changed: false,
		}
	}
	
	pub fn update(&mut self) {
		self.camera_position_changed = false;
		self.window_size_changed = false;
	}
	
	pub fn set_camera_position(&mut self, position : &Transform) {
		let translation = position.translation();
		self.camera_x = translation.x;
		self.camera_y = translation.y;
		self.camera_position_changed = true;
	}
	
	pub fn set_screen_size(world : &World, width : f32, height : f32) {
		let mut display = world.write_resource::<Display>();
		let mut cameras = world.write_storage::<Camera>();
		
		display.window_width = width;
		display.window_height = height;
		display.window_size_changed = true;
		cameras.insert(display.camera, Camera::standard_2d(width, height));
	}
}
