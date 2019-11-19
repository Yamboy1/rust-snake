use std::collections::VecDeque;
use std::ops::Add;

pub type Snake = VecDeque<CoordinateVector>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CoordinateVector(pub i32,pub i32);
impl Add for CoordinateVector {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		CoordinateVector(self.0 + rhs.0, self.1 + rhs.1)
	}
}
