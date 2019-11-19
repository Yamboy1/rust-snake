# Snake (WIP)

![Github Actions: Rust](https://github.com/Yamboy1/rust-snake/workflows/Rust/badge.svg) [![codecov](https://codecov.io/gh/Yamboy1/rust-snake/branch/master/graph/badge.svg)](https://codecov.io/gh/Yamboy1/rust-snake) ![License: Mit](https://img.shields.io/github/license/Yamboy1/rust-snake)

A snake game written with Rust and ncurses. Also has a public API for making your own snake game (in Rust).

# Installation

```bash
$ git clone https://github.com/Yamboy1/rust-snake
$ cd rust-snake
$ cargo install --path .
```

# Using the API

The docs are currently a WIP, however you can try having a look at ./src/bin/snake.rs for an example. Note that this
 is a very low level API for now until we develop a wrapper around it.