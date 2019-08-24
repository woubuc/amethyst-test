use amethyst::renderer::camera::Camera;
use amethyst::core::Transform;
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ecs::{System, WriteStorage, ReadStorage, Read, Join};
use rand::{thread_rng, Rng};

use crate::resources::pathfinding::{Pathfinder, Pos};
use crate::components::Movement;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::time::Instant;

/// The movement system will process pathfinding and moving for all mobile entities
///
/// This implementation is currently __very WIP__.
pub struct MovementSystem;

/// The walk speed of a mobile entity
///
/// TODO people should have varying walk speeds, they're not robots
const MOVEMENT_SPEED: f32 = 3.0;

impl<'s> System<'s> for MovementSystem {
	type SystemData = (
		WriteStorage<'s, Movement>,
		WriteStorage<'s, Transform>,
		Read<'s, Pathfinder>
	);
	
	fn run(&mut self, (mut movements, mut transforms, pathfinder): Self::SystemData) {
		
		let mut rng = thread_rng();
		
		for (movement, transform) in (&mut movements, &mut transforms).join() {
			
			#[cfg(feature = "profiling")]
			let t = std::time::Instant::now();
			
			// Get actual entity position
			let (x, y, z) = {
				let current_translation = transform.translation();
				let x = current_translation.x;
				let y = current_translation.y;
				let z = current_translation.z;
				(x, y, z)
			};
			
			// Update current tile from transform
			movement.current = {
				let x = (x / 32.0).round() as isize;
				let y = (y / 32.0).round() as isize;
				let z = (z / 32.0).round() as isize;
				Pos { x, y, z }
			};
			
			let mut path = match std::mem::replace(&mut movement.path, None) {
				Some(path) => path,
				None => {
					let destination = match movement.destination {
						Some(destination) => {
							if movement.current == destination {
								let destination = Pos { x: rng.gen_range(0, 100), y: rng.gen_range(0, 100), z: 0 };
								movement.destination = Some(destination);
								destination
							} else {
								destination
							}
						},
						None => {
							let destination = Pos { x: rng.gen_range(0, 100), y: rng.gen_range(0, 100), z: 0 };
							movement.destination = Some(destination);
							destination
						}
					};
					
					let path = pathfinder.find_path(movement.current, destination);
					
					if let Some(path) = path {
						VecDeque::from_iter(path)
					} else {
						VecDeque::new()
					}
				}
			};
			
			let next = loop {
				let next = path.front();
				if next.is_none() { break None }
				
				if let Some(next) = path.front() {
					if next != &movement.current {
						break Some(next);
					}
					
					path.pop_front();
				}
			};
			
			if let Some(next) = next {
				let next = next.as_transform();
				let next = next.translation();
				
				fn clamp(num : f32) -> f32 {
					if num < -MOVEMENT_SPEED {
						-MOVEMENT_SPEED
					} else if num > MOVEMENT_SPEED {
						MOVEMENT_SPEED
					} else {
						num
					}
				}
				
				let diff_x = clamp(next.x - x);
				let diff_y = clamp(next.y - y);
				let diff_z = clamp(next.z - z);
				
				transform.append_translation_xyz(diff_x, diff_y, diff_z);
			}
			
			if path.len() > 0 {
				movement.path = Some(path);
			} else {
				movement.path = None;
			}
			
			#[cfg(feature = "profiling")]
			println!("Movement: {:.2?}", t.elapsed());
		}
		
	}
}
