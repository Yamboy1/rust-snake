use std::collections::VecDeque;
use std::ops::Add;

fn main() {
	// First let's focus on getting a snake implementation,
	// not worrying about ncurses or anything like that
	let segments =
			vec![CoordinateVector(0,0), CoordinateVector(0,1)];
	let direction = CoordinateVector(1,0);
	let mut snake = Snake::new(segments,	direction);
	println!("{:?}", snake.segments);
	let new_head = travel(&mut snake);
	travel(&mut snake);
	println!("{}", check_touching(&snake, new_head));
	println!("{:?}", snake.segments);
}

fn travel(snake: &mut Snake) -> CoordinateVector {
	let &current_head = snake.segments.back().unwrap();
	let new_head = current_head + snake.direction;
	snake.segments.pop_front();
	snake.segments.push_back(new_head);
	new_head
}

fn check_touching(snake: &Snake, new_head: CoordinateVector) -> bool {
	let first_index =
			snake.segments.iter().position(|&x| x == new_head).unwrap();
	// Return true if there is an earlier element that is equal to the head
	first_index < snake.segments.len() - 1
}

#[derive(Debug)]
struct Snake {
	segments: VecDeque<CoordinateVector>,
	direction: CoordinateVector,
}

impl Snake {
	fn new(segments: Vec<CoordinateVector>, direction: CoordinateVector) -> Self {
		Self {
			segments: VecDeque::from(segments),
			direction,
		}
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct CoordinateVector(i32,i32);
impl Add for CoordinateVector {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		CoordinateVector(self.0 + rhs.0, self.1 + rhs.1)
	}
}