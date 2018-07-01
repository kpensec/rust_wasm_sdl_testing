extern crate sdl2;

mod utils;
mod synth;
mod platform;
mod renderer;

use sdl2::pixels::Color;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};
use sdl2::audio::{AudioSpecDesired};
use sdl2::render::{TextureQuery};

use std::collections::{HashSet, HashMap};
use std::path::Path;

use synth::Synthesizer;

static SCREEN_WIDTH : u32 = 800;
static SCREEN_HEIGHT : u32 = 600;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (SCREEN_WIDTH as i32 - w) / 2;
    let cy = 0; // (SCREEN_HEIGHT as i32 - h) / 2;
    rect!(cx, cy, w, h)
}

// TODO use a bmp texture and a better text handling system
fn render_text(target: &mut sdl2::render::Canvas<sdl2::video::Window>, text: &str, font: &sdl2::ttf::Font) {
    let surface = font.render(text)
        .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
    let texture_creator = target.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    let TextureQuery { width, height, .. } = texture.query();
    let padding = 64;
    let text_box = get_centered_rect(width/2, height/2, SCREEN_WIDTH - padding, SCREEN_HEIGHT - padding);
    target.copy(&texture, None, Some(text_box)).unwrap();
}

fn load_font<'l>(ttf_context: &'l sdl2::ttf::Sdl2TtfContext, filename: &str) -> sdl2::ttf::Font<'l, 'static> {
    let font_path: &Path = Path::new(filename);
    ttf_context.load_font(font_path, 128).unwrap()
}

fn main() {
    let sdl_context = sdl2::init()
        .unwrap();

    let video_subsystem = sdl_context.video()
        .unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let audio_subsystem = sdl_context.audio()
        .unwrap();

    let desired_spec = AudioSpecDesired {
        //freq: Some(44_100),
        freq: Some(22_050),
        channels: Some(2),
        samples: None
    };

    let mut device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        println!("{:?}", spec);
        Synthesizer::new(spec.freq)
    }).unwrap();

    device.resume();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    // TODO look for the ressource manager example on sdl2-rust github
    let font = load_font(&ttf_context, "fonts/main_font.otf");

    let mut event_pump = sdl_context.event_pump()
        .unwrap();

    let sprite_color = Color::RGB(255, 255, 255);
    let bg_color = Color::RGB(0, 0, 128);
    let mut rect = Rect::new(10, 10, 10, 10);
    let mut prev_keys = HashSet::new();
    let mut keyboard_notes = HashMap::new();

    keyboard_notes.insert(Keycode::Q, 0 as usize);
    keyboard_notes.insert(Keycode::S, 1 as usize);
    keyboard_notes.insert(Keycode::D, 2 as usize);
    keyboard_notes.insert(Keycode::F, 3 as usize);
    keyboard_notes.insert(Keycode::J, 4 as usize);
    keyboard_notes.insert(Keycode::K, 5 as usize);
    keyboard_notes.insert(Keycode::L, 6 as usize);
    keyboard_notes.insert(Keycode::M, 7 as usize);
    keyboard_notes.insert(Keycode::W, 8 as usize);
    keyboard_notes.insert(Keycode::X, 9 as usize);
    keyboard_notes.insert(Keycode::C, 10 as usize);
    keyboard_notes.insert(Keycode::V, 11 as usize);
    keyboard_notes.insert(Keycode::N, 12 as usize);

    let mut x = 0.0;
    let mut y = 0.0;
    let mut vx = 10.0;
    let mut vy = 10.0;

    let main_loop = || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    platform::exit_application();
                    println!("Application exited!");
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    rect.x -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    rect.x += 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    rect.y -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    rect.y += 10;
                },
                Event::KeyDown { keycode: Some(Keycode::F1), ..} => {
                    let mut lock = device.lock();
                    (*lock).toggle_audio();
                },
                Event::KeyDown { keycode: Some(Keycode::F2), ..} => {
                },
                //Event::KeyDown { keycode: Some(Keycode::F2), ..} => {
                //    let mut lock = device.lock();
                //    print!("info\nenvelop: {}\nvolume: {}", lock.envelop, lock.volume);
                //},
                Event::KeyDown { keycode: Some(Keycode::KpEnter), ..} => {
                    let mut lock = device.lock();
                    println!("volume -> {}", (*lock).get_volume());
                },
                Event::KeyDown { keycode: Some(Keycode::KpMinus), ..} => {
                    let mut lock = device.lock();
                    (*lock).set_volume(-0.1);

                },
                Event::KeyDown { keycode: Some(Keycode::KpPlus), ..} => {
                    let mut lock = device.lock();
                    (*lock).set_volume(0.1);
                },
                _ => {}
            }
        }

        // TODO wrap me?
        let keys = event_pump.keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        for key in new_keys {
            match keyboard_notes.get(&key) {
                Some(i) => {
                    let mut lock = device.lock();
                    (*lock).start_note(*i);
                }
                _ => {}
            }
        }

        for key in old_keys {
            match keyboard_notes.get(&key) {
                Some(i) => {
                    let mut lock = device.lock();
                    (*lock).release_note(*i);
                }
                _ => {}
            }
        }

        prev_keys = keys;

        canvas.set_draw_color(bg_color);
        canvas.clear();

        canvas.set_draw_color(sprite_color);
        let _ = canvas.fill_rect(rect);

        for i in 0..2 {
            let xx = 8 + (x as i32 + i);
            let yy = 8 + (y as i32 - i);
            let _ = renderer::display_cell(&mut canvas, xx, yy);
        }
        const EPS : f32 = 1.0/30.0;
        x = x + vx * EPS;
        y = y + vy * EPS;
        if x > 0.0 { vx = -2.0;}
        if x < -10.0 { vx = 2.0;}
        if y > 32.0 { vy = -5.0 }
        if y < 5.0 { vy = 5.0}

        // !
        render_text(&mut canvas, "foobar", &font);

        canvas.present();

        platform::sleep();

    };

    platform::start_loop(main_loop);
}
