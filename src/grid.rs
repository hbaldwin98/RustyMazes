use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    slice::ChunksExact,
};

use crate::prelude::*;

impl Iterator for dyn Grid {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        return None;
    }
}

impl Index<usize> for dyn Grid {
    type Output = Option<Cell>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.cells()[index];
    }
}

impl Index<Point> for dyn Grid {
    type Output = Option<Cell>;

    fn index(&self, index: Point) -> &Self::Output {
        return &self.cells()[self.point_to_index(index).unwrap()];
    }
}

impl IndexMut<usize> for dyn Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.cells_mut()[index];
    }
}

pub trait Grid {
    fn cells(&self) -> &Vec<Option<Cell>>;
    fn cells_mut(&mut self) -> &mut Vec<Option<Cell>>;

    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn link(&mut self, a: Point, b: Point, bidi: bool) {
        let index_a = self.point_to_index(a);
        let index_b = self.point_to_index(b);

        if let Some(index_a) = index_a {
            if let Some(cell_a) = self.cells_mut()[index_a].as_mut() {
                cell_a.link(b);
            }
        }

        if !bidi {
            return;
        }

        if let Some(index_b) = index_b {
            if let Some(cell_b) = self.cells_mut()[index_b].as_mut() {
                cell_b.link(a);
            }
        }
    }

    fn neighbors(&self, point: Point) -> Vec<Point> {
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

    fn get(&self, point: Point) -> Option<&Cell> {
        let cell = self
            .cells()
            .iter()
            .filter_map(|c| c.as_ref())
            .find(|&cell| cell.point == point);

        if let Some(cell) = cell {
            return Some(cell);
        }

        return None;
    }

    fn random_cell(&self) -> Option<&Cell> {
        let index = rand::thread_rng().gen_range(0..self.cells().len());
        let mut cell = self.cells().get(index).unwrap();

        while cell.is_none() {
            let index = rand::thread_rng().gen_range(0..self.cells().len());
            cell = self.cells().get(index).unwrap();
        }

        return Some(cell.as_ref().unwrap());
    }

    fn iter_rows(&self) -> ChunksExact<'_, Option<Cell>> {
        self.cells().chunks_exact(self.width())
    }

    fn point_to_index(&self, point: Point) -> Option<usize> {
        if point.x < 0 || point.y < 0 {
            return None;
        }

        let index = (point.y * self.width() as i32 + point.x) as usize;

        if index >= self.cells().len() {
            return None;
        }

        return Some(index);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RectangularGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Option<Cell>>,
    pub distances: Distances,
}

impl RectangularGrid {
    fn new(width: usize, height: usize) -> Self {
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

    fn contents_of(&self, cell: Option<Cell>) -> String {
        if let Some(cell) = cell {
            let distance = self.distances.distance(cell.point);

            if distance.is_some() {
                return RectangularGrid::format_radix(distance.unwrap() as u128, 36);
            }
        }

        return String::from(" ");
    }
}

impl Grid for RectangularGrid {
    fn cells(&self) -> &Vec<Option<Cell>> {
        self.cells.as_ref()
    }

    fn cells_mut(&mut self) -> &mut Vec<Option<Cell>> {
        self.cells.as_mut()
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Drawable for RectangularGrid {
    fn to_grid_image(&self, size: usize) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let img_width = self.width * size + 1;
        let img_height = self.height * size + 1;

        let mut imgbuf =
            image::ImageBuffer::from_fn(img_width as u32, img_height as u32, |_, _| {
                return BLACK;
            });

        for mode in vec!["background", "walls"] {
            for cell in self.cells.iter() {
                if let Some(cell) = cell {
                    let (x1, x2, y1, y2) = (
                        cell.point.x * size as i32,
                        (cell.point.x + 1) * size as i32,
                        cell.point.y * size as i32,
                        (cell.point.y + 1) * size as i32,
                    );

                    if mode == "background" {
                        let color = self.background_color_for(cell, &self.distances);
                        RectangularGrid::draw_line(&mut imgbuf, x1, y1, x2, y2, color);
                    } else {
                        if !cell.linked(self.get(cell.north.point.clone())) {
                            RectangularGrid::draw_line(&mut imgbuf, x1, y1, x2, y1, WHITE);
                        }

                        if !cell.linked(self.get(cell.west.point.clone())) {
                            RectangularGrid::draw_line(&mut imgbuf, x1, y1, x1, y2, WHITE);
                        }

                        if !cell.linked(self.get(cell.east.point.clone())) {
                            RectangularGrid::draw_line(&mut imgbuf, x2, y1, x2, y2, WHITE);
                        }

                        if !cell.linked(self.get(cell.south.point.clone())) {
                            RectangularGrid::draw_line(&mut imgbuf, x1, y2, x2, y2, WHITE);
                        }
                    }
                }
            }
        }

        return imgbuf;
    }
}

impl Display for RectangularGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("+");
        output.push_str("---+".repeat(self.width).as_str());
        output.push('\n');

