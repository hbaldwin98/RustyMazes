use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distances {
    pub root: Point,
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

    pub fn shortest_path_to(&self, grid: &Grid, goal: Point) -> Self {
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

    pub fn max(&self, grid: &Grid) -> (usize, Point) {
        let mut max_distance = 0;
        let mut max_point = self.root;

        for cell in grid.iter() {
            let distance = self.distance(cell.point).unwrap();

            if distance > max_distance {
                max_distance = distance;
                max_point = cell.point;
            }
        }

        return (max_distance, max_point);
    }
}
