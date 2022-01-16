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
    Continue,
    Die
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

    pub fn update(&mut self, grid: &mut Grid, cells_to_render: &mut CellsToRender) -> OrganismState {
        
        if self.age == 1000 {
            for pos in &self.cells {
                grid.set_state(&self.pos + pos, CellState::Food, cells_to_render);
            }
            return OrganismState::Die;
        }
        
        let mut rng  = rand::thread_rng();

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

        OrganismState::Continue
    }
}
