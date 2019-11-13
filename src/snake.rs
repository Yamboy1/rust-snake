use crate::util::{Coord, Direction};

struct Snake {
	position: Coord,
	direction: Direction
}

impl Snake {
	fn new(position: Coord, direction: Direction) -> Self {
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
}