use glutin::event_loop:: EventLoop;
use glutin::{ContextWrapper, PossiblyCurrent};
extern crate gl;

pub struct WindowContext {
    pub window_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
    pub event_loop: EventLoop<()>,
    pub gl: gl::Gl,
}


impl WindowContext {
    pub fn new() -> Result<Self, failure::Error> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Hello world!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024u32, 768u32));

        let windowed_context = glutin::ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        let gl = gl::Gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

        let win = WindowContext {
            event_loop,
            window_context: windowed_context,
            gl: gl.clone(),
        };

        Ok(win)
    }
/*    pub fn run(self) -> Result<(), String> {
        self.event_loop.run(move |event, _, control_flow| {
            //println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        self.window_context.resize(physical_size);
                        unsafe{self.gl.Viewport(0,0,physical_size.width as i32, physical_size.height as i32);}
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    self.renderer.draw();
                    self.window_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
        // Ok(())
    }*/
}
