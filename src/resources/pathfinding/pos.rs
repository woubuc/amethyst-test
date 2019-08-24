use std::ops::{Add, Sub};
use amethyst::core::Transform;

/// A position on the tile grid
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Pos {
	pub x : isize,
	pub y : isize,
	pub z : isize,
}

impl Pos {
	pub fn as_transform(&self) -> Transform {
		let mut transform = Transform::default();
		transform.set_translation_xyz(self.x as f32 * 32.0, self.y as f32 * 32.0, self.z as f32 * 32.0);
		transform
	}
}

impl Add for Pos {
	type Output = Pos;
	
	fn add(self, other : Pos) -> Self::Output {
		Pos {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl Add<(isize, isize, isize)> for Pos {
	type Output = Pos;
	
	fn add(self, (x, y, z) : (isize, isize, isize)) -> Self::Output {
		Pos {
			x: self.x + x,
			y: self.y + y,
			z: self.z + z,
		}
	}
}

impl Sub for Pos {
	type Output = Pos;
	
	fn sub(self, other : Self) -> Self::Output {
		Pos {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl Sub<(isize, isize, isize)> for Pos {
	type Output = Pos;
	
	fn sub(self, (x, y, z) : (isize, isize, isize)) -> Self::Output {
		Pos {
			x: self.x - x,
			y: self.y - y,
			z: self.z - z,
		}
	}
}
