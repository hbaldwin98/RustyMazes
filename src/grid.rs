use std::{path::Path, slice::ChunksExact};

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
    pub cells: Vec<Option<Cell>>,
    pub distances: Distances,
}

#[allow(dead_code)]
impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                cells.push(Some(Cell::new(Point::new(x as i32, y as i32))));
            }
        }

        Self {
            width,
            height,
            cells,
            distances: Distances::new(Point::new(0, 0)),
        }
    }

    pub fn link(&mut self, a: Point, b: Point, bidi: bool) {
        let index_a = self.point_to_index(a);
        let index_b = self.point_to_index(b);

        if let Some(index_a) = index_a {
            if let Some(cell_a) = self.cells[index_a].as_mut() {
                cell_a.link(b);
            }
        }

        if !bidi {
            return;
        }

        if let Some(index_b) = index_b {
            if let Some(cell_b) = self.cells[index_b].as_mut() {
                cell_b.link(a);
            }
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

        return result.into_iter().rev().collect();
    }

    pub fn get(&self, point: Point) -> Option<&Cell> {
        let cell = self.iter().find(|&cell| cell.point == point);

        if let Some(cell) = cell {
            return Some(cell);
        }

        return None;
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut Cell> {
        let cell = self
            .cells
            .iter_mut()
            .find(|cell| cell.unwrap().point == point);

        if let Some(cell) = cell {
            return cell.as_mut();
        }

        return None;
    }

    pub fn dead_ends(&self) -> Vec<&Cell> {
        return self
            .cells
            .iter()
            .filter(|cell| cell.unwrap().links().len() == 1)
            .map(|cell| cell.as_ref().unwrap())
            .collect();
    }

    pub fn random_cell(&self) -> Option<&Cell> {
        let index = rand::thread_rng().gen_range(0..self.cells.len());
        let mut cell = self.cells.get(index).unwrap();

        while cell.is_none() {
            let index = rand::thread_rng().gen_range(0..self.cells.len());
            cell = self.cells.get(index).unwrap();
        }

        return Some(cell.as_ref().unwrap());
    }

    pub fn iter_rows(&self) -> ChunksExact<'_, Option<Cell>> {
        self.cells.chunks_exact(self.width)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut().filter_map(|c| c.as_mut())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter().filter_map(|c| c.as_ref())
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

        self.cells[index.unwrap()] = Some(cell);
    }

    pub fn draw(&self, include_distances: bool) {
        let mut output = String::from("+");
        output.push_str("---+".repeat(self.width).as_str());
        output.push('\n');

        for row in self.iter_rows() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for cell in row {
                let mut body = format!(" {} ", self.contents_of(*cell));

                if !include_distances {
                    body = String::from("   ");
                }

                let east_boundary = if cell.is_some()
                    && cell
                        .unwrap()
                        .linked(self.get(cell.unwrap().east.point.clone()))
                {
                    " "
                } else {
                    "|"
                };
                top.push_str(body.as_str());
                top.push_str(east_boundary);

                let south_boundary = if cell.is_some()
                    && cell
                        .unwrap()
                        .linked(self.get(cell.unwrap().south.point.clone()))
                {
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

    pub fn contents_of(&self, cell: Option<Cell>) -> String {
        if let Some(cell) = cell {
            let distance = self.distances.distance(cell.point);

            if distance.is_some() {
                return Grid::format_radix(distance.unwrap() as u128, 36);
            }
        }

        return String::from(" ");
    }

    pub fn background_color_for(&self, cell: &Cell) -> Rgb<u8> {
        let distance = self.distances.distance(cell.point);

        if distance.is_none() {
            return BLACK;
        }

        let (max_distance, _) = self.distances.max(self);

        if max_distance == 0 {
            return BLACK;
        }

        let intensity = (max_distance - distance.unwrap()) as f64 / max_distance as f64;
        let dark = (255.0 * intensity) as u8;
        let bright = 128 + (127.0 * intensity) as u8;
        let color = image::Rgb([dark, bright, dark]);

        return color;
    }

    pub fn to_png(&self, size: usize) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let img_width = self.width * size + 1;
        let img_height = self.height * size + 1;

        let mut imgbuf =
            image::ImageBuffer::from_fn(img_width as u32, img_height as u32, |_, _| {
                return BLACK;
            });

        for mode in vec!["background", "walls"] {
            for cell in self.iter() {
                let (x1, x2, y1, y2) = (
                    cell.point.x * size as i32,
                    (cell.point.x + 1) * size as i32,
                    cell.point.y * size as i32,
                    (cell.point.y + 1) * size as i32,
                );

                if mode == "background" {
                    let color = self.background_color_for(cell);
                    for x in x1..x2 {
                        for y in y1..y2 {
                            imgbuf.put_pixel(x as u32, y as u32, color);
                        }
                    }
                } else {
                    if !cell.linked(self.get(cell.north.point.clone())) {
                        for x in x1..x2 {
                            imgbuf.put_pixel(x as u32, y1 as u32, WHITE);
                        }
                    }

                    if !cell.linked(self.get(cell.west.point.clone())) {
                        for y in y1..y2 {
                            imgbuf.put_pixel(x1 as u32, y as u32, WHITE);
                        }
                    }

                    if !cell.linked(self.get(cell.east.point.clone())) {
                        for y in y1..y2 {
                            imgbuf.put_pixel(x2 as u32, y as u32, WHITE);
                        }
                    }

                    if !cell.linked(self.get(cell.south.point.clone())) {
                        for x in x1..x2 {
                            imgbuf.put_pixel(x as u32, y2 as u32, WHITE);
                        }
                    }
                }
            }
        }

        // Put a pixel at the bottom right
        imgbuf.put_pixel(
            (self.width * size) as u32,
            (self.height * size) as u32,
            BLACK,
        );

        return imgbuf;
    }
}

impl Maskable for Grid {
    fn from_mask(mask: &Mask) -> Self {
        let mut grid = Grid::new(mask.width, mask.height);
        grid.mask(mask);

        // return the first true cell
        let mut start = None;
        for (i, cell) in grid.cells.iter().enumerate() {
            if cell.is_some() {
                start = Some(i);
                break;
            }
        }

        if let Some(start) = start {
            let point = Point::new((start % grid.width) as i32, (start / grid.width) as i32);
            grid.distances = Distances::new(point);
        }

        return grid;
    }

    fn mask(&mut self, mask: &Mask) {
        for (i, value) in mask.mask.iter().enumerate() {
            if !value {
                self.cells[i] = None;
            }
        }
    }
}
