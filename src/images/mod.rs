use image;
use image::load_from_memory;

const TEXTURE1: &[u8] = include_bytes!("lava.jpg");

pub struct Images {
	pub texture1: image::DynamicImage,
}

pub fn init_images() -> Box<Images> {
	Box::new(Images {
		texture1: load_from_memory(TEXTURE1).expect("Could not load lava.jpg"),
	})
}
