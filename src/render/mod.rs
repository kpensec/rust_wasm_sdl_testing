extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::render::{Canvas, RenderTarget, TextureCreator};
use sdl2::video::{Window, WindowContext};

macro_rules! color_rgb(
    ($red: expr, $green: expr, $blue: expr) => (
        Color{
            r: $red as u8,
            g: $green as u8,
            b: $blue as u8,
            a: 0,
        }
    )
);

static CLEAR_COLOR : Color = color_rgb!(0, 0, 0);

pub struct  RenderContext<T: RenderTarget> {
    canvas: Canvas<T>,
    texture_creator: TextureCreator<WindowContext>
}

impl RenderContext<Window> {
    pub fn from_window(window: Window) -> RenderContext<Window> {
        let canvas = window.into_canvas()
            .build()
            .unwrap();
        let texture_creator = canvas.texture_creator();

        RenderContext::<Window>{
            canvas: canvas,
            texture_creator: texture_creator,
        }

    }
}

impl<T: RenderTarget> RenderContext<T> {
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(CLEAR_COLOR);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn display_cell(&mut self, row: i32, col: i32, cell_size: u32, color: Color) -> Result<(), String> {
        let x = cell_size as i32 * row;
        let y = cell_size as i32 * col;
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(x, y, cell_size, cell_size))?;
        Ok(())
    }
}
//pub fn draw_square<T: RenderTarget>(r: &mut Canvas<T>, x: f32, y: f32, s: f32, color: Color) {
//
//}

