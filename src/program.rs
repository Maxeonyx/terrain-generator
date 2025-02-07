use super::images::Images;
use cgmath::{self, prelude::*, Deg, Matrix4, Point3, Quaternion, Rotation, Rotation3, Vector3};
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

struct Camera {
	position: Point3<f32>,
	direction: Vector3<f32>,
	speed: f32,
	rotation_speed: Deg<f32>,
}

enum CameraMovement {
	Left,
	Right,
	Forward,
	Back,
}

impl Camera {
	pub fn new() -> Self {
		let position = Point3 {
			x: -5.0,
			y: -5.0,
			z: 28.0,
		};
		let x_rotation = cgmath::Quaternion::from_axis_angle(Vector3::unit_x(), Deg(-30.0f32));
		let z_rotation = cgmath::Quaternion::from_axis_angle(Vector3::unit_z(), Deg(-45.0f32));
		let direction = z_rotation.rotate_vector(x_rotation.rotate_vector(Vector3::unit_y()));

		Camera {
			position,
			direction,
			speed: 1.5,
			rotation_speed: Deg(8.0),
		}
	}

	pub fn update(&mut self, movement: CameraMovement) {
		match movement {
			CameraMovement::Back => {
				self.position = self.position
					+ self.speed
						* Vector3::new(-self.direction.x, -self.direction.y, 0.0).normalize()
			}
			CameraMovement::Forward => {
				self.position = self.position
					+ self.speed * Vector3::new(self.direction.x, self.direction.y, 0.0).normalize()
			}
			CameraMovement::Left => {
				self.direction =
					Quaternion::from_axis_angle(Vector3::unit_z(), self.rotation_speed)
						.rotate_vector(self.direction);
			}
			CameraMovement::Right => {
				self.direction =
					Quaternion::from_axis_angle(Vector3::unit_z(), -self.rotation_speed)
						.rotate_vector(self.direction);
			}
		}
	}
}

pub struct Program {
	display: glium::Display,
	events_loop: glutin::EventsLoop,
	shaders: glium::Program,
	images: Images,
	camera: Camera,
	which_heightmap: i32,
	polygon_mode: glium::draw_parameters::PolygonMode,
	lava_level: f32,
	ash_level: f32,
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
			camera: Camera::new(),
			which_heightmap: 0,
			polygon_mode: glium::draw_parameters::PolygonMode::Fill,
			lava_level: 0.3f32,
			ash_level: 0.7f32,
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
		target.clear_color_and_depth((0.02, 0.02, 0.02, 0.0), 1.0);
		target
			.draw(
				&vertex_buffer,
				&index_buffer,
				&self.shaders,
				&uniform!{
					tex_heightmap: &self.images.heightmap,
					tex_heightmap_2: &self.images.heightmap_2,
					tex_lava: &self.images.lava,
					tex_lavarock: &self.images.lavarock,
					tex_ash: &self.images.ash,
					camera_position: <[f32; 3]>::from(self.camera.position.into()),
					world_width: WORLD_WIDTH,
					mvp_matrix: <[[f32; 4]; 4]>::from(mvp_matrix.into()),
					which_heightmap: self.which_heightmap,
					lava_level: self.lava_level,
					ash_level: self.ash_level,
					is_line_mode: if self.polygon_mode == glium::draw_parameters::PolygonMode::Line { 1 } else { 0 },
				},
				&glium::draw_parameters::DrawParameters {
					depth: glium::Depth {
						test: glium::draw_parameters::DepthTest::IfLess,
						write: true,
						..Default::default()
					},
					polygon_mode: self.polygon_mode,
					..Default::default()
				},
			)
			.unwrap();
		target.finish().unwrap();
	}

	fn make_mvp_matrix(&self) -> Matrix4<f32> {
		let perspective = cgmath::perspective(Deg(60 as f32), 1.25, 0.01, 200.0);

		let look_at = Matrix4::look_at_dir(
			self.camera.position,
			self.camera.direction,
			Vector3::unit_z(),
		);
		let mvp = perspective * look_at;

		mvp
	}

	fn handle_events(&mut self) {
		let camera = &mut self.camera;
		let events_loop = &mut self.events_loop;
		let heightmap = &mut self.which_heightmap;
		let polygon_mode = &mut self.polygon_mode;
		let lava_level = &mut self.lava_level;
		let ash_level = &mut self.ash_level;
		events_loop.poll_events(|event| match event {
			glutin::Event::WindowEvent { event, .. } => match event {
				glutin::WindowEvent::CloseRequested => std::process::exit(0),
				glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
					Some(keycode) => match keycode {
						glutin::VirtualKeyCode::Left => camera.update(CameraMovement::Left),
						glutin::VirtualKeyCode::Right => camera.update(CameraMovement::Right),
						glutin::VirtualKeyCode::Up => camera.update(CameraMovement::Forward),
						glutin::VirtualKeyCode::Down => camera.update(CameraMovement::Back),
						glutin::VirtualKeyCode::H => {
							if input.state == glutin::ElementState::Pressed {
								if *heightmap == 0 {
									*heightmap = 1
								} else {
									*heightmap = 0
								}
							}
						}
						glutin::VirtualKeyCode::P => {
							if input.state == glutin::ElementState::Pressed {
								if *polygon_mode == glium::draw_parameters::PolygonMode::Fill {
									*polygon_mode = glium::draw_parameters::PolygonMode::Line
								} else {
									*polygon_mode = glium::draw_parameters::PolygonMode::Fill
								}
							}
						}
						glutin::VirtualKeyCode::K => {
							if input.state == glutin::ElementState::Pressed {
								*lava_level += 0.005;
							}
						}
						glutin::VirtualKeyCode::L => {
							if input.state == glutin::ElementState::Pressed {
								*lava_level -= 0.005;
							}
						}
						glutin::VirtualKeyCode::I => {
							if input.state == glutin::ElementState::Pressed {
								*ash_level += 0.005;
							}
						}
						glutin::VirtualKeyCode::O => {
							if input.state == glutin::ElementState::Pressed {
								*ash_level -= 0.005;
							}
						}
						_ => {}
					},
					_ => {}
				},
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
