extern crate sdl2;


use sdl2::pixels::Color;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};
use sdl2::audio::{AudioSpecDesired};

use std::time::Duration;
use std::collections::{HashSet, HashMap};
use utils::clamp;

// TODO put platform dependent code and import in a single module!
#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[cfg(target_os = "emscripten")]
pub fn exit_application() -> () {
    emscripten::emscripten::exit_application();
}


#[cfg(not(target_os = "emscripten"))]
static mut quit: bool = false;

#[cfg(not(target_os = "emscripten"))]
pub fn exit_application() -> () {
    unsafe {
        quit = true;
    }
}


mod utils;
mod synth;

// const DEFAULT_FREQ : i32 = None;
// const DEFAULT_CHANNEL_NUMBER : u8 = 2;
// const DEFAULT_SAMPLE_SIZE : u16 = 2048;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();
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
    let mut device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        println!("{:?}", spec);
        synth::SquareWave::new(spec.freq)
    }).unwrap();

    synth::play(&mut device);
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump()
        .unwrap();

    let sprite_color = Color::RGB(255, 255, 255);
    let bg_color = Color::RGB(0, 0, 128);
    let mut rect = Rect::new(10, 10, 10, 10);
    // let mut phase_idx : usize = 0;
    synth::play(&mut device);
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
    keyboard_notes.insert(Keycode::Comma, 13 as usize);
    keyboard_notes.insert(Keycode::Semicolon, 14 as usize);
    keyboard_notes.insert(Keycode::Colon, 15 as usize);
    keyboard_notes.insert(Keycode::A, 24 as usize);
    keyboard_notes.insert(Keycode::Z, 19 as usize);
    keyboard_notes.insert(Keycode::E, 20 as usize);
    keyboard_notes.insert(Keycode::R, 22 as usize);

    let mut main_loop = || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    exit_application();
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
                //Event::KeyDown { keycode: Some(Keycode::Q), ..} => {
                //    let mut lock = device.lock();
                //    (*lock).start_note(0);
                //},
                //Event::KeyUp { keycode: Some(Keycode::Q), ..} => {
                //    let mut lock = device.lock();
                //    (*lock).release_note(0);
                //},
                Event::KeyDown { keycode: Some(Keycode::T), ..} => {
                    synth::play(&mut device);
                },
                Event::KeyDown { keycode: Some(Keycode::Y), ..} => {
                    let lock = device.lock();
                    println!("volume -> {}", lock.volume);
                },
                Event::KeyDown { keycode: Some(Keycode::O), ..} => {
                    let mut lock = device.lock();
                    (*lock).change_volume(-0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::P), ..} => {
                    let mut lock = device.lock();
                    (*lock).change_volume(0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::T), ..} => {
                    // let mut lock = device.lock();
                    // device.spec().samples = clamp(device.spec().samples << 1, 32, 4096);
                },
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                    // let mut lock = device.lock();
                    // device.spec().samples = clamp(device.spec().samples >> 1, 32, 4096);
                },
                //Event::KeyDown { keycode: Some(Keycode::I), ..} => {
                //    let mut lock = device.lock();
                //    (*lock).change_tone(phase_idx, 5);
                //},
                //Event::KeyDown { keycode: Some(Keycode::U), ..} => {
                //    let mut lock = device.lock();
                //    (*lock).change_tone(phase_idx, -5);
                //},
                // Event::KeyDown { keycode: Some(Keycode::L), ..} => {
                //     phase_idx = (phase_idx + 1) % 4
                // },
                // Event::KeyDown { keycode: Some(Keycode::H), ..} => {
                //     phase_idx = (phase_idx - 1) % 4
                // },
                // Event::KeyDown { keycode: Some(Keycode::J), ..} => {
                //     let mut lock = device.lock();
                //     (*lock).change_phase(phase_idx, -0.005);
                // },
                // Event::KeyDown { keycode: Some(Keycode::K), ..} => {
                //     let mut lock = device.lock();
                //     (*lock).change_phase(phase_idx, 0.005);
                // },
                _ => {}
            }
        }

        let keys = event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        // if !new_keys.is_empty() || !old_keys.is_empty() {
        //     println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
        // }

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
        // TODO RTFM for this ret val...
        let _ = canvas.fill_rect(rect);

        canvas.present();

        //
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

    };


    #[cfg(target_os = "emscripten")]
    emscripten::emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    'running: loop {
        main_loop();
        unsafe {
        if quit {
            break
        }}
    }
}
