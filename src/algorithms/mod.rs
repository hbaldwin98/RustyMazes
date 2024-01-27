use crate::prelude::*;

pub enum Algorithm {
    BinaryTree,
    Sidewinder,
    AldousBroder,
    Wilsons,
    HuntAndKill,
    RecursiveBacktracker,
    None,
}

impl Algorithm {
    pub fn on(&mut self, grid: &mut dyn Grid) {
        match self {
            Algorithm::BinaryTree => self.binary_tree(grid),
            Algorithm::Sidewinder => self.sidewinder(grid),
            Algorithm::AldousBroder => self.aldous_broder(grid),
            Algorithm::Wilsons => self.wilsons(grid),
            Algorithm::HuntAndKill => self.hunt_and_kill(grid),
            Algorithm::RecursiveBacktracker => self.recursive_backtracker(grid),
            Algorithm::None => {}
        }
    }

    fn binary_tree(&mut self, grid: &mut dyn Grid) {
        let mut actions = Vec::new();
        for cell in grid.cells().iter() {
            let mut neighbors = Vec::new();
            if let Some(cell) = cell {
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
        }

        for (cell_point, neighbor_point) in actions.iter() {
            grid.link(*cell_point, *neighbor_point, true);
        }
    }

    fn sidewinder(&mut self, grid: &mut dyn Grid) {
        let mut random = rand::thread_rng();
        let mut actions = Vec::new();

        for row in grid.iter_rows() {
            let mut run = Vec::new();

            for cell in row.iter() {
                if let Some(cell) = cell {
                    run.push(cell);

                    let at_eastern_boundary = cell.east.point.x == (grid.width() as i32);
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
        }

        for (cell_point, neighbor_point) in actions.iter() {
            grid.link(*cell_point, *neighbor_point, true);
        }
    }

    fn aldous_broder(&mut self, grid: &mut dyn Grid) {
        let mut random = rand::thread_rng();

        let mut cell = *grid.random_cell().unwrap();
        let mut unvisited = grid.width() * grid.height() - 1;

        while unvisited > 0 {
            let neighbors = cell.neighbors(grid);
            let random_index = random.gen_range(0..neighbors.len());
            let neighbor = neighbors.get(random_index).unwrap();

            if neighbor.links().len() == 0 {
                grid.link(cell.point, neighbor.point, true);
                unvisited -= 1;
            }

            cell = neighbor.clone();
        }
    }

    fn wilsons(&mut self, grid: &mut dyn Grid) {
        let mut unvisited = grid
            .cells()
            .iter()
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect::<Vec<Cell>>()
            .clone();

        let mut random = rand::thread_rng();
        let index = random.gen_range(0..unvisited.len());

        unvisited.remove(index);

        while !unvisited.is_empty() {
            let index = random.gen_range(0..unvisited.len());
            let mut cell = *unvisited.get(index).unwrap();
            let mut path = vec![cell.clone()];

            while unvisited.contains(&cell) {
                let index = random.gen_range(0..cell.neighbors(grid).len());
                cell = *cell.neighbors(grid).get(index).unwrap();

                let position = path.iter().position(|c| c == &cell);

                if let Some(position) = position {
                    path.truncate(position + 1);
                } else {
                    path.push(cell.clone());
                }
            }

            for i in 0..path.len() - 1 {
                grid.link(path[i].point, path[i + 1].point, true);
                unvisited.retain(|c| c != &path[i]);
            }
        }
    }

    fn hunt_and_kill(&mut self, grid: &mut dyn Grid) {
        let mut random = rand::thread_rng();
        let mut current = Some(*grid.random_cell().unwrap());

        while current.is_some() {
            let neighbors = current.unwrap().neighbors(grid);
            let mut unvisited_neighbors = Vec::new();

            for neighbor in neighbors {
                if neighbor.links().is_empty() {
                    unvisited_neighbors.push(neighbor);
                }
            }

            if !unvisited_neighbors.is_empty() {
                let index = random.gen_range(0..unvisited_neighbors.len());
                let neighbor = *unvisited_neighbors.get(index).unwrap();
                grid.link(current.unwrap().point, neighbor.point, true);
                current = Some(neighbor);
            } else {
                let cells = grid
                    .cells()
                    .iter()
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect::<Vec<Cell>>();

                current = None;

                for cell in cells.iter() {
                    let mut visited_neighors = Vec::new();

                    for neighbor in cell.neighbors(grid) {
                        if !neighbor.links().is_empty() {
                            visited_neighors.push(neighbor);
                        }
                    }

                    if cell.links().is_empty() && !visited_neighors.is_empty() {
                        let index = random.gen_range(0..visited_neighors.len());
                        let neighbor = *visited_neighors.get(index).unwrap();
                        grid.link(cell.point, neighbor.point, true);
                        current = Some(*cell);
                        break;
                    }
                }
            }
        }
    }

    fn recursive_backtracker(&mut self, grid: &mut dyn Grid) {
        let mut random = rand::thread_rng();
        let mut stack: Vec<Point> = Vec::new();
        let random_cell = grid.random_cell().unwrap().clone();
        stack.push(random_cell.point);

        while !stack.is_empty() {
            let current = stack.last();
            let neighbors = grid
                .neighbors(*current.unwrap())
                .iter()
                .map(|&p| grid.get(p).unwrap())
                .filter(|&n| n.links().is_empty())
                .map(|&n| n.point)
                .collect::<Vec<Point>>();

            if neighbors.is_empty() {
                stack.pop();
            } else {
                let index = random.gen_range(0..neighbors.len());
                let neighbor = *neighbors.get(index).unwrap();

                grid.link(*current.unwrap(), neighbor, true);
                stack.push(grid.get(neighbor).unwrap().point);
            }
        }
    }
}
