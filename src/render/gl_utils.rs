extern crate gl;

use gl::types::*;
use std::ffi::CString;
use std::{str, ptr};
use platform::io::read_file;

type GLH_Program = GLuint;

fn compile_shader_file(path: &str, ty: GLenum) -> GLuint {
    compile_shader(read_file(path).as_str(), ty)
}

pub fn make_program(vs_path: &str, fs_path: &str) -> GLH_Program {
    let vs = compile_shader_file(vs_path, gl::VERTEX_SHADER);
    let fs = compile_shader_file(fs_path, gl::FRAGMENT_SHADER);
    link_program(vs, fs)
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
                );
            panic!(
                "{}",
                str::from_utf8(&buf)
                .ok()
                .expect("ShaderInfoLog not valid utf8")
                );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
                );
            panic!(
                "{}",
                str::from_utf8(&buf)
                .ok()
                .expect("ProgramInfoLog not valid utf8")
                );
        }
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        program
    }
}

