use crate::types::{Snake, CoordinateVector};

/* MOVEMENT */

/// Moves the snake IN PLACE along the `direction` Vector.
/// This is also the function that makes the snake grow
/// by keeping the tail intact, whilst adding onto the head
///
/// The typical usage of the grow ability would be
/// to check if the snake head is touching food,
/// and if so, you would set grow to true, and in the next
/// frame, the snake will grow.
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use snake::types::CoordinateVector;
/// use snake::core::travel;
///
/// let mut snake = VecDeque::from(vec![CoordinateVector(0,1)]);
/// travel(&mut snake, CoordinateVector(0,1), false);
/// assert_eq!(&snake, &VecDeque::from(vec![CoordinateVector(0,2)]));
///
/// let mut snake = VecDeque::from(vec![CoordinateVector(0,1)]);
/// travel(&mut snake, CoordinateVector(0,1), true);
/// assert_eq!(&snake, &VecDeque::from(vec![CoordinateVector(0,1), CoordinateVector(0,2)]));
/// ```
pub fn travel(snake: &mut Snake, direction: CoordinateVector, grow: bool) -> CoordinateVector {
	let &old_head = snake.back().unwrap();
	if !grow { snake.pop_front().unwrap(); }
	let new_head = old_head + direction;
	snake.push_back(old_head + direction);
	new_head
}

/* CONDITIONS */

/// Checks if the head of the snake is touching `object`.
/// This is useful for collision detection with food or obstacles
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use snake::types::CoordinateVector;
/// use snake::core::head_touching_object;
///
/// let snake = VecDeque::from(vec![CoordinateVector(5,5), CoordinateVector(5,6)]);
///
/// let object = CoordinateVector(5,6);
/// assert_eq!(head_touching_object(&snake, object), true);
///
/// let object = CoordinateVector(4,4);
/// assert_eq!(head_touching_object(&snake, object), false);
/// ```
pub fn head_touching_object(snake: &Snake, object: CoordinateVector) -> bool {
	*snake.back().unwrap() == object
}

/// Checks if the snakes head is touching itself.
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use snake::types::CoordinateVector;
/// use snake::core::head_touching_self;
///
/// let snake = VecDeque::from(vec![
/// 	CoordinateVector(5,5), CoordinateVector(5,6),
/// 	CoordinateVector(6,6), CoordinateVector(6,5),
/// 	CoordinateVector(5,5)]);
///
/// assert_eq!(head_touching_self(&snake), true);
///
/// let snake = VecDeque::from(vec![CoordinateVector(5,5), CoordinateVector(5,6)]);
///
/// assert_eq!(head_touching_self(&snake), false);
/// ```
pub fn head_touching_self(snake: &Snake) -> bool {
	let &head = snake.back().unwrap();
	// Find the position of first snake segment which is equal to the head
	let position = snake.iter()
		.position(|&coord| coord == head).unwrap();
	// Return true if the found position is not the head.
	position < snake.len() - 1
}

/// Checks if the snakes head is out of bounds.
///
/// These bounds are given in the form of a CoordinateVector,
/// and valid bounds are from 0 to the point given by `bounds` (exclusive)
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use snake::types::CoordinateVector;
/// use snake::core::head_out_of_bounds;
///
/// let snake = VecDeque::from(vec![CoordinateVector(14,0)]);
/// assert_eq!(head_out_of_bounds(&snake, CoordinateVector(15,15)), false);
///
/// let snake = VecDeque::from(vec![CoordinateVector(-1, 15)]);
/// // -1 is out of bounds because it's less than 0 (the implied lower bound),
/// // and 15 is out of bounds because it's greater than or equal to 15 (the explicit upper bound)
/// assert_eq!(head_out_of_bounds(&snake, CoordinateVector(15,15)), true);
/// ```
pub fn head_out_of_bounds(snake: &Snake, bounds: CoordinateVector) -> bool {
	let &head = snake.back().unwrap();
	head.0 >= bounds.0 || head.1 >= bounds.1 || head.0 < 0 || head.1 < 0
}

