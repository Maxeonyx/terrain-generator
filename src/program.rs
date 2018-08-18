use super::images::Images;
use cgmath::{self, Deg, Matrix4, One, Point3, Vector3, Vector4, Zero};
use glium::{self, glutin, index::PrimitiveType, Surface};
use std;

const TERRAIN_WIDTH: usize = 10;
const NUM_VERTICES: usize = TERRAIN_WIDTH * TERRAIN_WIDTH;
const NUM_PATCH_VERTICES: usize = (TERRAIN_WIDTH - 1) * (TERRAIN_WIDTH - 1) * 4;

const WORLD_WIDTH: f32 = 100.0;

#[derive(Copy, Clone, Default)]
struct Vertex {
	position: [f32; 3],
}
implement_vertex!(Vertex, position);

pub struct Program {
	display: glium::Display,
	events_loop: glutin::EventsLoop,
	shaders: glium::Program,
	images: Images,
}

impl Program {
	pub fn new(
		display: glium::Display,
		events_loop: glutin::EventsLoop,
		shaders: glium::Program,
		images: Images,
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
		let (vertex_buffer, index_buffer) = self.make_vertices();
		let mvp_matrix = self.make_mvp_matrix();
		let mut target = self.display.draw();
		target.clear_color(0.0, 1.0, 0.0, 0.0);
		target
			.draw(
				&vertex_buffer,
				&index_buffer,
				&self.shaders,
				&uniform!{
					tex_heightmap: &self.images.heightmap,
					tex_lava: &self.images.lava,
					world_width: WORLD_WIDTH,
					mvp_matrix: <[[f32; 4]; 4]>::from(mvp_matrix.into()),
				},
				&glium::draw_parameters::DrawParameters {
					polygon_mode: glium::draw_parameters::PolygonMode::Line,
					..Default::default()
				},
			)
			.unwrap();
		target.finish().unwrap();
	}

	fn make_mvp_matrix(&self) -> Matrix4<f32> {
		let perspective = cgmath::perspective(Deg(30 as f32), 1.25, 0.01, 200.0);

		let look_at = Matrix4::look_at(
			Point3 {
				x: -8.0,
				y: -8.0,
				z: 22.0,
			},
			Point3 {
				x: 40.0,
				y: 40.0,
				z: 0.0,
			},
			Vector3::unit_z(),
		);
		let mvp = perspective * look_at;

		mvp
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

	fn make_vertices(&self) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
		let vertex_buffer = {
			let mut vertices = [Vertex::default(); NUM_VERTICES];

			for x in 0..TERRAIN_WIDTH {
				for y in 0..TERRAIN_WIDTH {
					let lerp_x = x as f32 / (TERRAIN_WIDTH - 1) as f32;
					let lerp_y = y as f32 / (TERRAIN_WIDTH - 1) as f32;

					vertices[y * TERRAIN_WIDTH + x].position =
						[lerp_x * WORLD_WIDTH, lerp_y * WORLD_WIDTH, 0.0]
				}
			}

			glium::VertexBuffer::new(&self.display, &vertices).unwrap()
		};

		let index_buffer = {
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

		(vertex_buffer, index_buffer)
	}
}
