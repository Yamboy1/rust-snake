use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;
use rand::prelude::*;

use pancurses::{initscr, noecho, Window, Input, endwin, curs_set};

use snake::{
	types::{CoordinateVector, Snake},
	base::{travel, head_touching_object, snake_touching_head, head_out_of_bounds},
};

fn main() {
	let window = initscr();
	window.refresh();
	window.keypad(true);
	window.nodelay(true);
	curs_set(0);
	noecho();
	let (y,x) = window.get_max_yx();

	let mut rng = thread_rng();
	let mut direction = CoordinateVector(1,0);
	let board_bounds = CoordinateVector(x,y);
	let mut snake = VecDeque::from(vec![CoordinateVector(0,0)]);
	let mut food = get_new_food_position(&snake, board_bounds, &mut rng);

	loop {
		display(&window, &snake, food);
		direction = get_new_direction(&window, direction);
		let eating_food = head_touching_object(&snake, food);
		if eating_food { food = get_new_food_position(&snake, board_bounds, &mut rng) }
		travel(&mut snake, direction, eating_food);
		if snake_touching_head(&snake) || head_out_of_bounds(&snake, board_bounds) { break; }
		sleep(Duration::from_millis(100));
	}
	endwin();
}

fn get_new_direction(window: &Window, old: CoordinateVector) -> CoordinateVector {
	match window.getch() {
		Some(Input::KeyLeft)  if old.1 != 0 => CoordinateVector(-1,0),
		Some(Input::KeyRight) if old.1 != 0 => CoordinateVector(1,0),
		Some(Input::KeyUp)    if old.0 != 0 => CoordinateVector(0,-1),
		Some(Input::KeyDown)  if old.0 != 0 => CoordinateVector(0,1),
		_ => old
	}
}

fn get_new_food_position(snake: &Snake, bounds: CoordinateVector, rng: &mut ThreadRng) -> CoordinateVector {
	let new_position = CoordinateVector(rng.gen_range(0,bounds.0), rng.gen_range(0,bounds.1));
	match snake.contains(&new_position) {
		true => get_new_food_position(snake, bounds, rng),
		false => new_position,
	}
}

fn display(window: &Window, snake: &Snake, food: CoordinateVector) {
	window.clear();
	window.mvaddch(food.1, food.0, '-');
	for segment in snake {
		window.mvaddch(segment.1, segment.0, 'a');
	}

}