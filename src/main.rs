use std::time::SystemTime;

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
        mut cells_to_render,
        mut organisms,
        mut organisms_shadow
    ) = engine::init()?;

    let mut canvas = renderer::init(window)?;

    let mut clock = FpsClock::new(1000);

    let mut tick: u128 = 0;

    let mut last_update_time = SystemTime::now();

    let mut delta_time: f64;

    'main_loop: loop {
        match engine::update(
            &mut event_pump,
            &settings,
            &mut grid,
            &mut cells_to_render,
            &mut organisms,
            &mut organisms_shadow
        )? {
            | EngineState::Continue() => (),
            | EngineState::Exit => break 'main_loop,
        }

        renderer::render(
            &mut canvas,
            &cells_to_render,
            &grid,
            &settings,
        )?;
        cells_to_render.clear();

        clock.tick();

        delta_time = SystemTime::now()
            .duration_since(last_update_time)
            .map_err(|err| err.to_string())?
            .as_millis() as f64;
        last_update_time = SystemTime::now();

        if tick % 1000 == 0 {
            println!(
                "{}",
                1000 as f64 / delta_time
            );
        }

        tick += 1
    }

    Ok(())
}