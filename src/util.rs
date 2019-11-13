pub type Coord = (u32, u32);

#[derive(PartialEq, Debug)]
pub enum Direction {
	Up, Right, Left, Down,
}
