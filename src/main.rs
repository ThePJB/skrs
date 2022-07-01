mod lib;
mod application;
mod renderer;
mod manifest;
mod level;
mod instance;
mod world;
mod session;
mod level_repository;

use application::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use std::env;




fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let event_loop = glutin::event_loop::EventLoop::new();
    let mut application = Application::new(&event_loop);
    
    event_loop.run(move |event, _, control_flow| {
        application.handle_event(&event);
        match event {
            Event::LoopDestroyed |
            Event::WindowEvent {event: WindowEvent::CloseRequested, ..}
            => {
                *control_flow = ControlFlow::Exit;
            },
            _ => (),
        }
    });
}