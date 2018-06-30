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
        freq: Some(22_050),
        channels: Some(2),
        samples: Some(512) 
    };

    let device = audio_subsystem.open_queue::<f32,_>(None, &desired_spec)
        .unwrap();
    device.resume();

    let mut synthesizer = Synthesizer::new(device.spec().freq);
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

    let mut x = 0.0;
    let mut y = 0.0;
    let mut vx = 10.0;
    let mut vy = 10.0;
    let mut _timer_subsystem = sdl_context.timer()
        .unwrap();
   
    // let mut lastFrameTime: u32 = 0;
    println!("device specs: {:?}", device.spec());
    let main_loop = || {
        // let mut synthesizer = &mut synthesizer;
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
                Event::KeyDown { keycode: Some(Keycode::F2), ..} => {
                    println!("queue size: {}", device.size());
                },
                //Event::KeyDown { keycode: Some(Keycode::F2), ..} => {
                //    let mut lock = device.lock();
                //    print!("info\nenvelop: {}\nvolume: {}", lock.envelop, lock.volume);
                //},
                Event::KeyDown { keycode: Some(Keycode::KpEnter), ..} => {
                    println!("volume -> {}", synthesizer.get_volume());
                },
                Event::KeyDown { keycode: Some(Keycode::KpMinus), ..} => {
                    synthesizer.set_volume(-0.1);
                },
                Event::KeyDown { keycode: Some(Keycode::KpPlus), ..} => {
                    synthesizer.set_volume(0.1);
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
        if device.size() <= 4096 * 2  {
        //audio_timer = (256) as f32 / device.spec().freq as f32;
        //    let buffer = &synthesizer.update(EPS);
        //    if buffer.is_empty() {
        //        println!("?");
        //    } else {
        //        if buffer[0] != 0.0 {
        //            println!("{:?}", buffer);
        //        }
        //    }
        //    device.queue(&buffer);
        // }
            device.queue(&synthesizer.update(EPS));
        }
        

        canvas.set_draw_color(bg_color);
        canvas.clear();

        canvas.set_draw_color(sprite_color);
        // TODO handle driver failure?
        let _ = canvas.fill_rect(rect);

        for i in 0..2 {
            let xx = 8 + (x as i32 + i);
            let yy = 8 + (y as i32 - i);
            let _ = renderer::display_cell(&mut canvas, xx, yy);
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
