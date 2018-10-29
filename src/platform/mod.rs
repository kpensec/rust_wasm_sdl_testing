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
//}

