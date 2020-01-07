#[macro_use]
extern crate serde_derive;
use crumble::*;
use num_traits::cast::ToPrimitive;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
// fn to_f64_2(x: F2) -> [f64; 2] {
// 	[x[0].to_f64().unwrap(), x[1].to_f64().unwrap()]
// }
fn to_f64_4(x: F4) -> [f64; 4] {
	[
		x[0].to_f64().unwrap(),
		x[1].to_f64().unwrap(),
		x[2].to_f64().unwrap(),
		x[3].to_f64().unwrap(),
	]
}
fn from_f64_2(x: [f64; 2]) -> F2 {
	F2!(x[0].into(), x[1].into())
}
fn from_f64_4(x: [f64; 4]) -> F4 {
	F4!(x[0].into(), x[1].into(), x[2].into(), x[3].into())
}

#[derive(Serialize)]
pub struct PieceRO {
	aabb: [f64; 4],
	color: Color,
}
impl PieceRO {
	fn new(p: &Piece) -> Self {
		Self {
			aabb: to_f64_4(p.aabb),
			color: p.color,
		}
	}
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct BoardW(Board);
#[wasm_bindgen]
impl BoardW {
	pub fn new_starting() -> Self {
		Self(Board::new_starting())
	}
	pub fn select(&self, arg0: JsValue) -> JsValue {
		let arg0: [f64; 4] = from_value(arg0).unwrap();
		let result: Vec<PieceRO> = self
			.0
			.select(from_f64_4(arg0))
			.iter()
			.map(|x| PieceRO::new(x))
			.collect();
		to_value(&result).unwrap()
	}
	pub fn select_one(&self, arg0: JsValue) -> JsValue {
		let arg0: [f64; 2] = from_value(arg0).unwrap();
		let result: Option<PieceRO> = self
			.0
			.select_one(from_f64_2(arg0))
			.and_then(|x| Some(PieceRO::new(&*x)));
		to_value(&result).unwrap()
	}
	pub fn join(&mut self, color: Color, arg0: JsValue) -> JsValue {
		let arg0: [f64; 4] = from_value(arg0).unwrap();
		let result: Option<PieceRO> = self
			.0
			.join(color, from_f64_4(arg0))
			.and_then(|x| Some(PieceRO::new(&*x)));
		to_value(&result).unwrap()
	}
	pub fn split(&mut self, color: Color, arg0: JsValue) -> JsValue {
		let arg0: [f64; 4] = from_value(arg0).unwrap();
		let result: Option<Vec<PieceRO>> = self
			.0
			.split(color, from_f64_4(arg0))
			.and_then(|v| Some(v.iter().map(|x| PieceRO::new(x)).collect()));
		to_value(&result).unwrap()
	}
	fn swap_helper(&mut self, color: Color, arg0: JsValue, arg1: JsValue) -> Option<PieceRO> {
		let arg0: [f64; 2] = from_value(arg0).unwrap();
		let arg1: [f64; 2] = from_value(arg1).unwrap();
		let arg0 = self.0.select_one(from_f64_2(arg0))?;
		let arg1 = self.0.select_one(from_f64_2(arg1))?;
		self
			.0
			.swap(color, arg0, arg1)
			.and_then(|x| Some(PieceRO::new(&*x)))
	}
	pub fn swap(&mut self, color: Color, arg0: JsValue, arg1: JsValue) -> JsValue {
		let result = self.swap_helper(color, arg0, arg1);
		to_value(&result).unwrap()
	}
}
