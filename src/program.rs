use glium::{self, glutin, index::PrimitiveType, Surface};
use std;

pub struct Program {
	display: glium::Display,
	events_loop: glutin::EventsLoop,
	shaders: glium::Program,
}

impl Program {
	pub fn new(
		display: glium::Display,
		events_loop: glutin::EventsLoop,
		shaders: glium::Program,
	) -> Self {
		Program {
			display,
			events_loop,
			shaders,
		}
	}

	pub fn main_loop(&mut self) -> ! {
		loop {
			self.handle_events();
			self.render();
		}
	}

	fn render(&self) {
		let vertex_buffer = {
			#[derive(Copy, Clone)]
			struct Vertex {
				position: [f32; 2],
			}

			implement_vertex!(Vertex, position);

			glium::VertexBuffer::new(
				&self.display,
				&[
					Vertex {
						position: [-0.5, -0.5],
					},
					Vertex {
						position: [0.0, 0.5],
					},
					Vertex {
						position: [0.5, -0.5],
					},
				],
			).unwrap()
		};

		// building the index buffer
		let index_buffer =
			glium::IndexBuffer::new(&self.display, PrimitiveType::TriangleStrip, &[0u16, 1, 2])
				.unwrap();

		let mut target = self.display.draw();
		target.clear_color(0.0, 1.0, 0.0, 0.0);
		target
			.draw(
				&vertex_buffer,
				&index_buffer,
				&self.shaders,
				&uniform!{},
				&Default::default(),
			)
			.unwrap();
		target.finish().unwrap();
	}

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
