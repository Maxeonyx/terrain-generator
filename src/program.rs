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
		const TERRAIN_WIDTH: usize = 10;

		let vertex_buffer = {
			#[derive(Copy, Clone, Default)]
			struct Vertex {
				position: [f32; 2],
			}

			implement_vertex!(Vertex, position);

			// let vertices = [
			// 	Vertex {
			// 		position: [-1f32, 0f32],
			// 	},
			// 	Vertex {
			// 		position: [-1f32, 1f32],
			// 	},
			// 	Vertex {
			// 		position: [0f32, 0f32],
			// 	},
			// 	Vertex {
			// 		position: [0f32, 1f32],
			// 	},
			// 	Vertex {
			// 		position: [1f32, 0f32],
			// 	},
			// 	Vertex {
			// 		position: [1f32, 1f32],
			// 	},
			// ];
			let mut vertices = [Vertex::default(); TERRAIN_WIDTH * TERRAIN_WIDTH];

			for x in 0..TERRAIN_WIDTH {
				for y in 0..TERRAIN_WIDTH {
					vertices[y * TERRAIN_WIDTH + x].position = [
						x as f32 / TERRAIN_WIDTH as f32,
						y as f32 / TERRAIN_WIDTH as f32,
					]
				}
			}

			glium::VertexBuffer::new(&self.display, &vertices).unwrap()
		};

		let index_buffer = {
			const NUM_PATCH_VERTICES: usize = (TERRAIN_WIDTH - 1) * (TERRAIN_WIDTH - 1) * 4;

			let mut indices = [0u16; NUM_PATCH_VERTICES];

			let mut index_index = 0;
			for x in 0..(TERRAIN_WIDTH - 1) {
				for y in 0..(TERRAIN_WIDTH - 1) {
					let vertex_indices = [
						((y) * TERRAIN_WIDTH + (x)) as u16,
						((y + 1) * TERRAIN_WIDTH + (x)) as u16,
						((y) * TERRAIN_WIDTH + (x + 1)) as u16,
						((y + 1) * TERRAIN_WIDTH + (x + 1)) as u16,
					];
					indices[index_index] = vertex_indices[0];
					indices[index_index + 1] = vertex_indices[1];
					indices[index_index + 2] = vertex_indices[2];
					indices[index_index + 3] = vertex_indices[3];
					index_index += 4;
				}
			}

			glium::IndexBuffer::new(
				&self.display,
				PrimitiveType::Patches {
					vertices_per_patch: 4,
				},
				&indices,
			).unwrap()
		};

		let mut target = self.display.draw();
		target.clear_color(0.0, 1.0, 0.0, 0.0);
		target
			.draw(
				&vertex_buffer,
				&index_buffer,
				&self.shaders,
				&uniform!{},
				&glium::draw_parameters::DrawParameters {
					polygon_mode: glium::draw_parameters::PolygonMode::Line,
					..Default::default()
				},
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
