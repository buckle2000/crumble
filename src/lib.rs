mod math;
use math::*;

pub const BLACK: bool = false;
pub const WHITE: bool = true;

#[derive(Clone, PartialEq)]
pub struct Piece {
	pub aabb: F4,
	pub color: bool,
}
impl std::ops::Deref for Piece {
	type Target = F4;
	fn deref(&self) -> &Self::Target {
		&self.aabb
	}
}

macro_rules! of_same_color {
	($pieces:expr, $color:expr) => {
		$pieces.iter().any(|x| x.color == $color)
	};
}

macro_rules! total_area {
	($pieces:expr) => {
		$pieces
			.iter()
			.map(|piece| piece.area())
			.fold(F!(0), |sum, x| sum + x)
	};
}

use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Board {
	pieces: Vec<Rc<Piece>>,
}
impl Board {
	pub fn new_starting() -> Self {
		let mut board = Self::default();
		for x in 0..6 {
			for y in 0..6 {
				board.pieces.push(Rc::new(Piece {
					aabb: F4!(F!(x, 6), F!(y, 6), F!(1, 6), F!(1, 6)),
					color: (x + y) % 2 != 0,
				}));
			}
		}
		board
	}
	pub fn select(&self, selection: F4) -> Vec<Rc<Piece>> {
		self
			.pieces
			.iter()
			.filter(|piece| aabb_intersect(piece.aabb, selection))
			.map(|x| x.clone())
			.collect()
	}
	pub fn change(&mut self, to_remove: &Vec<Rc<Piece>>, to_add: Vec<Piece>) {
		self.pieces.retain(|x| !to_remove.contains(x));
		self.pieces.extend(to_add.into_iter().map(Rc::new));
	}
	/// Merge pieces
	/// assuming no overlap
	pub fn join(&mut self, selection: F4) -> Option<()> {
		let to_join = self.select(selection);
		let aabb = aabb(to_join.iter().map(|piece| piece.aabb));
		let color = to_join[0].color;
		if is_valid_shape(aabb) && of_same_color!(to_join, color) && total_area!(to_join) == aabb.area() {
			self.change(&to_join, vec![Piece { aabb, color }]);
			Some(())
		} else {
			None
		}
	}
	pub fn split(&mut self, path: F4) -> Option<()> {
		let to_split = self.select(path);
		let color = to_split[0].color;
		if !of_same_color!(to_split, color) {
			return None;
		}

		let splited: Vec<Piece> = to_split
			.iter()
			.map(|piece| try_split(&piece, path))
			.collect::<Option<Vec<Vec<Piece>>>>()? // fail first
			.into_iter()
			.flatten() // then flatten
			.collect();
		self.change(&to_split, splited);
		Some(())
	}
	// TODO swap
	// TODO capture
}

fn try_split(p: &Piece, path: F4) -> Option<Vec<Piece>> {
	let cut = aabb_clamp(path, **p);
	Some(vec![
		Piece {
			aabb: aabb_p01(p.p0(), cut.p1()),
			..*p
		},
		Piece {
			aabb: aabb_p01(p.p1(), cut.p0()),
			..*p
		},
	])
}

// Is this a valid piece?
pub fn is_valid_shape(rect: F4) -> bool {
	let (w, h) = (rect[2], rect[3]);
	w == h || w == h * F!(2) || w * F!(2) == h
}
