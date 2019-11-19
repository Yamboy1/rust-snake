use crate::types::{Snake, CoordinateVector};

/* MOVEMENT */

pub fn travel(snake: &mut Snake, direction: CoordinateVector, grow: bool) -> CoordinateVector {
	let &old_head = snake.back().unwrap();
	if !grow { snake.pop_front().unwrap(); }
	let new_head = old_head + direction;
	snake.push_back(old_head + direction);
	new_head
}

/* CONDITIONS */

pub fn head_touching_object(snake: &Snake, object: CoordinateVector) -> bool {
	*snake.back().unwrap() == object
}

pub fn snake_touching_head(snake: &Snake) -> bool {
	let &head = snake.back().unwrap();
	let position = snake.iter()
		.position(|&coord| coord == head).unwrap();
	// Return true if the found position is not the head.
	position < snake.len() - 1
}

pub fn head_out_of_bounds(snake: &Snake, bounds: CoordinateVector) -> bool {
	let &head = snake.back().unwrap();
	head.0 >= bounds.0 || head.1 >= bounds.1 || head.0 < 0 || head.1 < 0
}
