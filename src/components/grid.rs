use amethyst::ecs::{ Component, VecStorage, FlaggedStorage };

use crate::resources::pathfinding::Pos;

#[derive(Debug)]
pub struct Floor {
	pub floor_type : usize,
}

impl Component for Floor {
	type Storage = VecStorage<Self>;
}

impl Default for Floor {
	fn default() -> Self {
		Floor { floor_type: 0 }
	}
}
