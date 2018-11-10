use utils::Vec2;
/**
 * platform abstraction layer
 */

// io module
pub mod io;

//{ EMSCRIPTEN_CONFIG:
#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[cfg(target_os = "emscripten")]
pub fn exit_application() -> () {
    emscripten::exit_application();
}

#[cfg(target_os = "emscripten")]
pub fn start_loop<F>(callback: F) where F: FnMut() {
    emscripten::set_main_loop_callback(callback);
}
//}

//{ NON_EMSCRIPTEN_CONFIG:
#[cfg(not(target_os = "emscripten"))]
static mut QUIT: bool = false;

#[cfg(not(target_os = "emscripten"))]
pub fn exit_application() -> () {
    unsafe { QUIT = true; }
}

#[cfg(not(target_os = "emscripten"))]
pub fn start_loop<F>(mut main_loop: F) where F: FnMut() {
    'running: loop {
        main_loop();
        unsafe { if QUIT { break 'running } }
    }
}
//}

//{ SHARED_CONFIG:

pub fn sleep() {
    use std::time::Duration;
    const DURATION : u32 = 1_000_000u32 / 60;
    ::std::thread::sleep(Duration::new(0, DURATION));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowCfg {
  pub name: String,
  pub size: Vec2
}

pub struct BackendSystems {
   pub context: sdl2::Sdl,
   pub video: sdl2::VideoSubsystem,
   pub window: sdl2::video::Window,
   pub gl_context: sdl2::video::GLContext,
   pub audio: sdl2::AudioSubsystem,
   pub event: sdl2::EventPump,
   pub timer: sdl2::TimerSubsystem,
}

fn window_init(video_subsystem: &mut sdl2::VideoSubsystem, cfg: WindowCfg) -> sdl2::video::Window {

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);

    video_subsystem.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate);

    let window =
        video_subsystem
        .window(&cfg.name, cfg.size.x as u32, cfg.size.y as u32)
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    gl::load_with(|name| unsafe {
        std::mem::transmute(video_subsystem.gl_get_proc_address(name))
    });

    window
}

// TODO
fn audio_init(audio_subsystem: &mut sdl2::AudioSubsystem, desired_spec: sdl2::audio::AudioSpecDesired) -> (){
}

pub fn init(window_cfg: WindowCfg) -> BackendSystems {
    let sdl_context = sdl2::init()
        .unwrap();

    let mut video_subsystem = sdl_context.video()
        .unwrap();

    let window_subsystem =
        window_init(&mut video_subsystem, window_cfg);

    let audio_subsystem = sdl_context.audio()
        .unwrap();

    let timer_subsystem = sdl_context.timer()
        .unwrap();

    let gl_context = window_subsystem.gl_create_context()
        .expect("Couldn't create GL context");

    let event_subsystem = sdl_context.event_pump()
        .unwrap();

    BackendSystems {
        context: sdl_context,
        video: video_subsystem,
        window: window_subsystem,
        gl_context: gl_context,
        audio: audio_subsystem,
        timer: timer_subsystem,
        event: event_subsystem,
    }
}

//}

