use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteExpect, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::camera::Camera;

use crate::resources::Display;

pub struct CameraControlSystem;

const CAMERA_SPEED : f32 = 8.0;

impl<'s> System<'s> for CameraControlSystem {
	type SystemData = (
		Entities<'s>,
		WriteStorage<'s, Transform>,
		WriteStorage<'s, Camera>,
		Read<'s, InputHandler<StringBindings>>,
		WriteExpect<'s, Display>,
	);
	
	fn run(&mut self, (entities, mut transforms, mut cameras, input, mut display): Self::SystemData) {
		
		for (entity, transform, camera) in (&entities, &mut transforms, &cameras).join() {
			
			let mut update = false;
			
			if let Some(horizontal) = input.axis_value("move_horizontal") {
				if horizontal != 0.0 {
					transform.append_translation_xyz(horizontal * CAMERA_SPEED, 0.0, 0.0);
					update = true;
				}
			}
			
			if let Some(vertical) = input.axis_value("move_vertical") {
				if vertical != 0.0 {
					transform.append_translation_xyz(0.0, vertical * CAMERA_SPEED, 0.0);
					update = true;
				}
			}
			
			if update {
				display.set_camera_position(transform);
			}
		}
		
	}
}
