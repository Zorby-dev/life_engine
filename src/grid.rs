use crate::{
    organism::cell::{Cell, CellState},
    util::{CellsToRender, Vector},
};

pub struct Grid {
    grid: Vec<Cell>,
    dimensions: Vector,
}

impl Grid {
    pub fn new(dimensions: impl Into<Vector>) -> Self {
        let dimensions = dimensions.into();

        let mut grid = Vec::with_capacity((dimensions.x * dimensions.y) as usize);
        for y in 0..dimensions.y {
            for x in 0..dimensions.x {
                grid.push(Cell::new(Vector::new(x, y)));
            }
        }

        Self { grid, dimensions }
    }

    pub fn get(&self, pos: impl Into<Vector>) -> Option<&Cell> {
        let pos = pos.into();

        self.grid.get((pos.y * self.dimensions.x + pos.x) as usize)
    }

    pub fn get_mut(&mut self, pos: impl Into<Vector>) -> Option<&mut Cell> {
        let pos = pos.into();

        self.grid
            .get_mut((pos.y * self.dimensions.x + pos.x) as usize)
    }

    pub fn set_state(
        &mut self,
        pos: impl Into<Vector>,
        state: CellState,
        cells_to_render: &mut CellsToRender,
    ) {
        match self.get_mut(pos).ok_or("".to_string()) {
            Ok(cell) => {
                cell.state = state;
                cells_to_render.push(cell.pos.clone());
            }
            Err(_) => {
                return;
            }
        }
    }

    pub fn set_state_if_empty(
        &mut self,
        pos: impl Into<Vector>,
        state: CellState,
        cells_to_render: &mut CellsToRender,
    ) {
        let pos = pos.into();

        if self.match_state(&pos, CellState::Empty) {
            self.set_state(pos, state, cells_to_render);
        }
    }

    pub fn match_state(&self, pos: impl Into<Vector>, state: CellState) -> bool {
        match self.get(pos) {
            Some(cell) => cell.state == state,
            None => false
        }
    }
}
