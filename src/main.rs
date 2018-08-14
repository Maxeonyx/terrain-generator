extern crate cgmath;
extern crate glium;
extern crate image;

use glium::glutin;

mod program;

fn init_glium() -> (glium::Display, glutin::EventsLoop) {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("COSC422 Assignment 1 - Terrain");
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    (display, events_loop)
}

fn main() -> ! {
    let (display, events_loop) = init_glium();
    let mut program = program::Program::new(display, events_loop);
    program.main_loop()
}