        for row in self.iter_rows() {
            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for cell in row {
                let body = format!(" {} ", self.contents_of(*cell));

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

        write!(f, "{}", output)
    }
}

impl Maskable for RectangularGrid {
    fn from_mask(mask: &Mask) -> Self {
        let mut grid = RectangularGrid::new(mask.width, mask.height);
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolarGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Option<Cell>>,
    pub distances: Distances,
}

impl PolarGrid {
    fn new(width: usize, height: usize) -> Self {
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
}

impl Grid for PolarGrid {
    fn cells(&self) -> &Vec<Option<Cell>> {
        self.cells.as_ref()
    }

    fn cells_mut(&mut self) -> &mut Vec<Option<Cell>> {
        self.cells.as_mut()
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl Drawable for PolarGrid {
    fn to_grid_image(&self, cell_size: usize) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let img_size = 2 * cell_size * self.height;

        let mut imgbuf = image::ImageBuffer::new((img_size) as u32 + 1, (img_size) as u32 + 1);

        let center = (img_size / 2) as i32;

        for cell in self.cells.iter() {
            if let Some(cell) = cell {
                let cells_in_row = self
                    .iter_rows()
                    .nth(cell.point.y as usize)
                    .filter(|c| !c.is_empty())
                    .unwrap()
                    .len() as i32;

                let theta = 2.0 * std::f32::consts::PI / cells_in_row as f32;
                let inner_radius = cell.point.y * cell_size as i32;
                let outer_radius = (cell.point.y + 1) * cell_size as i32;

                let theta_ccw = cell.point.x as f32 * theta;
                let theta_cw = (cell.point.x + 1) as f32 * theta;

                let ax = center + (inner_radius as f32 * theta_ccw.cos()).round() as i32;
                let ay = center + (inner_radius as f32 * theta_ccw.sin()).round() as i32;
                //let bx = center + (outer_radius as f32 * theta_ccw.cos()).round() as i32;
                //let by = center + (outer_radius as f32 * theta_ccw.sin()).round() as i32;
                let cx = center + (inner_radius as f32 * theta_cw.cos()).round() as i32;
                let cy = center + (inner_radius as f32 * theta_cw.sin()).round() as i32;
                let dx = center + (outer_radius as f32 * theta_cw.cos()).round() as i32;
                let dy = center + (outer_radius as f32 * theta_cw.sin()).round() as i32;

                if !cell.links().contains(&Point::north(&cell.point)) {
                    RectangularGrid::draw_line(&mut imgbuf, ax, ay, cx, cy, WHITE);
                }

                if !cell.links().contains(&Point::east(&cell.point)) {
                    RectangularGrid::draw_line(&mut imgbuf, cx, cy, dx, dy, WHITE);
                }
            }
        }

        PolarGrid::circle(
            &mut imgbuf,
            center as u32,
            center as u32,
            self.height * cell_size,
            WHITE,
        );

        return imgbuf;
    }
}

impl Maskable for PolarGrid {
    fn from_mask(mask: &Mask) -> Self {
        let mut grid = PolarGrid::new(mask.width, mask.height);
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
