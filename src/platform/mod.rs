
//{ EMSCRIPTEN_CONFIG:
#[cfg(target_os = "emscripten")]
pub mod emscripten;

#[cfg(target_os = "emscripten")]
pub fn exit_application() -> () {
    emscripten::emscripten::exit_application();
}

#[cfg(target_os = "emscripten")]
pub fn start_loop<F>(callback: F) where F: FnMut() {
    emscripten::emscripten::set_main_loop_callback(callback);
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
use std::time::Duration;

// const sleep_duration : Duration = ;

pub fn sleep() {
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
}
//}

