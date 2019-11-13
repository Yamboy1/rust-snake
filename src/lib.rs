pub mod model;

/// A tuple of x and y coordinates.
pub type Position = (i32, i32);

/// An enum with 2D Directions
#[derive(PartialEq, Debug)]
pub enum Direction {
	Up, Right, Left, Down,
}

