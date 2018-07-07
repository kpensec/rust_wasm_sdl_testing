use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::render::{Canvas, RenderTarget};


pub fn display_cell<T: RenderTarget>(r: &mut Canvas<T>, row: i32, col: i32, cell_size: u32, color : Color) -> Result<(), String> {
    let x = cell_size as i32 * row;
    let y = cell_size as i32 * col;
    r.set_draw_color(color);
    r.fill_rect(Rect::new(x, y, cell_size, cell_size))?;
    Ok(())
}
