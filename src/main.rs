// Texture : A Mazes texture represents the tendency that it has to create
// a certain type of a maze. For example, a binary tree maze texture will
// create a maze that has long corridors top-left to top-right (east to west in the north)
// and bottom-right to top-right. (south to north in the east) as it has
// to move north or east.
// Bias : A tendency towards a texture.

mod algorithms;
mod distances;
mod drawable;
mod grid;
mod cell;
mod mask;
mod point;

mod prelude {
    pub use crate::algorithms::*;
    pub use crate::distances::*;
    pub use crate::drawable::*;
    pub use crate::grid::*;
    pub use crate::cell::*;
    pub use crate::mask::*;
    pub use crate::point::*;

    pub use clap::Parser;
    pub use image::*;
    pub use rand::Rng;
    pub use std::{path::Path, process::Command};

    pub const GRID_WIDTH: usize = 8;
    pub const GRID_HEIGHT: usize = 8;
    pub const WHITE: Rgb<u8> = image::Rgb([255u8, 255u8, 255u8]);
    pub const BLACK: Rgb<u8> = image::Rgb([0u8, 0u8, 0u8]);

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        #[arg(
            short = 'w',
            long,
            help = "A text mask to use for the maze, made of . and x characters. Input is the full path of the .txt file.",
            conflicts_with = "mask_image"
        )]
        pub mask: Option<String>,
        #[arg(
            short,
            long,
            help = "An image mask to use for the maze. Input is the full path of the image file.",
            conflicts_with = "mask"
        )]
        pub mask_image: Option<String>,
        #[arg(
            short,
            long,
            help = "The algorithm to apply. Not all masks will work properly with all algorithms.",
            default_value = "recursivebacktracker",
        )]
        pub algorithm: Option<String>,
        #[arg(short, long, help = "Output the maze as a PNG image.")]
        pub to_png: bool,
        #[arg(
            short = 'p',
            long,
            help = "Output the maze as a polar coordinated PNG image (circle)."
        )]
        pub to_polar_png: bool,
        #[arg(
            short,
            long,
            help = "Resolution of the output image.",
            requires = "to_png",
            default_value = "16"
        )]
        pub resolution: Option<usize>,
        #[arg(
            short,
            long,
            help = "Show Dijkstra distances in output.",
            requires = "output",
            default_value = "false"
        )]
        pub show_distances: bool,
        #[arg(short, long, help = "Show maze in output.", default_value = "false")]
        pub output: bool,
    }
}

use prelude::*;

fn get_algorithm(name: &str) -> Algorithm {
    match name.to_lowercase().as_str() {
        "binarytree" => Algorithm::BinaryTree,
        "sidewinder" => Algorithm::Sidewinder,
        "aldousbroder" => Algorithm::AldousBroder,
        "wilsons" => Algorithm::Wilsons,
        "huntandkill" => Algorithm::HuntAndKill,
        "recursivebacktracker" => Algorithm::RecursiveBacktracker,
        "none" => Algorithm::None,
        _ => panic!("Algorithm not found"),
    }
}

fn main() {
    let args = Args::parse();
    generate_maze(args);
}

fn generate_maze(args: Args) {
    let mut algorithm = get_algorithm(args.algorithm.unwrap().as_str());

    let mut mask = match args.mask {
        Some(mask) => match Mask::from_txt(&mask) {
            Ok(mask) => mask,
            Err(e) => panic!("Error: {}", e),
        },
        None => Mask::new(GRID_WIDTH, GRID_HEIGHT),
    };

    mask = match args.mask_image {
        Some(mask_image) => match Mask::from_png(&mask_image) {
            Ok(mask) => mask,
            Err(e) => panic!("Error: {}", e),
        },
        None => mask,
    };

    let mut grid = RectangularGrid::from_mask(&mask);
    algorithm.on(&mut grid);

    if args.show_distances {
        grid.distances.compute(grid.clone());
    }

    if args.output {
        println!("{}", grid);
    }

    if args.to_png {
        let path = Path::new("maze.png");
        grid.to_grid_image(args.resolution.unwrap())
            .save(path)
            .unwrap();
    }

    if args.to_polar_png {
        let mut grid = PolarGrid::from_mask(&mask);
        algorithm.on(&mut grid);

        let path = Path::new("maze_polar.png");
        grid.to_grid_image(args.resolution.unwrap())
            .save(path)
            .unwrap();
    }
}
