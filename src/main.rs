//! peeky

use clap::Parser;
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
	// verbose output
	#[arg(short='v', long, default_value_t=false)]
	verbose: bool,
}



fn main() {
	let CliArgs {
		verbose,
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

			// TODO
		} // end of render

		window.update_with_buffer(&buffer, w, h).expect(UNABLE_TO_UPDATE_WINDOW_BUFFER);
	} // end of main loop
} // end of main

const UNABLE_TO_UPDATE_WINDOW_BUFFER: &str = "unable to update window buffer";



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

