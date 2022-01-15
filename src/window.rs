#![allow(dead_code)]
use std::ffi::CString;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::{ContextWrapper, PossiblyCurrent};
extern crate gl;

pub struct WindowContext {
    event_loop: EventLoop<()>,
    renderer: Renderer,
}

pub struct Renderer {
    pub window_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    shaders: Vec<gl::types::GLuint>,
}
pub struct Shader {
    id: gl::types::GLuint,
}

impl Renderer {
    pub fn new(
        context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    ) -> Result<Self, String> {
        Ok(Renderer {
            window_context: context,
            shaders: Vec::new(),
        })
    }
    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(25.0 / 255.0, 27.0 / 255.0, 28.0 / 255.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.window_context.swap_buffers().unwrap();
    }
}

impl Shader {
    /// Compile shader from source
    /// @param source - shader source
    /// @param kind - shader type veretex/fragment
    fn compile_shader(
        source: &std::ffi::CStr,
        kind: gl::types::GLuint,
    ) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }
        let mut suecess: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut suecess);
        }
        if suecess == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }

        //TODO manage shaders in renderer?
        //self.shaders.push(id);
        Ok(id)
    }

    pub fn from_source(source: &std::ffi::CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = Shader::compile_shader(source, kind)?;
        Ok(Shader { id })
    }
    pub fn vert_from_source(source: &std::ffi::CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }
    pub fn frag_from_source(source: &std::ffi::CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
						self.id = 0;
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

impl WindowContext {
    pub fn new() -> Result<Self, String> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Hello world!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        gl::load_with(|ptr| windowed_context.context().get_proc_address(ptr) as *const _);

        let win = WindowContext {
            event_loop,
            renderer: Renderer::new(windowed_context)?,
        };

        Ok(win)
    }
    pub fn run(self) -> Result<(), String> {
        self.event_loop.run(move |event, _, control_flow| {
            println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        self.renderer.window_context.resize(physical_size)
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    self.renderer.draw();
                    self.renderer.window_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
        // Ok(())
    }
}
