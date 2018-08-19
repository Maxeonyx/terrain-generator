extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate image;

use glium::glutin;

mod images;
mod program;
mod shaders;

fn init_glium() -> (glium::Display, glutin::EventsLoop) {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("COSC422 Assignment 1 - Terrain");
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    (display, events_loop)
}

fn main() -> ! {
    let (display, events_loop) = init_glium();
    let shaders = shaders::init_shaders(&display);
    let images = images::init_images(&display);
    let mut program = program::Program::new(display, events_loop, shaders, images);
    program.main_loop()
}
