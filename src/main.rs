extern crate sdl2;
extern crate rand;
extern crate gl;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate libloading;

mod ui;
mod utils;
mod synth;
mod platform;
mod render;

// TODO lookup for some virtual fs crates?
//

use std::collections::{HashSet, HashMap};
use std::{mem, str, ptr};
use std::ffi::CString;
use std::os::raw::c_void;


// TODO should be on the platform layer code
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioSpecDesired};

use gl::types::*;

use utils::{Vec2, Unit, Newable};
//use render::{/*Scene, Sprite, RenderContext*/};
use render::gl_utils::make_program;
use synth::Synthesizer;

// TODO read from config file!
const NAME: &'static str = "rust-sdl2 demo: Video";

const SCREEN_SIZE: Vec2 = Vec2 {
    x: 1024 as Unit,
    y: 768 as Unit
};
// mod RessourceManager
fn load_obj(filepath: &str) -> Vec<f32> {
    let mut result : Vec<f32> = Vec::<f32>::new();

    platform::io::load_asset(filepath, |line: &String| {
        match line.get(0..1) {
            Some("#") => return (),
            _ => {}

        }

        for s in line.trim().split(" ") {
            if s != "" {
                result.push(s.parse::<f32>().unwrap());
            }
        }
    });

    result
}

struct SDLSys {
   context: sdl2::Sdl,
   video: sdl2::VideoSubsystem,
   window: sdl2::video::Window,
   gl_context: sdl2::video::GLContext,
   audio: sdl2::AudioSubsystem,
   event: sdl2::EventPump,
   timer: sdl2::TimerSubsystem,
}

fn window_init(video_subsystem: &mut sdl2::VideoSubsystem) -> sdl2::video::Window {
    { // gl init version
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 0);
    }


    video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate);
    video_subsystem.window(NAME, SCREEN_SIZE.x as u32, SCREEN_SIZE.y as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap()
}

// TODO should be in platform ingnite function?
fn init() -> SDLSys {
    let sdl_context = sdl2::init()
        .unwrap();

    let mut video_subsystem = sdl_context.video()
        .unwrap();

    let window_subsystem = window_init(&mut video_subsystem);

    let audio_subsystem = sdl_context.audio()
        .unwrap();

    let timer_subsystem = sdl_context.timer()
        .unwrap();

    let gl_context = window_subsystem.gl_create_context()
        .expect("Couldn't create GL context");

    gl::load_with(
        |name| video_subsystem.gl_get_proc_address(name) as *const _
        );

    let event_subsystem = sdl_context.event_pump()
        .unwrap();

    SDLSys {
        context: sdl_context,
        video: video_subsystem,
        window: window_subsystem,
        gl_context: gl_context,
        audio: audio_subsystem,
        timer: timer_subsystem,
        event: event_subsystem,
    }

}


pub fn load_ressource<RType, F>(filepath: &str, mut func: F) -> RType
    where F : FnMut(&String, &mut RType) -> (),
          RType : Newable {

    let mut ressource = RType::new();
    platform::io::load_asset(filepath, |line: &String| {
        // skip comment
        match line.get(0..1) {
            Some("#") => return,
            _ => {}
        }

        func(line, &mut ressource);
    });
    ressource
}

type Keymap = HashMap<sdl2::keyboard::Keycode, usize>;

impl Newable for Keymap {
    fn new() -> Self {
        HashMap::new()
    }
}

pub fn load_keybind(filepath: &str) -> Keymap {
    load_ressource(filepath, |line: &String, keymap: &mut Keymap| {
        let array : Vec<&str> = line.trim().split(" ").collect();
        let keyname = array[0];
        let note = array[1];

        // TODO change this with a logger function?
        println!("{}, {}", keyname, note);

        keymap.insert(
            sdl2::keyboard::Keycode::from_name(keyname).unwrap(),
            note.parse::<i32>().unwrap() as usize
            );
    })
}

pub struct DataBufferTest {
    pub x: i32
}

struct Library {
    pub path: String,
    pub handle: libloading::Library
}

impl Library {
    pub fn new(filepath: &str) -> Self {
        let handle = libloading::Library::new(filepath).unwrap();
        Library {
            path: filepath.to_string(),
            handle: handle
        }
    }

    pub fn reload(lib: &mut Self) -> Self {
        let filepath = lib.path.clone();
        drop(&lib.handle);
        println!("reloading library...");
        Self::new(&filepath)
    }

