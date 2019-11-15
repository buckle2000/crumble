mod math;
use math::*;

type Color = bool;
pub const BLACK: Color = false;
pub const WHITE: Color = true;

#[derive(Clone, PartialEq)]
pub struct Piece {
	pub aabb: F4,
	pub color: Color,
}
impl Piece {
	pub fn check(self) -> Option<Rc<Self>> {
		if is_valid_shape(*self) {
			Some(Rc::new(self))
		} else {
			None
		}
	}
	pub fn flip(&self) -> Self {
		Self {
			color: !self.color,
			..self.clone()
		}
	}
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

pub type ChangeResult = Option<()>; // TODO maybe improve error types

use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Board {
	pieces: Vec<Rc<Piece>>,
	// TODO what if we clone a board? would the pieces be cloned or merely referenced?
	// ideally, the pieces should be cloned as well
}
impl Board {
	pub fn new_starting() -> Self {
		let mut board = Self::default();
		for x in 0..6 {
			for y in 0..6 {
				board.pieces.push(
					Piece {
						aabb: F4!(F!(x, 6), F!(y, 6), F!(1, 6), F!(1, 6)),
						color: (x + y) % 2 != 0,
					}
					.check()
					.unwrap(),
				);
			}
		}
		board
	}
	pub fn select(&self, selection: F4) -> Vec<Rc<Piece>> {
		self
			.pieces
			.iter()
			.filter(|piece| aabb_intersect(piece.aabb, selection))
			.cloned()
			.collect()
	}
	pub fn select_one(&self, point: F2) -> Option<Rc<Piece>> {
		self
			.pieces
			.iter()
			.find(|piece| aabb_contains(piece.aabb, point))
			.cloned()
	}
	pub fn change(&mut self, to_remove: &Vec<Rc<Piece>>, to_add: &Vec<Rc<Piece>>) {
		self.pieces.retain(|x| !to_remove.contains(x));
		self.pieces.extend(to_add.to_owned());
	}
	/// Merge pieces
	/// assuming no overlap
	pub fn join(&mut self, color: Color, selection: F4) -> Option<Rc<Piece>> {
		let to_join = self.select(selection);
		let aabb = aabb(to_join.iter().map(|piece| piece.aabb));
		if of_same_color!(to_join, color) && total_area!(to_join) == aabb.area() {
			let piece = Piece { aabb, color }.check()?;
			self.change(&to_join, &vec![piece.clone()]);
			Some(piece)
		} else {
			None
		}
	}
	pub fn split(&mut self, color: Color, path: F4) -> Option<Vec<Rc<Piece>>> {
		let to_split = self.select(path);
		if !of_same_color!(to_split, color) {
			return None;
		}
		let splited: Vec<Rc<Piece>> = to_split
			.iter()
			.map(|piece| try_split(&piece, path))
			.collect::<Option<Vec<Vec<Rc<Piece>>>>>()? // fail if any one of the split fails
			.into_iter()
			.flatten()
			.collect();
		self.change(&to_split, &splited);
		Some(splited)
	}
	/// Swap two pieces
	/// `from` must be your piece
	pub fn swap(&mut self, color: Color, from: Rc<Piece>, to: Rc<Piece>) -> Option<Rc<Piece>> {
		if from.color == color && aabb_share_edge(from.aabb, to.aabb) {
			let dst = to.flip().check()?;
			let to_add = vec![from.flip().check()?, dst.clone()];
			self.change(&vec![from, to], &to_add);
			if self
				.capture_check()
				.iter()
				.any(|captured| captured.from == dst)
			{
				None
			} else {
				Some(dst)
			}
		} else {
			None
		}
	}
	/// Capture pieces
	fn capture_check(&mut self) -> Vec<CapturedPiece> {
		unimplemented!()
	}
}

struct CapturedPiece {
	from: Rc<Piece>,
	to: Rc<Piece>,
}

fn try_split(p: &Piece, path: F4) -> Option<Vec<Rc<Piece>>> {
	let cut = aabb_clamp(path, p.aabb);
	Some(vec![
		Piece {
			aabb: aabb_p01(p.p0(), cut.p1()),
			..*p
		}
		.check()?,
		Piece {
			aabb: aabb_p01(p.p1(), cut.p0()),
			..*p
		}
		.check()?,
	])
}
