// Texture : A Mazes texture represents the tendency that it has to create
// a certain type of a maze. For example, a binary tree maze texture will
// create a maze that has long corridors top-left to top-right (east to west in the north)
// and bottom-right to top-right. (south to north in the east) as it has
// to move north or east.
// Bias : A tendency towards a texture.

mod algorithms;
mod grid;
mod point;

mod prelude {
    pub use crate::algorithms::*;
    pub use crate::grid::*;
    pub use crate::point::*;

    pub use image::*;
    pub use rand::Rng;
}

use prelude::*;

fn main() {
    let mut grid = Grid::new(5, 5);
    let mut algorithm = algorithms::Sidewinder;

    algorithm.on(&mut grid);
    grid.distances.compute(grid.clone());
    grid.draw();

    let new_distances = grid.distances.path_to(&grid, Point::new((grid.width - 1) as i32, (grid.height - 1) as i32));
    grid.distances = new_distances;
    grid.draw();
    grid.to_png(64);
}
