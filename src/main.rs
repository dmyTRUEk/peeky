//! peeky

use clap::Parser;
use image::{ImageReader, Rgb};
use minifb::{Key, Window, WindowOptions};



#[derive(Parser, Debug)]
#[clap(
	about,
	author,
	version,
	help_template = "\
		{before-help}{name} v{version}\n\
		\n\
		{about}\n\
		\n\
		Author: {author}\n\
		\n\
		{usage-heading} {usage}\n\
		\n\
		{all-args}{after-help}\
	",
)]
struct CliArgs {
	// // disable fullscreen
	// #[arg(short='f', long, default_value_t=false)]
	// disable_fullscreen: bool,

	// verbose output
	#[arg(short='v', long, default_value_t=false)]
	verbose: bool,

	/// filepath to image to show
	filepath: String,
}



fn main() {
	let CliArgs {
		//disable_fullscreen, // TODO(fix): change "windowing" library?
		verbose,
		filepath,
	} = CliArgs::parse();

	// TODO: config file

	let (mut w, mut h) = (1600, 900);
	let mut buffer: Vec<u32> = vec![BG_COLOR.0; w * h];

	let mut window = Window::new(
		concat!("Peeky v", env!("CARGO_PKG_VERSION")),
		w, h,
		WindowOptions {
			resize: true,
			..WindowOptions::default()
		}
	).expect("unable to create window");

	window.set_target_fps(60);
	window.update_with_buffer(&buffer, w, h).expect(UNABLE_TO_UPDATE_WINDOW_BUFFER);

	let img = ImageReader::open(&filepath).unwrap().decode().unwrap();
	let pixels = img.into_rgb8();

	let mut frame_i: u64 = 0;
	#[allow(unused_labels)]
	'mainloop: while window.is_open() && !window.is_key_down(Key::Escape) {
		let mut is_redraw_needed: bool = frame_i == 0; // condition needed to render first frame

		// handle resizing
		let wh_prev = (w, h);
		(w, h) = window.get_size();
		if (w, h) != wh_prev {
			#[allow(irrefutable_let_patterns)]
			if let new_size = w * h && new_size != buffer.len() {
				buffer.resize(new_size, BG_COLOR.0);
			}
			if verbose { eprintln!("Resized to {w}x{h}") }
			is_redraw_needed = true;
		}

		if window.is_key_pressed_once(Key::H) {
			// TODO
		}
		if window.is_key_pressed_once(Key::J) {
			// TODO
		}
		if window.is_key_pressed_once(Key::K) {
			// TODO
		}
		if window.is_key_pressed_once(Key::L) {
			// TODO
		}

		if is_redraw_needed {
			frame_i += 1;
			if verbose { eprintln!("\nframe {frame_i}:") }

			buffer.fill(BG_COLOR.0);

			for y in 0..pixels.height() {
				for x in 0..pixels.width() {
					if let Some(buffer_pixel) = buffer.get_mut(xy_to_buf_index(x, y, w)) {
						let rgb = pixels.get_pixel(x, y);
						*buffer_pixel = Color::from(*rgb).0;
					}
				}
			}
		} // end of render

		window.update_with_buffer(&buffer, w, h).expect(UNABLE_TO_UPDATE_WINDOW_BUFFER);
	} // end of main loop
} // end of main

const UNABLE_TO_UPDATE_WINDOW_BUFFER: &str = "unable to update window buffer";

fn xy_to_buf_index(x: u32, y: u32, w: usize) -> usize {
	(x as usize) + w * (y as usize)
}





#[derive(Clone, Copy)]
struct Color(u32);

const BLACK: Color = Color(0x000000);
const WHITE: Color = Color(0xffffff);

const RED  : Color = Color(0xff0000);
const GREEN: Color = Color(0x00ff00);
const BLUE : Color = Color(0x0000ff);

const CYAN   : Color = Color(0x00ffff);
const MAGENTA: Color = Color(0xff00ff);
const YELLOW : Color = Color(0xffff00);

const BG_COLOR: Color = BLACK;

impl From<Rgb<u8>> for Color {
	fn from(Rgb(rgb): Rgb<u8>) -> Self {
		Color(rgb_to_u32(rgb))
	}
}
fn rgb_to_u32([r,g,b]: [u8; 3]) -> u32 {
	u32::from_be_bytes([0xff,r,g,b])
}





trait WindowExtIsKeyPressed {
	fn is_key_pressed_once(&self, key: Key) -> bool;
	fn is_key_pressed_repeat(&self, key: Key) -> bool;
}
impl WindowExtIsKeyPressed for Window {
	fn is_key_pressed_once(&self, key: Key) -> bool {
		self.is_key_pressed(key, minifb::KeyRepeat::No)
	}
	fn is_key_pressed_repeat(&self, key: Key) -> bool {
		self.is_key_pressed(key, minifb::KeyRepeat::Yes)
	}
}

