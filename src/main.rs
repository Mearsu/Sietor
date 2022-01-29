#[macro_use]
extern crate render_derive;
#[macro_use]
extern crate failure;
extern crate vec_2_10_10_10;

use debug::failure_to_string;
use failure::err_msg;
use glutin::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow, dpi::PhysicalSize,
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use resources::Resources;
use std::path::Path;

mod debug;
mod renderer;
mod resources;
mod window;
mod triangle;

fn main() {
    env_logger::init();
    if let Err(e) = sietor() {
        error!("{}", failure_to_string(e));
    }
}

fn sietor() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
   let win = window::WindowContext::new().map_err(err_msg)?;

    let shader_program = renderer::GlProgram::from_res(&win.gl, &res, "shaders/triangle")?;
    let mut viewport = renderer::Viewport::for_window(900,700);
    shader_program.set_used();
    let triangle = triangle::Triangle::new(&res, &win.gl)?;

    //viewport.set_used(&win.gl);
    win.event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                trace!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent{
                event: WindowEvent::Resized(size),
                ..
            } =>{
                viewport.update_size(size.width as i32, size.height as i32);
                viewport.set_used(&win.gl);
            }
            Event::MainEventsCleared => {
                // Application update code.
                triangle.draw(&win.gl);
                win.window_context.swap_buffers().unwrap();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
                //
                // Could be usefull for gui
            }
            _ => (),
        }
    });
//    Ok(())
}
