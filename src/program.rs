use glium;
use glium::glutin;
use std;

pub struct Program {
	_display: glium::Display,
	events_loop: glutin::EventsLoop,
}

impl Program {
	pub fn new(display: glium::Display, events_loop: glutin::EventsLoop) -> Self {
		Program {
			_display: display,
			events_loop,
		}
	}

	pub fn main_loop(&mut self) -> ! {
		loop {
			self.handle_events();
			self.render();
		}
	}

	fn render(&self) {}

	fn handle_events(&mut self) {
		self.events_loop.poll_events(|event| match event {
			glutin::Event::WindowEvent { event, .. } => match event {
				glutin::WindowEvent::CloseRequested => std::process::exit(0),
				_ => {}
			},
			_ => {}
		})
	}
}
