# Flood-It

A simple 2D game where you have to change all cells of a grid to the same color.
There is only one thing you can do: you can flood fill (hence the name) from the
top left corner with any of the colors present on the grid.

This is a reimplementation of Jan Wolter’s [original HTML/JavaScript
implementation](http://unixpapa.com/floodit/).

Don’t expect too much from this implementation. I made this just for fun in a
few hours.

## Usage
As with most Rust projects, this game can be compiled using `cargo build
--release` and installed (to `~/.cargo/bin` by default) using `cargo install`.
Alternatively, you can run it locally using `cargo run --release`.

`flood-it` accepts two command line arguments:

* the number of colors (between 3 and 8) and
* the size of the grid (greater than 1).

To play on a 23x23 grid with all eight colors, execute `cargo run --release 8
23`. When only one argument is given, it is interpreted as the number of colors.
By default, you play on a 14x14 grid with 6 colors.
