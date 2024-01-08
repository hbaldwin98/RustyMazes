use std::{collections::HashMap, path::Path};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NeighborPoint {
    pub point: Point,
    pub linked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub point: Point,
    pub north: NeighborPoint,
    pub east: NeighborPoint,
    pub south: NeighborPoint,
    pub west: NeighborPoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distances {
    root: Point,
    cells: HashMap<Point, usize>,
}

impl Distances {
    pub fn new(root: Point) -> Self {
        let mut cells = HashMap::new();
        cells.insert(root, 0);
        Self { root, cells }
    }

    pub fn distance(&self, point: Point) -> Option<usize> {
        return self.cells.get(&point).copied();
    }

    pub fn compute(&mut self, grid: Grid) -> &mut Self {
        let mut frontier = vec![self.root];

        while !frontier.is_empty() {
            let mut new_frontier = Vec::new();
            
            for point in frontier {
                let cell = grid.get(point);

                if cell.is_none() {
                    continue;
                }

                for link in cell.unwrap().links() {
                    if self.cells.contains_key(&link) {
                        continue;
                    }

                    self.cells.insert(link, self.distance(point).unwrap() + 1);
                    new_frontier.push(link);
                }
            }

            frontier = new_frontier;
        }

        return self;
    }

    pub fn path_to(&self, grid: &Grid, goal: Point) -> Self {
        let mut current = goal;
        let mut breadcrumbs = Distances::new(self.root);
        breadcrumbs.cells.insert(current, self.distance(current).unwrap());

        while current != self.root {
            for neighbor in grid.get(current).unwrap().links() {
                if self.distance(neighbor) < self.distance(current) {
                    breadcrumbs.cells.insert(neighbor, self.distance(neighbor).unwrap());
                    current = neighbor;
                    break;
                }
            }
        }

        return breadcrumbs;
    }
}

#[allow(dead_code)]
impl Cell {
    pub fn new(point: Point) -> Self {
        Self {
            point,
            north: NeighborPoint {
                point: point + Point::new(0, -1),
                linked: false,
            },
            east: NeighborPoint {
                point: point + Point::new(1, 0),
                linked: false,
            },
            south: NeighborPoint {
                point: point + Point::new(0, 1),
                linked: false,
            },
            west: NeighborPoint {
                point: point + Point::new(-1, 0),
                linked: false,
            },
        }
    }

    pub fn link(&mut self, other_position: Point) {
        let point = other_position - self.point;
        let x = point.x;
        let y = point.y;

        match (x, y) {
            (0, -1) => {
                self.north.linked = true;
            }
            (0, 1) => {
                self.south.linked = true;
            }
            (1, 0) => {
                self.east.linked = true;
            }
            (-1, 0) => {
                self.west.linked = true;
            }
            _ => panic!("Invalid point"),
        }
    }

    pub fn links(&self) -> Vec<Point> {
        let mut links = Vec::new();

        if self.north.linked {
            links.push(self.north.point);
        }
        if self.south.linked {
            links.push(self.south.point);
        }
        if self.east.linked {
            links.push(self.east.point);
        }
        if self.west.linked {
            links.push(self.west.point);
        }

        return links;
    }

    pub fn linked(&self, other: Option<&Cell>) -> bool {
        if other.is_none() {
            return false;
        }

        return self.links().contains(&other.unwrap().point);
    }

    pub fn neighbors(&self, grid: &Grid) -> Vec<Cell> {
        let mut neighbors = Vec::new();

        if let Some(north) = grid.get(self.north.point) {
            neighbors.push(north.clone());
        }

        if let Some(south) = grid.get(self.south.point) {
            neighbors.push(south.clone());
        }

        if let Some(east) = grid.get(self.east.point) {
            neighbors.push(east.clone());
        }

        if let Some(west) = grid.get(self.west.point) {
            neighbors.push(west.clone());
        }

        return neighbors;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub distances: Distances
}

#[allow(dead_code)]
impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(Point::new(x as i32, y as i32)));
            }
        }

        Self {
            width,
            height,
            cells,
            distances: Distances::new(Point::new(0, 0))
        }
    }

    pub fn link(&mut self, a: Point, b: Point) {
        let index_a = self.point_to_index(a);
        let index_b = self.point_to_index(b);

        if let Some(index_a) = index_a {
            self.cells[index_a].link(b);
        }

        if let Some(index_b) = index_b {
            self.cells[index_b].link(a);
        }
    }

    pub fn neighbors(&self, point: Point) -> Vec<Point> {
        let mut neighbors = Vec::new();

        if let Some(north) = self.get(point.north()) {
            neighbors.push(north.point);
        }

        if let Some(south) = self.get(point.south()) {
            neighbors.push(south.point);
        }

        if let Some(east) = self.get(point.east()) {
            neighbors.push(east.point);
        }

        if let Some(west) = self.get(point.west()) {
            neighbors.push(west.point);
        }

        return neighbors;
    }

    fn format_radix(mut x: u128, radix: u32) -> String {
        let mut result = vec![];

        loop {
            let m = x % radix as u128;
            x = x / radix as u128;

            // will panic if you use a bad radix (< 2 or > 36).
            result.push(std::char::from_digit(m as u32, radix).unwrap());
            if x == 0 {
                break;
            }
        }
        result.into_iter().rev().collect()
    }

    pub fn contents_of(&self, cell: Cell) -> String {
        let distance = self.distances.distance(cell.point);
        
        if distance.is_some() {
            return Grid::format_radix(distance.unwrap() as u128, 36);
        }

        return String::from(" ");
    }

    pub fn get(&self, point: Point) -> Option<&Cell> {
        return self.cells.iter().find(|&cell| cell.point == point);
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut Cell> {
        return self.cells.iter_mut().find(|cell| cell.point == point);
    }

    pub fn random_cell(&self) -> Option<&Cell> {
        let index = rand::thread_rng().gen_range(0..self.cells.len());

        return self.cells.get(index);
    }

    pub fn iter_rows(&self) -> std::slice::ChunksExact<Cell> {
        return self.cells.chunks_exact(self.width);
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Cell> {
        return self.cells.iter_mut();
    }

    pub fn iter(&self) -> std::slice::Iter<Cell> {
        return self.cells.iter();
    }

    pub fn point_to_index(&self, point: Point) -> Option<usize> {
        if point.x < 0 || point.y < 0 {
            return None;
        }

        let index = (point.y * self.width as i32 + point.x) as usize;

        if index >= self.cells.len() {
            return None;
        }

        return Some(index);
    }

    pub fn update_cell(&mut self, cell: Cell) {
        let index = self.point_to_index(cell.point);

        if index.is_none() {
            return;
        }

        self.cells[index.unwrap()] = cell;
    }

    pub fn draw(&self) {
        let mut output = String::from("+");
        output.push_str("---+".repeat(self.width).as_str());
        output.push('\n');

        for row in self.iter_rows() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for cell in row {
                let body = format!(" {} ", self.contents_of(cell.clone()));
                let east_boundary = if cell.linked(self.get(cell.east.point.clone())) {
                    " "
                } else {
                    "|"
                };
                top.push_str(body.as_str());
                top.push_str(east_boundary);

                let south_boundary = if cell.linked(self.get(cell.south.point.clone())) {
                    "   "
                } else {
                    "---"
                };

                bottom.push_str(south_boundary);
                bottom.push_str("+");
            }

            output.push_str(&top);
            output.push_str("\n");
            output.push_str(&bottom);
            output.push_str("\n");
        }

        println!("{}", output);
    }

    pub fn to_png(&self, size: usize) {
        let img_width = self.width * size + 1;
        let img_height = self.height * size + 1;

        let background = image::Rgb([255u8, 255u8, 255u8]);
        let wall = image::Rgb([0u8, 0u8, 0u8]);

        let mut imgbuf =
            image::ImageBuffer::from_fn(img_width as u32, img_height as u32, |_, _| {
                return background;
            });

        for cell in self.iter() {
            let x1 = cell.point.x * size as i32;
            let y1 = cell.point.y * size as i32;
            let x2 = (cell.point.x + 1) * size as i32;
            let y2 = (cell.point.y + 1) * size as i32;

            if !cell.linked(self.get(cell.north.point.clone())) {
                for x in x1..x2 {
                    imgbuf.put_pixel(x as u32, y1 as u32, wall);
                }
            }

            if !cell.linked(self.get(cell.west.point.clone())) {
                for y in y1..y2 {
                    imgbuf.put_pixel(x1 as u32, y as u32, wall);
                }
            }

            if !cell.linked(self.get(cell.east.point.clone())) {
                for y in y1..y2 {
                    imgbuf.put_pixel(x2 as u32, y as u32, wall);
                }
            }

            if !cell.linked(self.get(cell.south.point.clone())) {
                for x in x1..x2 {
                    imgbuf.put_pixel(x as u32, y2 as u32, wall);
                }
            }
        }

        let path = Path::new("maze.png");
        let _ = imgbuf.save(path).unwrap();
    }
}

impl Iterator for Grid {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        return self.cells.pop();
    }
}
