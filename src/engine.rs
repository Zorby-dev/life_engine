use sdl2::{event::Event, mouse::MouseButton, video::Window, EventPump};
use hashbrown::{HashMap};

const WIDTH: i32 = 100;
const HEIGHT: i32 = 100;

use crate::{
    grid::Grid,
    organism::{
        cell::CellState,
        organism::{Organism, OrganismState},
    },
    settings::Settings,
    util::{CellsToRender, Vector},
};

pub fn init() -> Result<
    (
        Settings,
        Window,
        EventPump,
        Grid,
        CellsToRender,
        HashMap<Vector, Organism>,
        HashMap<Vector, Organism>
    ),
    String,
> {
    let context = sdl2::init()?;

    let video = context.video()?;

    let settings = Settings {
        cell_size: Vector::new(5, 5),
    };

    let window = video
        .window(
            "Life Engine",
            (WIDTH * settings.cell_size.x) as u32,
            (HEIGHT * settings.cell_size.y) as u32,
        )
        .opengl()
        .position_centered()
        .build()
        .map_err(|err| err.to_string())?;

    let event_pump = context.event_pump()?;

    let cells_to_render = vec![];

    let grid = Grid::new((WIDTH, HEIGHT));

    let organisms = HashMap::with_capacity((WIDTH * HEIGHT) as usize);

    let organisms_shadow = organisms.clone();

    Ok((
        settings,
        window,
        event_pump,
        grid,
        cells_to_render,
        organisms,
        organisms_shadow
    ))
}

pub enum EngineState {
    Continue(),
    Exit,
}

pub fn update(
    event_pump: &mut EventPump,
    settings: &Settings,
    grid: &mut Grid,
    cells_to_render: &mut CellsToRender,
    mut organisms: &mut HashMap<Vector, Organism>,
    organisms_shadow: &mut HashMap<Vector, Organism>
) -> Result<EngineState, String> {
    let cell_size_x = settings.cell_size.x as i32;
    let cell_size_y = settings.cell_size.y as i32;

    for event in event_pump.poll_iter() {
        match event {
            | Event::Quit { .. } => return Ok(EngineState::Exit),
            | Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                let real_x = x / cell_size_x;
                let real_y = y / cell_size_y;

                grid.set_state(
                    (
                        real_x - 1,
                        real_y - 1,
                    ),
                    CellState::Producer,
                    cells_to_render,
                );
                grid.set_state(
                    (
                        real_x, real_y,
                    ),
                    CellState::Mouth,
                    cells_to_render,
                );
                grid.set_state(
                    (
                        real_x + 1,
                        real_y + 1,
                    ),
                    CellState::Producer,
                    cells_to_render,
                );
                organisms.insert(
                    (real_x, real_y).into(),
                    Organism::new(
                        (
                            real_x, real_y,
                        ),
                        vec![Vector::new(-1, -1), Vector::new(0, 0), Vector::new(1, 1)],
                    ),
                );
            }
            | _ => {}
        }
    }

    organisms_shadow.clone_from(&organisms);

    for (pos, organism) in organisms_shadow.iter_mut() {
        match organism.update(
            grid,
            &mut organisms,
            cells_to_render,
        ) {
            OrganismState::Live => {},
            OrganismState::Die => { organisms.remove(pos); },
        }
    }

    Ok(EngineState::Continue())
}
