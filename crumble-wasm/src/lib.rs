#[macro_use]
extern crate serde_derive;
use crumble::*;
use num_traits::cast::ToPrimitive;
use wasm_bindgen::prelude::*;

fn vec4_to_slice(v: F4) -> [f64; 4] {
	macro_rules! f {
		($i:expr) => {
			v[$i].to_f64().unwrap()
		};
	}
	[f!(0), f!(1), f!(2), f!(3)]
}
fn slice_to_vec4(v: [f64; 4]) -> F4 {
	macro_rules! f {
		($i:expr) => {
			F::from(v[$i])
		};
	}
	F4!(f!(0), f!(1), f!(2), f!(3))
}

#[wasm_bindgen]
pub struct PieceRef {
	piece: Rc<Piece>,
}
#[wasm_bindgen]
impl PieceRef {
	pub fn color(&self) -> bool {
		self.piece.color
	}
	pub fn aabb(&self) -> JsValue {
		JsValue::from_serde(&vec4_to_slice(self.piece.aabb)).unwrap()
	}
}
use std::rc::Rc;
impl From<Rc<Piece>> for PieceRef {
	fn from(piece: Rc<Piece>) -> Self {
		Self { piece }
	}
}

#[wasm_bindgen]
pub struct Instance {
	board: Board,
	selection: Option<[f64; 4]>,
}
#[wasm_bindgen]
impl Instance {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self {
			board: Board::new_starting(),
			selection: None,
		}
	}
	pub fn pieces(&self) -> JsValue {
		let pieces: Vec<PieceRef> = self
			.board
			.pieces
			.iter()
			.map(|piece| PieceRef::from(*piece))
			.collect();
		JsValue::from_serde(&pieces).unwrap()
	}
	// pub fn
	// TODO select
	// swap
	// split
	// join
}
