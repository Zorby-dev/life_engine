use sdl2::{event::Event, video::Window, EventPump, mouse::MouseButton};

use crate::{
    grid::Grid,
    organism::{cell::CellState, organism::{Organism, OrganismState}},
    settings::Settings,
    util::{CellsToRender, Vector},
};

pub fn init() -> Result<(Settings, Window, EventPump, Grid, CellsToRender), String> {
    let context = sdl2::init()?;

    let video = context.video()?;

    let window = video
        .window("Life Engine", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .map_err(|err| err.to_string())?;

    let event_pump = context.event_pump()?;

    let settings = Settings {
        cell_size: Vector::new(2, 2),
    };

    let cells_to_render = vec![];

    let grid = Grid::new((1600, 1200));

    Ok((settings, window, event_pump, grid, cells_to_render))
}

pub enum EngineState {
    Continue(Vec<Organism>),
    Exit,
}

pub fn update(
    event_pump: &mut EventPump,
    settings: &Settings,
    grid: &mut Grid,
    cells_to_render: &mut CellsToRender,
    organisms: &mut Vec<Organism>
) -> Result<EngineState, String> {
    let cell_size_x = settings.cell_size.x as i32;
    let cell_size_y = settings.cell_size.y as i32;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return Ok(EngineState::Exit),
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let real_x = x / cell_size_x;
                let real_y = y / cell_size_y;

                grid.set_state((real_x - 1, real_y - 1), CellState::Producer, cells_to_render);
                grid.set_state((real_x, real_y), CellState::Mouth, cells_to_render);
                grid.set_state((real_x + 1, real_y + 1), CellState::Producer, cells_to_render);
                organisms.push(Organism::new((real_x, real_y),
                    vec![
                        Vector::new(-1, -1),
                        Vector::new(0, 0),
                        Vector::new(1, 1)
                    ]
                ))
            },
            _ => {}
        }
    }

    let mut new_organisms = Vec::with_capacity(organisms.len());

    for organism in organisms {
        match organism.update(grid, &mut new_organisms, cells_to_render) {
            OrganismState::Live => new_organisms.push(organism.clone()),
            OrganismState::Die => {}
        }
    }

    Ok(EngineState::Continue(new_organisms))
}
