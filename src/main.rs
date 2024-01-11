// Texture : A Mazes texture represents the tendency that it has to create
// a certain type of a maze. For example, a binary tree maze texture will
// create a maze that has long corridors top-left to top-right (east to west in the north)
// and bottom-right to top-right. (south to north in the east) as it has
// to move north or east.
// Bias : A tendency towards a texture.

mod algorithms;
mod distances;
mod grid;
mod point;

macro_rules! name_of {
    ($name:ident in $ty:ty) => {{
        #[allow(dead_code)]
        fn dummy(v: $ty) {
            let _ = &v.$name;
        }
        stringify!($name)
    }};

    ($name:ident) => {{
        let _ = &$name;
        stringify!($name)
    }};
}

mod prelude {
    pub use crate::algorithms::*;
    pub use crate::distances::*;
    pub use crate::grid::*;
    pub use crate::point::*;

    pub use image::*;
    pub use rand::Rng;

    pub const GRID_WIDTH: usize = 8;
    pub const GRID_HEIGHT: usize = 8;
    pub const ALGORITHM: &str = name_of!(Wilsons);
    pub const WHITE: Rgb<u8> = image::Rgb([255u8, 255u8, 255u8]);
    pub const BLACK: Rgb<u8> = image::Rgb([0u8, 0u8, 0u8]);
}

use prelude::*;

fn main() {
    //generate_normal_maze();
    generate_picture_maze();
    //draw_max_distance_maze();
}

fn get_algorithm() -> Box<dyn Algorithm> {
    match ALGORITHM {
        "BinaryTree" => Box::new(BinaryTree),
        "Sidewinder" => Box::new(Sidewinder),
        "AldousBroder" => Box::new(AldousBroder),
        "Wilsons" => Box::new(Wilsons),
        _ => panic!("Algorithm not found"),
    }
}

fn generate_normal_maze() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut algorithm = get_algorithm();

    algorithm.on(&mut grid);
    grid.distances.compute(grid.clone());

    grid.draw();
}

fn generate_picture_maze() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut algorithm = get_algorithm();
    algorithm.on(&mut grid);

    let new_distances = Distances::new(Point::new(
        (grid.width / 2) as i32,
        (grid.height / 2) as i32,
    ));
    grid.distances = new_distances;
    grid.distances.compute(grid.clone());

    grid.to_png(32);
}

fn draw_max_distance_maze() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut algorithm = get_algorithm();

    algorithm.on(&mut grid);
    grid.distances.compute(grid.clone());

    let distances = &grid.distances;
    let (_, new_start) = distances.max(&grid);

    let mut max_distances = Distances::new(new_start);
    max_distances.compute(grid.clone());
    let (_, max_point) = max_distances.max(&grid);

    grid.distances = max_distances.shortest_path_to(&grid, max_point);
    grid.draw();
}
