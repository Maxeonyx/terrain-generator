use super::images::Images;
use glium::{self, glutin, index::PrimitiveType, Surface};
use std;

pub struct Program {
	display: glium::Display,
	events_loop: glutin::EventsLoop,
	shaders: glium::Program,
	images: Box<Images>,
}

impl Program {
	pub fn new(
		display: glium::Display,
		events_loop: glutin::EventsLoop,
		shaders: glium::Program,
		images: Box<Images>,
	) -> Self {
		Program {
			display,
			events_loop,
			shaders,
			images,
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
				position: [f32; 3],
			}

			implement_vertex!(Vertex, position);

			let mut vertices = [Vertex::default(); TERRAIN_WIDTH * TERRAIN_WIDTH];

			for x in 0..TERRAIN_WIDTH {
				for y in 0..TERRAIN_WIDTH {
					let lerp_x = x as f32 / (TERRAIN_WIDTH - 1) as f32;
					let lerp_y = y as f32 / (TERRAIN_WIDTH - 1) as f32;

					vertices[y * TERRAIN_WIDTH + x].position = [
						lerp_x * TERRAIN_WIDTH as f32 * 2.0 - TERRAIN_WIDTH as f32,
						lerp_y * TERRAIN_WIDTH as f32 * 2.0 - TERRAIN_WIDTH as f32,
						0.0,
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

		let rgb_image = self.images.texture1.to_rgb();
		let dimensions = rgb_image.dimensions();
		let texture1_img =
			glium::texture::RawImage2d::from_raw_rgb(rgb_image.into_raw(), dimensions);
		let texture1_tex = glium::texture::Texture2d::new(&self.display, texture1_img).unwrap();
		let mut target = self.display.draw();
		target.clear_color(0.0, 1.0, 0.0, 0.0);
		target
			.draw(
				&vertex_buffer,
				&index_buffer,
				&self.shaders,
				&uniform!{
					texture1: &texture1_tex,
				},
				&glium::draw_parameters::DrawParameters {
					//polygon_mode: glium::draw_parameters::PolygonMode::Line,
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
