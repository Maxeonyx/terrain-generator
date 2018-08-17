use glium;

const VERTEX_SHADER: &str = include_str!("terrain.vert.glsl");
const FRAGMENT_SHADER: &str = include_str!("terrain.frag.glsl");
const TESS_EVAL_SHADER: &str = include_str!("terrain.eval.glsl");
const TESS_CONT_SHADER: &str = include_str!("terrain.cont.glsl");
const GEOMETRY_SHADER: &str = include_str!("terrain.geom.glsl");

pub fn init_shaders(display: &glium::Display) -> glium::Program {
	program!(display, 
		400 => {
			vertex: VERTEX_SHADER,
			fragment: FRAGMENT_SHADER,
			tessellation_evaluation: TESS_EVAL_SHADER,
			tessellation_control: TESS_CONT_SHADER,
			geometry: GEOMETRY_SHADER,
		},
	).expect("Failed to initialise shaders.")
}
