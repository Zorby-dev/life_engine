use sdl2::{render::Canvas, video::Window};

use crate::{
    grid::Grid,
    settings::Settings,
    util::{pos_to_rect, CellsToRender, Vector},
};

pub fn init(window: Window) -> Result<Canvas<Window>, String> {
    let canvas = window
        .into_canvas()
        .build()
        .map_err(|err| err.to_string())?;

    Ok(canvas)
}

pub fn render<'a>(
    canvas: &mut Canvas<Window>,
    cells_to_render: &CellsToRender,
    grid: &Grid,
    settings: &Settings,
) -> Result<(), String> {
    for pos in cells_to_render {
        let cell = grid.get(pos).unwrap();

        let pos = Vector::new(
            cell.pos.x * settings.cell_size.x,
            cell.pos.y * settings.cell_size.y,
        );

        canvas.set_draw_color(cell.state.color());
        canvas.fill_rect(
            pos_to_rect(
                &pos,
                &settings.cell_size,
            ),
        )?;
    }

    canvas.present();

    Ok(())
}
