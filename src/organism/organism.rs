use rand::Rng;

use crate::{grid::Grid, util::{Vector, CellsToRender}};

use super::cell::CellState;

#[derive(Clone)]
pub struct Organism {
    pos: Vector,
    cells: Vec<Vector>,
    food: u16,
    age: u16
}

const FOOD_PROD_PROB: u32 = 5;

pub enum OrganismState {
    Live,
    Die,
}

impl Organism {
    pub fn new(pos: impl Into<Vector>, cells: Vec<Vector>) -> Self {
        Self {
            pos: pos.into(),
            cells,
            food: 0,
            age: 0
        }
    }

    pub fn clone_with_cells(&self, pos: impl Into<Vector>, grid: &mut Grid, cells_to_render: &mut CellsToRender) -> Self {
        let mut cells = vec![];

        let pos = pos.into();

        for cell_pos in &self.cells {
            let cell = grid.get(&self.pos + cell_pos).unwrap().state.clone();

            cells.push((cell_pos.clone(), cell));
        }

        Self::new_with_cells(pos, cells, grid, cells_to_render)
    }

    pub fn new_with_cells(pos: impl Into<Vector>, cells: Vec<(Vector, CellState)>, grid: &mut Grid, cells_to_render: &mut CellsToRender) -> Self {
        let mut cell_poses = Vec::with_capacity(cells.len());

        let pos = pos.into();

        for (cell_pos, cell) in cells {
            grid.set_state(&pos + &cell_pos, cell, cells_to_render);
            cell_poses.push(cell_pos);
        }

        Self::new(pos, cell_poses)
    }

    pub fn update(&mut self, grid: &mut Grid, organisms: &mut Vec<Organism>, cells_to_render: &mut CellsToRender) -> OrganismState {
        
        if self.age == 1000 {
            for pos in &self.cells {
                grid.set_state(&self.pos + pos, CellState::Food, cells_to_render);
            }
            return OrganismState::Die;
        }

        let mut rng = rand::thread_rng();

        if self.food > 10 {
            let offspring_pos = &self.pos + &Vector::new(rng.gen_range(-3..=3), rng.gen_range(-3..=3));

            if self.is_pos_clear(&offspring_pos, grid) {
                organisms.push(self.clone_with_cells(&offspring_pos, grid, cells_to_render));
                self.food -= 10;
            }
        }


        for pos in &self.cells {
            let pos = &self.pos + pos;
            
            let cell = grid.get(&pos).unwrap();

            match cell.state {
                CellState::Producer => {
                    if rng.gen_range(0..=100) <= FOOD_PROD_PROB {
                        let neighbors = pos.cross_neighbors();
                        grid.set_state_if_empty(
                            &neighbors[rng.gen_range(0..4)],
                            CellState::Food, cells_to_render
                        );
                    }
                },
                CellState::Mouth => {
                    for neighbor in &pos.neighbors() {
                        if grid.match_state(neighbor, CellState::Food) {
                            grid.set_state(neighbor, CellState::Empty, cells_to_render);
                            self.food += 1;
                        }
                    }
                }
                CellState::Empty | CellState::Food => unreachable!(),
            }
        }

        self.age += 1;

        OrganismState::Live
    }

    fn is_pos_clear(&self, pos: impl Into<Vector>, grid: &Grid) -> bool {
        let in_pos = pos.into();

        for pos in &self.cells {
            let real_pos = &in_pos + pos;

            if !grid.match_state(real_pos, CellState::Empty) {
                return false;
            }
        }

        true
    }
}
