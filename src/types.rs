use std::collections::VecDeque;
use std::ops::Add;

/// A snake object. This is currently a type alias,
/// but that is likely to change in the future.
pub type Snake = VecDeque<CoordinateVector>;

/// A CoordinateVector represents a Euclidean vector
/// in 2D space. It is mainly used for directions
/// and positions. For example, the `travel` function
/// has a CoordinateVector input for the direction,
/// and the `head_touching_object` function has a
/// CoordinateVector input for the position of the object.
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CoordinateVector(pub i32,pub i32);
impl Add for CoordinateVector {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		CoordinateVector(self.0 + rhs.0, self.1 + rhs.1)
	}
}
