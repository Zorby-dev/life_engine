use sdl2::pixels::Color;

use crate::util::Vector;

#[derive(PartialEq, Clone)]
pub enum CellState {
    Empty,
    Food,
    Producer,
    Mouth
}

impl CellState {
    pub fn color(&self) -> Color {
        match self {
            CellState::Empty => Color::BLACK,
            CellState::Food => Color::BLUE,
            CellState::Producer => Color::GREEN,
            CellState::Mouth => Color::YELLOW
        }
    }
}

pub struct Cell {
    pub pos: Vector,
    pub state: CellState,
}

impl Cell {
    pub fn new(pos: Vector) -> Self {
        Self {
            pos,
            state: CellState::Empty,
        }
    }
}
