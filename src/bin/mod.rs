use snake::{
	model::Snake,
	Direction,
};

#[cfg_attr(tarpaulin, skip)]
fn main() {
	let snake = Snake::new((0,0), Direction::Right);
	println!("{:?}", snake.get_position());
	// TODO
}