use crate::{Position, Direction};

/// A struct for a snake character. This contains all of the functionality of the snake.
pub struct Snake {
	position: Position,
	direction: Direction
}

impl Snake {
	/// Instantiates a new Snake with starting position and direction.
	///
	/// # Example
	/// ```
	/// use snake::{ model::Snake, Direction };
	///
	/// Snake::new((0,0), Direction::Right);
	/// ```
	pub fn new(position: Position, direction: Direction) -> Self {
		Snake { position, direction }
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn new_should_set_fields() {
		let snake = Snake::new((0,0), Direction::Right);
		assert_eq!(snake.position, (0,0));
		assert_eq!(snake.direction, Direction::Right);
	}
	#[test]
	fn position_can_be_negative() {
		let snake = Snake::new((-1, -1), Direction::Right);
	}
}