    pub fn func(&self, buffer: &mut DataBufferTest) {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn(&mut DataBufferTest) -> ()> = self.handle.get(b"hello_world").unwrap();
            println!("sym loaded!");
            func(buffer);
            println!("func called!");
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ProgramDef {
    vertex: String,
    fragment: String,
}

fn main() {
    let mut buffer = DataBufferTest{x: 1};
    let library_paths = ["dylibtest/target/release/libdylibtest1.so", "dylibtest/target/release/libdylibtest2.so"];
    let mut library_index = 0;
    let mut library = Library::new(library_paths[library_index]);
    library_index = 1;


    let mut systems = init();

    let mut last_frame_time = 0 as u32;
    // let mut sprite = Sprite::new(Vec2::new(10.0, 10.0), Vec2::new(100.0, 100.0));

    let desired_spec = AudioSpecDesired {
        freq: Some(2*22_050),
        channels: Some(2),
        samples: None
    };

    let instrument : synth::Instrument =
      serde_yaml::from_str(&platform::io::read_file("assets/instrument/test.yml")).unwrap();
    println!("instr: {:?}", instrument);

    let mut device = systems.audio.open_playback(None, &desired_spec, |spec| {
        println!("{:?}", spec);
        Synthesizer::new(spec.freq, instrument)
    }).unwrap();

    device.resume();

    // TODO rework keyboard handling API!
    let mut prev_keys = HashSet::new();
    let keyboard_notes = load_keybind("assets/keybind.txt");

    let mut frame_count = 10;
    let mut frame_count_time = 0.0;
    let mut frame_per_sec = 60.0;

    let vertex_data = load_obj("assets/triangle.obj");

    // TODO abstract gl object loading!

    // TODO gl program in yaml seems ok
    let program_def: ProgramDef = serde_yaml::from_str(&platform::io::read_file("assets/sprite.yml")).unwrap();
    println!("{:?}", program_def);

    //println!("loaded instrument: {:?}", instrument.clone());

    let program = make_program(&program_def.vertex, &program_def.fragment);
    let mut vbo = 0;
    let mut vao = 0;
    unsafe { // uploading gl data
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertex_data.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
        let color_attr = gl::GetAttribLocation(program, CString::new("color").unwrap().as_ptr());

        let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 3, gl::FLOAT, gl::FALSE as GLboolean, stride, ptr::null());
        gl::EnableVertexAttribArray(color_attr as GLuint);
        gl::VertexAttribPointer(color_attr as GLuint, 3, gl::FLOAT, gl::FALSE as GLboolean, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    }

    // TODO meh dirty
    let tri_number = vertex_data.len() as i32 / 6;

    let main_loop = || {
        let new_frame_time = systems.timer.ticks();
        let eps = (new_frame_time - last_frame_time) as f32 / 1000.0;
        last_frame_time = new_frame_time;

        for event in systems.event.poll_iter() {
            // TODO use a message queue to dispatch event to subsystems
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
                    // block_size = utils::clamp(block_size >> 1, 1, 64);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    // block_size = utils::clamp(block_size << 1, 1, 64);
                },
                Event::KeyDown { keycode: Some(Keycode::F1), ..} => {
                    let mut lock = device.lock();
                    (*lock).toggle_audio();
                },
                Event::KeyDown { keycode: Some(Keycode::F2), ..} => {
                    let path = library_paths[library_index];
                    library_index = (library_index + 1) % 2;
                    println!("loading library: {}", path);
                    library = Library::new(&path);

                },
                Event::KeyDown { keycode: Some(Keycode::F3), ..} => {
                    library.func(&mut buffer);
                }
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
        {
        let keys = systems.event.keyboard_state()
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
        }

        // universe.tick(eps);

        // rendering:
        // render_context.clear();
        // canvas.set_draw_color(sprite_color);

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
        //universe.render(&mut render_context);
        //{
        //    let txQuery = _font_map.query();
        //    let src_rect = rect!(0, 0, txQuery.width, txQuery.height);
        //    let dst_rect = rect!(0, 0, txQuery.width * 2, txQuery.height * 2);
        //    println!("{:?}",txQuery);
        //    canvas.copy(&_font_map, src_rect, dst_rect);
        //}
        //sprite.render(&mut render_context);



        //ui.window(im_str!("Hello World"))
        //    .size((100.0,100.0), imgui::ImGuiCond::FirstUseEver)
        //    .build(|| {
        //        ui.text(im_str!("hello"));
        //    });
        // window.gl_set_context_to_current();
            // render_context.begin_gl();
        unsafe {
            systems.window.gl_set_context_to_current();
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, tri_number);
        }

        systems.window.gl_swap_window();
        // render_context.present();

        platform::sleep();
    };

    platform::start_loop(main_loop);
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
