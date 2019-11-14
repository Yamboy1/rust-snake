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

	/// Moves the Snake one unit in self.direction
	///
	/// # Example
	/// ```
	/// use snake::{ model::Snake, Direction };
	///
	/// let mut snake = Snake::new((0,0), Direction::Right);
	/// snake.travel();
	/// println!("{:?}", snake.get_position()); // (-1, 0)
	/// ```
	pub fn travel(&mut self) {
		self.position = match self.direction {
			Direction::Up => (self.position.0, self.position.1-1),
			Direction::Left => (self.position.0-1, self.position.1),
			Direction::Down => (self.position.0, self.position.1+1),
			Direction::Right => (self.position.0+1, self.position.1),
		};
	}

	/// Gets the Snake's current position
	///
	/// # Example
	/// ```
	/// use snake::{ model::Snake, Direction };
	///
	/// let snake = Snake::new((0,0), Direction::Right);
	/// println!("{:?}", snake.get_position()); // (-1, 0)
	/// ```
	pub fn get_position(&self) -> Position {
		self.position
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
		let snake = Snake::new((-1,-1), Direction::Right);
		assert_eq!(snake.position, (-1,-1))
	}
	#[test]
	fn snake_travels_in_the_right_direction() {
		let mut snake = Snake::new((0,0), Direction::Right);
		snake.travel();
		assert_eq!(snake.position, (1,0));
		snake.direction = Direction::Down;
		snake.travel();
		assert_eq!(snake.position, (1,1));
		snake.direction = Direction::Left;
		snake.travel();
		assert_eq!(snake.position, (0,1));
		snake.direction = Direction::Up;
		snake.travel();
		assert_eq!(snake.position, (0,0));
	}
	#[test]
	fn get_position_gets_current_position() {
		let mut snake = Snake::new((0,0), Direction::Right);
		assert_eq!(snake.get_position(), snake.position);
		snake.position = (-1,566);
		assert_eq!(snake.get_position(), snake.position)
	}
}
