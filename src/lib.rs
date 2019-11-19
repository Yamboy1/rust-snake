//! Snake is a crate that provides an API to help people
//! develop snake games without having to deal with all the
//! intricacies of how one works.
//!
//! Currently, we only provide a low level API, meaning that
//! you still need to deal with the game logic such as moving
//! the snake, or checking whether it has crashed into
//! a wall, however these have been abstracted into single
//! function calls. This gives you fine grained control over
//! how the game works, without forcing you to understand
//! exactly how the different components work.
//!
//! See our exported [binary] as an example of how to use
//! our API.
//!
//! In the future, we will also provide a much higher level
//! framework, for people to make working snakes games in a
//! matter of minutes. Watch our [github repository] for all
//! future developments.
//!
//! [binary]: https://github.com/Yamboy1/rust-snake/blob/master/src/bin/snake.rs
//! [github repository]: https://github.com/Yamboy1/rust-snake/

/// Types for the crate
pub mod types;
/// This is the heart of the crate. The core module contains the
/// basic functions required to make a snake game.
pub mod core;