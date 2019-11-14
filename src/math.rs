pub type F = fraction::GenericFraction<i8>;
// type F2 = cgmath::Vector2<F>;
// type F4 = cgmath::Vector4<F>;
pub type F2 = nalgebra::Vector2<F>;
pub type F4 = nalgebra::Vector4<F>;

#[macro_export]
macro_rules! F {
	($x:expr)      => { F::new($x,1) };
	($($x:expr),*) => { F::new($($x),*) };
}
#[macro_export]
macro_rules! F2 { ($($x:expr),*) => { F2::new($($x),*) }; }
#[macro_export]
macro_rules! F4 { ($($x:expr),*) => { F4::new($($x),*) }; }

pub trait AABBExt {
	// upper left corner
	fn p0(&self) -> F2;
	fn size(&self) -> F2;
	// lower right corner
	fn p1(&self) -> F2;
	fn area(&self) -> F;
}
impl AABBExt for F4 {
	fn p0(&self) -> F2 {
		F2!(self[0], self[1])
	}
	fn size(&self) -> F2 {
		F2!(self[4], self[3])
	}
	fn p1(&self) -> F2 {
		self.p0() + self.size()
	}
	fn area(&self) -> F {
		let size = self.size();
		size[0] * size[1]
	}
}

use partial_min_max::{max, min};

/// REMEMBER: THE ORDER IS X Y WIDTH HEIGHT

// Calculate the AABB of given rectangles
pub fn aabb(rects: impl Iterator<Item = F4>) -> F4 {
	let (p0, p1) = rects.fold(
		(
			F2!(F::infinity(), F::infinity()),
			F2!(F::neg_infinity(), F::neg_infinity()),
		),
		|(opt0, opt1), x| {
			let x1 = x.p1();
			(
				F2!(min(opt0[0], x[0]), min(opt0[1], x[1])),
				F2!(max(opt1[0], x1[0]), max(opt1[1], x1[1])),
			)
		},
	);
	aabb_p01(p0, p1)
}

// Clamp src to dst
pub fn aabb_clamp(src: F4, dst: F4) -> F4 {
	let sr1 = src.p1();
	let ds1 = dst.p1();
	aabb_p01(
		F2!(max(src[0], dst[0]), max(src[1], dst[1])),
		F2!(min(sr1[0], ds1[0]), min(sr1[1], ds1[1])),
	)
}

// Create bounding box from two corners
pub fn aabb_p01(p0: F2, p1: F2) -> F4 {
	let size = p1 - p0;
	F4!(p0[0], p0[1], size[0], size[1])
}

pub fn aabb_intersect(lhs: F4, rhs: F4) -> bool {
	lhs[0] < rhs[2] && lhs[1] < rhs[3] && rhs[0] < lhs[2] && rhs[1] < lhs[3]
}

pub fn aabb_touch() -> bool {
	unimplemented!()
}

// if they can swap
pub fn aabb_share_edge() {
	unimplemented!()
}
