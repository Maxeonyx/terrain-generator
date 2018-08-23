use glium;
use image::load_from_memory;

const TEX_HEIGHTMAP: &[u8] = include_bytes!("heightmap.png");
const TEX_HEIGHTMAP_2: &[u8] = include_bytes!("heightmap_2.png");
const TEX_LAVA: &[u8] = include_bytes!("lava.jpg");
const TEX_LAVAROCK: &[u8] = include_bytes!("lavarock.png");
const TEX_ASH: &[u8] = include_bytes!("ash_brown.png");

pub struct Images {
	pub heightmap: glium::Texture2d,
	pub heightmap_2: glium::Texture2d,
	pub lava: glium::Texture2d,
	pub lavarock: glium::Texture2d,
	pub ash: glium::Texture2d,
}

pub fn init_images(display: &glium::Display) -> Images {
	Images {
		heightmap: u8_to_tex(display, TEX_HEIGHTMAP),
		heightmap_2: u8_to_tex(display, TEX_HEIGHTMAP_2),
		lava: u8_to_tex(display, TEX_LAVA),
		lavarock: u8_to_tex(display, TEX_LAVAROCK),
		ash: u8_to_tex(display, TEX_ASH),
	}
}

fn u8_to_tex(display: &glium::Display, bytes: &[u8]) -> glium::Texture2d {
	let dynamic_image = load_from_memory(bytes).expect("Could not load image.");
	let rgb_image = dynamic_image.to_rgb();
	let dimensions = rgb_image.dimensions();
	let image_2d = glium::texture::RawImage2d::from_raw_rgb(rgb_image.into_raw(), dimensions);

	glium::texture::Texture2d::new(display, image_2d).unwrap()
}
