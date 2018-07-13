extern crate sdl2;
extern crate rand;

mod utils;
mod synth;
mod platform;
mod render;
mod game_of_life;

use utils::{Vec2, Unit};
use render::{Scene};
use std::collections::{HashSet, HashMap};
// use std::path::Path;

use sdl2::pixels::Color;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};
use sdl2::audio::{AudioSpecDesired};
// use sdl2::video::Window;

use render::RenderContext;
use synth::Synthesizer;
use game_of_life::{Universe, Renderer};

const NAME: &str = "rust-sdl2 demo: Video";
const SCREEN_SIZE: Vec2 = Vec2{
    x: 1366 as Unit,
    y: 768 as Unit
};

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

trait TextRendering {
    fn render_text(&mut self, text: &str, font: &sdl2::ttf::Font, line_no: u32, color: Color) -> ();

    // Scale fonts to a reasonable size when they're too big (though they might look less smooth)
    fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
        let wr = rect_width as f32 / cons_width as f32;
        let hr = rect_height as f32 / cons_height as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                let h = (rect_height as f32 / wr) as i32;
                (cons_width as i32, h)
            } else {
                let w = (rect_width as f32 / hr) as i32;
                (w, cons_height as i32)
            }
        } else {
            (rect_width as i32, rect_height as i32)
        };

        let cx = 0;
        let cy = 0; // (SCREEN_HEIGHT as i32 - h) / 2;
        rect!(cx, cy, w, h)
    }
}

// impl TextRendering for Canvas<Window> {
//     fn render_text(&mut self, text: &str, font: &sdl2::ttf::Font, line_no: u32, color: Color) -> () {
//         let surface = font.render(text)
//             .blended(color).unwrap();
//         let texture_creator = self.texture_creator();
//         let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
//
//         let TextureQuery { width, height, .. } = texture.query();
//         let padding = 16;
//         let mut text_box = Self::get_centered_rect(width/2, height/2, - padding, SCREEN_HEIGHT - padding);
//         text_box.y += ((height/2+padding) * line_no) as i32;
//         text_box.x += padding as i32;
//         self.copy(&texture, None, Some(text_box)).unwrap();
//     }
// }

// fn load_font<'l>(ttf_context: &'l sdl2::ttf::Sdl2TtfContext, filename: &str, font_size: u16) -> sdl2::ttf::Font<'l, 'static> {
//     let font_path: &Path = Path::new(filename);
//     ttf_context.load_font(font_path, font_size).unwrap()
// }

fn main() {

    let sdl_context = sdl2::init()
        .unwrap();

    let video_subsystem = sdl_context.video()
        .unwrap();
    video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate);
    let window = video_subsystem.window(NAME, SCREEN_SIZE.x as u32, SCREEN_SIZE.y as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
    let mut scene = Scene::new(SCREEN_SIZE);
    let mut render_context = RenderContext::from_window(window);

    let audio_subsystem = sdl_context.audio()
        .unwrap();
    let _image_context = sdl2::image::init(sdl2::image::INIT_PNG)
        .unwrap();

    let mut timer_subsystem = sdl_context.timer()
        .unwrap();

    let mut last_frame_time = 0 as u32;

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


    //let _font_map = render_context.load_texture(Path::new("assets/font.png"))
    //    .unwrap();
    // let ttf_context = sdl2::ttf::init().unwrap();

    let mut event_pump = sdl_context.event_pump()
        .unwrap();

    // let sprite_color = Color::RGB(255, 255, 255);
    // let bg_color = Color::RGB(0, 0, 128);
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

    // let mut render_time_accumulator = 0.0;
    // let render_time_update = 10.0/1000.0;
    // let mut block_size = 16;
    let mut frame_count = 10;
    let mut frame_count_time = 0.0;
    let mut frame_per_sec = 60.0;
    let mut universe = Universe::new(0.5);
    let main_loop = || {
        let new_frame_time = timer_subsystem.ticks();
        let eps = (new_frame_time - last_frame_time) as f32 / 1000.0;
        last_frame_time = new_frame_time;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    platform::exit_application();
                    println!("Application exited!");
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    let mut lock = device.lock();
                    (*lock).switch_instrument(-1);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    /*let mut lock =*/ device.lock().switch_instrument(1);
                    // (*lock).switch_instrument(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    universe.set_time_update(0.05);
                    // block_size = utils::clamp(block_size >> 1, 1, 64);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    // block_size = utils::clamp(block_size << 1, 1, 64);
                    universe.set_time_update(-0.05);
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
        universe.tick(eps);

        // rendering:
        render_context.clear();
        //canvas.set_draw_color(sprite_color);

        {
            let lock = device.lock();
            let _volume = (*lock).get_volume();
            let _instrument = (*lock).get_instrument();

            frame_count_time += eps;
            frame_count -= 1;
            if frame_count == 0 {
                if frame_count_time == 0.0 {
                    println!("divided by zero!");
                }
                frame_count = 60;
                frame_per_sec =  frame_count as f32 / frame_count_time ;
                frame_count_time = 0.0;
            //    println!("fps: {:.*}", 2, frame_per_sec);
            }
            // canvas.render_text(&format!("fps: {:.*}", 2, frame_per_sec), &font, 0, Color::RGBA(((1.0+t)*128.0) as u8,((1.0-t) * 128.0) as u8,0,255));
            // canvas.render_text(&format!("volume: {}, red value: {}", volume, t), &font, 0, Color::RGBA(((1.0+t)*128.0) as u8,((1.0-t) * 128.0) as u8,0,255));
            // canvas.render_text(&format!("current intstrument: {}", instrument), &font, 1, Color::RGBA(255,255,0,255));
            // canvas.render_text(&format!("universe update time: {}", universe.get_time_update()), &font, 2, Color::RGBA(255,255,0,255));
        }
        //for y in 8..SCREEN_HEIGHT/block_size - 8 {
        //    for x in 8..SCREEN_WIDTH/block_size - 8 {
        //        let color = Color::RGB(
        //            (rand::random::<f32>() * 128.0 + 64.0) as u8,
        //            (rand::random::<f32>() * 128.0 + 64.0) as u8,
        //            (rand::random::<f32>() * 128.0 + 64.0) as u8,
        //            );

        //        renderer::display_cell(&mut canvas, x as i32, y as i32, block_size, color).unwrap();
        //    }
        //}
        universe.render(&mut render_context);
        //{
        //    let txQuery = _font_map.query();
        //    let src_rect = rect!(0, 0, txQuery.width, txQuery.height);
        //    let dst_rect = rect!(0, 0, txQuery.width * 2, txQuery.height * 2);
        //    println!("{:?}",txQuery);
        //    canvas.copy(&_font_map, src_rect, dst_rect);
        //}
        render_context.present();


        platform::sleep();

    };

    platform::start_loop(main_loop);
}
