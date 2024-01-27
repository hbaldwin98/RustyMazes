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

    pub fn neighbors(&self, grid: &dyn Grid) -> Vec<Cell> {
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

