use std::collections::HashMap;

use pathfinding::prelude::astar;

pub use self::pos::Pos;

mod pos;

#[derive(Debug, Default)]
pub struct Pathfinder {
	tiles : HashMap<Pos, u32>,
}

impl Pathfinder {
	pub fn new(size_x : isize, size_y : isize, size_z : isize) -> Self {
		let mut tiles = HashMap::new();
		
		for x in 0..size_x {
			for y in 0..size_y {
				for z in 0..size_z {
					let pos = Pos { x, y, z };
					tiles.insert(pos, 1);
				}
			}
		}
		
		Pathfinder { tiles }
	}
	
	pub fn set_tile(&mut self, pos : Pos, cost : u32) {
		self.tiles.insert(pos, cost);
	}
	
	pub fn remove_tile(&mut self, pos : Pos) {
		self.tiles.remove(&pos);
	}
	
	pub fn find_path(&self, start : Pos, target : Pos) -> Option<Vec<Pos>> {
		let result = astar(
			&start,
			|current| self.get_tile_connections(current),
			|current| self.calculate_heuristic(current, &target),
			|current| current == &target,
		);
		
		match result {
			Some((path, cost)) => Some(path),
			_ => None,
		}
	}
	
	fn calculate_heuristic(&self, current : &Pos, target : &Pos) -> u32 {
		(target.x - current.x).abs() as u32 + (target.y - current.y).abs() as u32 + (target.z - current.z).abs() as u32
	}
	
	fn get_tile_connections(&self, pos : &Pos) -> Vec<(Pos, u32)> {
		
		let tile = self.tiles.get(pos);
		if tile.is_none() { return Vec::new() }
		
		let tile = tile.unwrap();
		let pos = pos;
		
		let mut connections : Vec<(Pos, u32)> = Vec::new();
		
		// -1 but +2 because range is exclusive upper bound (i.e. `0..2` = 0,1)
		for x in (pos.x - 1)..(pos.x + 2) {
			for y in (pos.y - 1)..(pos.y + 2) {
				for z in (pos.z - 1)..(pos.z + 2) {
					
					if x == pos.x && y == pos.y && z == pos.z { continue }
					
					let tile_pos = Pos { x, y, z };
					
					if let Some(cost) = self.tiles.get(&tile_pos) {
						connections.push((tile_pos, *cost));
					}
				}
			}
		}
		
		connections
	}
}
