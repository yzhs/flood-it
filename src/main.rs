use std::collections::HashMap;

use macroquad::prelude::*;

mod colour;
use crate::colour::*;

mod grid;
use crate::grid::*;

// Represent the state of the game as a graph.  One node for each cell.  There is an edge between
// two cells if and only if they are the same colour and have a distance of 1 according to the
// Manhattan metric.
//
// The game has only one real operation (i.e. not counting level generation): Switching the colour
// of the connected component containing the top left cell to whichever colour the clicked cell
// has.  This results in said component growing by every component it neighbours which has its new
// colour.  In essence, the aim of the gaim is to join connected components until only one is
// left.  Fewer clicks is better.
//
// So, we represent the game state as a collection of connected components, each of which has a
// colour, a list of coordinates of the constituent cells, and a list of it s neighbours.  As we
// do all our operations on connected components, we actually work not with the graph of
// neighbouring cells but with a graph of connected components.  The graph of connected components
// is made up of one node representing each connected component and an edge between any two
// connected components that have cells which share an edge, i.e. two cells which would have been
// connected by an edge in the original graph if they had had the same colour (which they did
// not).

struct Position {
    row: u8,
    column: u8,
}

struct ConnectedComponent {
    colour: Colour,
    cells: Vec<Position>,
}

struct Graph {
    neighours: HashMap<ConnectedComponent, Vec<ConnectedComponent>>,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
