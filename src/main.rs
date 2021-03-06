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

use std::collections::{HashSet, HashMap};

use synth::Synthesizer;

fn main() {
    let sdl_context = sdl2::init()
        .unwrap();

    let video_subsystem = sdl_context.video()
        .unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let audio_subsystem = sdl_context.audio()
        .unwrap();

    // let desired_spec = AudioSpecDesired {
    //     freq: Some(DEFAULT_FREQ),
    //     channels: Some(DEFAULT_CHANNEL_NUMBER),  // mono
    //     samples: Some(DEFAULT_SAMPLE_SIZE) // default sample size
    // };
    let desired_spec = AudioSpecDesired {
        freq: None,
        channels: None,
        samples: None
    };

    let device = audio_subsystem.open_queue::<f32,_>(None, &desired_spec)
        .unwrap();

    let synthesizer = Synthesizer::new();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump()
        .unwrap();

    let sprite_color = Color::RGB(255, 255, 255);
    let bg_color = Color::RGB(0, 0, 128);
    let mut rect = Rect::new(10, 10, 10, 10);
    // let mut phase_idx : usize = 0;
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
    // keyboard_notes.insert(Keycode::Comma, 13 as usize);
    // keyboard_notes.insert(Keycode::Semicolon, 14 as usize);
    // keyboard_notes.insert(Keycode::Colon, 15 as usize);
    // keyboard_notes.insert(Keycode::A, 16 as usize);
    // keyboard_notes.insert(Keycode::Z, 17 as usize);
    // keyboard_notes.insert(Keycode::E, 18 as usize);
    // keyboard_notes.insert(Keycode::R, 19 as usize);
    // keyboard_notes.insert(Keycode::U, 20 as usize);
    // keyboard_notes.insert(Keycode::I, 21 as usize);
    // keyboard_notes.insert(Keycode::O, 22 as usize);
    // keyboard_notes.insert(Keycode::P, 23 as usize);
    // keyboard_notes.insert(Keycode::Num1, 24 as usize);
    // keyboard_notes.insert(Keycode::Num2, 25 as usize);
    // keyboard_notes.insert(Keycode::Num3, 26 as usize);
    // keyboard_notes.insert(Keycode::Num4, 27 as usize);
    // keyboard_notes.insert(Keycode::Num7, 28 as usize);
    // keyboard_notes.insert(Keycode::Num8, 29 as usize);
    // keyboard_notes.insert(Keycode::Num9, 30 as usize);
    // keyboard_notes.insert(Keycode::Num0, 31 as usize);

    let mut x = 0.0;
    let mut y = 0.0;
    let mut vx = 10.0;
    let mut vy = 10.0;
    let mut timer_subsystem = sdl_context.timer()
        .unwrap();
   
    // let mut lastFrameTime: u32 = 0;

    let main_loop = || {
        // let time = timer_subsystem.ticks() as f32 / 1000.0;
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
                    synthesizer.toggle_audio();
                },
                //Event::KeyDown { keycode: Some(Keycode::F2), ..} => {
                //    let mut lock = device.lock();
                //    print!("info\nenvelop: {}\nvolume: {}", lock.envelop, lock.volume);
                //},
                Event::KeyDown { keycode: Some(Keycode::KpEnter), ..} => {
                    let lock = device.lock();
                    println!("volume -> {}", lock.volume);
                },
                Event::KeyDown { keycode: Some(Keycode::KpMinus), ..} => {
                    let mut lock = device.lock();
                    (*lock).change_volume(-0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::KpPlus), ..} => {
                    let mut lock = device.lock();
                    (*lock).change_volume(0.1);
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
                    synthesizer.start_note(*i);
                }
                _ => {}
            }
        }

        for key in old_keys {
            match keyboard_notes.get(&key) {
                Some(i) => {
                    synthesizer.release_note(*i);
                }
                _ => {}
            }
        }

        prev_keys = keys;
        // TODO use real eps
        const EPS: f32 = 1.0 / 30.0;

        synthesizer.update(EPS);
        canvas.set_draw_color(bg_color);
        canvas.clear();

        canvas.set_draw_color(sprite_color);
        // TODO handle driver failure?
        let _ = canvas.fill_rect(rect);

        for i in 0..2 {
            let xx = 8 + (x as i32 + i);
            let yy = 8 + (y as i32 - i);
            renderer::display_cell(&mut canvas, xx, yy);
        }
        x = x + vx * EPS;
        y = y + vy * EPS;
        if x > 0.0 { vx = -2.0;} 
        if x < -10.0 { vx = 2.0;}
        if y > 32.0 { vy = -5.0 }
        if y < 5.0 { vy = 5.0}

        canvas.present();

        platform::sleep();

    };

    platform::start_loop(main_loop);
}
