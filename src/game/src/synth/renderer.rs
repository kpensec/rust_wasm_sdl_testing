
use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::render::{Canvas, RenderTarget};

const CELL_SIZE : i32 = 32;

fn display_cell<T: RenderTarget>(r: &mut Canvas<T>, row: i32, col: i32) -> Result<(), String> {
    let x = CELL_SIZE * row;
    let y = CELL_SIZE * col;
    let cell_color = Color::RGB(200,222,128);

    r.set_draw_color(cell_color);
    r.fill_rect(Rect::new(x, y, CELL_SIZE as u32, CELL_SIZE as u32))?;

}
