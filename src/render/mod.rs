extern crate sdl2;

use utils::Vec2;

use sdl2::rect::{Rect};
use sdl2::render::{Canvas, TextureCreator};
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

mod scene;
mod sprite;
pub mod gl_utils;

pub type Color = sdl2::pixels::Color;
pub type Scene = scene::Scene;
pub type Sprite = sprite::Sprite;
pub use sdl2::render::RenderTarget;

static CLEAR_COLOR : Color = color_rgb!(0, 0, 0);

pub struct RenderContext<T: RenderTarget> {
    canvas: Canvas<T>,
    texture_creator: TextureCreator<WindowContext>,
    // sprite_batch: Vec<SpriteVertexData>,
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (k,v) in sdl2::render::drivers().enumerate() {
        if v.name == "opengl" {
            return Some(k as u32);
        }
    }
    None
}

impl RenderContext<Window> {
    pub fn from_window(window: Window) -> RenderContext<Window> {
        let canvas = window.into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        let texture_creator = canvas.texture_creator();

        RenderContext::<Window>{
            canvas: canvas,
            texture_creator: texture_creator,
        }

    }

    pub fn present(&mut self) {
        self.canvas.window().gl_swap_window();
    }

    pub fn window(&mut self) -> &Window {
        self.canvas.window()
    }
}

impl<T: RenderTarget> RenderContext<T> {

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(CLEAR_COLOR);
        self.canvas.clear();
    }


    pub fn display_cell(&mut self, row: i32, col: i32, cell_size: u32, color: Color) -> Result<(), String> {
        let x = cell_size as i32 * row;
        let y = cell_size as i32 * col;
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(Rect::new(x, y, cell_size, cell_size))?;
        Ok(())
    }
    pub fn draw_rect(&mut self, p: Vec2, s: Vec2) -> Result<(), String> {

        let x = (p.x - s.x * 0.5) as i32;
        let y = (p.y - s.y * 0.5) as i32;
        let w = s.x as u32;
        let h = s.y as u32;

        self.canvas.set_draw_color(color_rgb!(255,0,0));
        self.canvas.fill_rect(Rect::new(x, y, w, h))?;
        Ok(())
    }
}

impl RenderContext<Window> {
    pub fn begin_gl(&mut self) {
        self.canvas.window().gl_set_context_to_current().unwrap();
    }
}

pub trait Renderer {
    fn render<T: RenderTarget>(&self, ctx: &mut RenderContext<T>);
}

