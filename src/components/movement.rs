use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::*;

use crate::resources::pathfinding::Pos;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Movement {
	pub current : Pos,
	pub destination : Option<Pos>,
	pub path : Option<VecDeque<Pos>>,
}

impl Movement {
	pub fn new(start_pos : Pos) -> Self {
		Movement {
			current: start_pos,
			destination: None,
			path: None,
		}
	}
}

impl Component for Movement {
	type Storage = VecStorage<Self>;
}
