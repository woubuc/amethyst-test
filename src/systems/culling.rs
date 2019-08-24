use amethyst::core::{Hidden, Transform};
use amethyst::ecs::{Join, Read, Entities, ReadExpect, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::camera::Camera;
use amethyst::renderer::SpriteRender;

use crate::resources::Display;

/// The culling system will apply the `Hidden` tag to all tiles
/// outside of the camera's viewport, to prevent them from being
/// rendered
///
/// Culling the invisible tiles drastically improves rendering
/// performance, especially on large tile maps.
#[derive(Debug, Default)]
pub struct CullingSystem {
	width: f32,
	height: f32,
}

/// Size in pixels to keep drawn outside of the camera viewport
///
/// This should be at least the size of one tile, since screen
/// coordinates of tiles are for the corner of a tile.
const BORDER_SIZE : f32 = 64.0;

impl<'s> System<'s> for CullingSystem {
	type SystemData = (
		Entities<'s>,
		WriteStorage<'s, Hidden>,
		ReadStorage<'s, Camera>,
		ReadStorage<'s, SpriteRender>,
		ReadStorage<'s, Transform>,
		ReadExpect<'s, Display>,
	);
	
	fn run(&mut self, (entities, mut hidden, cameras, renders, transforms, display): Self::SystemData) {
		use rayon::prelude::*;
		use amethyst::ecs::ParJoin;
		
		// We should only update culling tags if the camera has moved
		// or the window size has changed, otherwise we're just wasting
		// performance
		if display.window_width == self.width && display.window_height == self.width && !display.camera_position_changed {
			return;
		}
		
		#[cfg(feature = "profiling")]
		let t = std::time::Instant::now();
		
		self.width = display.window_width;
		self.height = display.window_height;
		
		let (min_x, max_x, min_y, max_y) = {
			let w = self.width / 2.0;
			let h = self.height / 2.0;
			
			(display.camera_x - w - BORDER_SIZE,
			 display.camera_x + w + BORDER_SIZE,
			 display.camera_y - h - BORDER_SIZE,
			 display.camera_y + h + BORDER_SIZE)
		};
		
		for (entity, _render, transform) in (&entities, &renders, &transforms).join() {
		
			let pos = transform.translation();
			
			if pos.x < min_x || pos.x > max_x || pos.y < min_y || pos.y > max_y {
				hidden.insert(entity, Hidden);
			} else {
				hidden.remove(entity);
			}
			
		}
		
		#[cfg(feature = "profiling")]
		println!("Culling: {:.2?}", t.elapsed());
		
	}
}
