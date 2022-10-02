use std::collections::HashMap;

use crate::colour::Colour;
use crate::grid::Grid;

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
    row: usize,
    column: usize,
}

struct ConnectedComponent {
    colour: Colour,
    cells: HashSet<Position>,
}

struct Graph {
    neighours: HashMap<ConnectedComponent, Vec<ConnectedComponent>>,
}
