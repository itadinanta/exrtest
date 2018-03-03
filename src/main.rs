extern crate getopts;
extern crate openexr;

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::path::Path;

use openexr::{FrameBuffer, Header, ScanlineOutputFile, PixelType};

fn main() {
	let args = env::args_os().collect::<Vec<_>>();

	let width = 12000;
	let height = 6000;
	let mut pixel_data = vec![(0.82f32, 1.78f32, 0.21f32); width * height];

	let mut idx: usize = 0;
	for rgb in &mut pixel_data {
		let x = idx % width;
		let y = idx / width;
		*rgb = if (x + y) % 2 == 0 { (0., 0., 0.) } else { (1., 1., 1.) };
		idx += 1;
	}

	let f = env::args_os().nth(1).unwrap_or(OsString::from("example.exr"));
	let mut file = File::create(Path::new(&f)).expect("Could not open file");

	let mut exr_file = ScanlineOutputFile::new(&mut file,
											   &Header::new()
												   .set_resolution(width as u32, height as u32)
												   .add_channel("R", PixelType::FLOAT)
												   .add_channel("G", PixelType::FLOAT)
												   .add_channel("B", PixelType::FLOAT))
		.unwrap();

	let fb = {
		// Create the frame buffer
		let mut fb = FrameBuffer::new(width, height);
		fb.insert_channels(&["R", "G", "B"], &pixel_data);
		fb
	};

	exr_file.write_pixels(&fb).unwrap();
}