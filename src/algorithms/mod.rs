use crate::prelude::*;

pub struct BinaryTree;
pub struct Sidewinder;

pub trait Algorithm {
    fn on(&mut self, grid: &mut Grid);
}

impl Algorithm for BinaryTree {
    fn on(&mut self, grid: &mut Grid) {
        let mut actions = Vec::new();
        for cell in grid.iter() {
            let mut neighbors = Vec::new();

            if let Some(north) = grid.get(cell.north.point.clone()) {
                neighbors.push(north);
            }

            if let Some(east) = grid.get(cell.east.point.clone()) {
                neighbors.push(east);
            }

            if !neighbors.is_empty() {
                let index = rand::thread_rng().gen_range(0..neighbors.len());
                let neighbor_point = neighbors[index].point.clone();
                let cell_point = cell.point.clone();

                actions.push((cell_point, neighbor_point));
            }
        }

        for (cell_point, neighbor_point) in actions.iter() {
            grid.link(*cell_point, *neighbor_point);
        }
    }
}

impl Algorithm for Sidewinder {
    fn on(&mut self, grid: &mut Grid) {
        let mut random = rand::thread_rng();
        let mut actions = Vec::new();

        for row in grid.iter_rows() {
            let mut run = Vec::new();

            for cell in row.iter() {
                run.push(cell);

                let at_eastern_boundary = cell.east.point.x == (grid.width as i32);
                let at_northern_boundary = cell.north.point.y <= 0;

                let should_close_out =
                    at_eastern_boundary || (!at_northern_boundary && random.gen_bool(0.5));

                if should_close_out {
                    let index = random.gen_range(0..run.len());
                    let member = run.get(index).unwrap();
                    let north = member.north.point;

                    actions.push((member.point.clone(), north.clone()));
                    run.clear();
                } else {
                    actions.push((cell.point.clone(), cell.east.point.clone()));
                }
            }
        }

        for (cell_point, neighbor_point) in actions.iter() {
            grid.link(*cell_point, *neighbor_point);
        }
    }
}
