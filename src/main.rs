use engine::EngineState;
use fps_clock::FpsClock;

mod engine;
mod grid;
mod organism;
mod renderer;
mod settings;
mod util;

fn main() -> Result<(), String> {
    #[rustfmt::skip]
    let (
        settings,
        window,
        mut event_pump,
        mut grid,
        mut cells_to_render
    ) = engine::init()?;

    let mut canvas = renderer::init(window)?;

    let mut organisms = vec![];

    let mut clock = FpsClock::new(1000);

    let mut tick: u128 = 0; 

    'main_loop: loop {

        match engine::update(&mut event_pump, &settings, &mut grid, &mut cells_to_render, &mut organisms)? {
            EngineState::Continue(new_orgs) => organisms = new_orgs,
            EngineState::Exit => break 'main_loop,
        }

        renderer::render(&mut canvas, &cells_to_render, &grid, &settings)?;
        cells_to_render.clear();

        let delta = clock.tick();

        if tick % 60 == 0 {
            println!("{}", 1_000_000_000 as f32 / delta);
        }

        tick += 1
    }

    Ok(())
}
