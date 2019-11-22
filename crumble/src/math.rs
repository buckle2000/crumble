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
	// lower right corner
	fn p1(&self) -> F2;
	// upper right
	fn p10(&self) -> F2;
	// lower left
	fn p01(&self) -> F2;
	fn size(&self) -> F2;
	fn area(&self) -> F;
}
impl AABBExt for F4 {
	fn p0(&self) -> F2 {
		F2!(self[0], self[1])
	}
	fn p1(&self) -> F2 {
		F2!(self[0]+self[2], self[1]+self[3])
	}
	fn p10(&self) -> F2 {
		F2!(self[0]+self[2], self[1])
	}
	fn p01(&self) -> F2 {
		F2!(self[0], self[1]+self[3])
	}
	fn size(&self) -> F2 {
		F2!(self[2], self[3])
	}
	fn area(&self) -> F {
		let size = self.size();
		size[0] * size[1]
	}
}

use partial_min_max::{max, min};

/// REMEMBER: THE ORDER IS X Y WIDTH HEIGHT

/// Calculate the AABB of given rectangles
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
	aabb_p0p1(p0, p1)
}

/// If rectangle contains a point
pub fn aabb_contains(r0: F4, point: F2) -> bool {
	let r1 = r0.p1();
	point[0] >= r0[0] && point[1] >= r0[1] && point[0] < r1[0] && point[1] < r1[1]
}

/// Clamp source to destination
pub fn aabb_clamp(s0: F4, d0: F4) -> F4 {
	let s1 = s0.p1();
	let d1 = d0.p1();
	aabb_p0p1(
		F2!(max(s0[0], d0[0]), max(s0[1], d0[1])),
		F2!(min(s1[0], d1[0]), min(s1[1], d1[1])),
	)
}

/// Create bounding box from two corners
pub fn aabb_p0p1(p0: F2, p1: F2) -> F4 {
	let size = p1 - p0;
	F4!(p0[0], p0[1], size[0], size[1])
}

/// If two rectangles intersect with each other
/// touching rectangles don't count as intersecting
pub fn aabb_intersect(l0: F4, r0: F4) -> bool {
	let (l1, r1) = (l0.p1(), r0.p1());
	l0[0] < r1[0] && l0[1] < r1[1] && r0[0] < l1[0] && r0[1] < l1[1]
}

pub fn aabb_touch(lhs: F4, rhs: F4) -> bool {
	let rhs_clamped = aabb_clamp(rhs, lhs);
	rhs.p0() == rhs_clamped.p0()
		|| rhs.p1() == rhs_clamped.p1()
		|| rhs.p01() == rhs_clamped.p01()
		|| rhs.p10() == rhs_clamped.p10()
}

pub fn aabb_aligned_edges(l0: F4, r0: F4) -> usize {
	let (l1, r1) = (l0.p1(), r0.p1());
	let l = [[l0[0], l1[0]], [l0[1], l1[1]]];
	let r = [[r0[0], r1[0]], [r0[1], r1[1]]];
	let mut aligned_edges = 0;
	for axis in 0..2 {
		for side_l in 0..2 {
			for side_r in 0..2 {
				if l[axis][side_l] == r[axis][side_r] {
					aligned_edges += 1;
				}
			}
		}
	}
	aligned_edges
}

/// if they can swap
/// aabb_share_edge(x,y) == aabb_share_edge(y,x)
pub fn aabb_share_edge(l0: F4, r0: F4) -> bool {
	let aligned_edges = aabb_aligned_edges(l0, r0);
	match aligned_edges {
		0 | 1 | 2 => false,
		3 => true,
		4 => panic!("The two pieces overlap!"),
		_ => panic!("Wat"),
	}
}

/// Is this a valid piece?
pub fn is_valid_shape(rect: F4) -> bool {
	let (w, h) = (rect[2], rect[3]);
	w == h || w == h * F!(2) || w * F!(2) == h
}
