use glium;

const VERTEX_SHADER: &str = include_str!("terrain.vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("terrain.frag.glsl");

pub fn init_shaders(display: &glium::Display) -> glium::Program {
	program!(display, 
		140 => {
			vertex: VERTEX_SHADER,
			fragment: FRAGMENT_SHADER,
		},
	).expect("Failed to initialise shaders.")
}
