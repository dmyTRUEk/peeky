//! peeky

#![allow(
	clippy::iter_nth_zero,
	clippy::let_and_return,
	clippy::useless_format,
)]

#![deny(
	unconditional_recursion,
	unknown_lints,
	unreachable_code,
	unreachable_patterns,
	unused_variables,
	//clippy::as_conversions,
	//clippy::cast_possible_truncation,
	//clippy::cast_possible_wrap,
	//clippy::cast_precision_loss,
	//clippy::cast_sign_loss,
	clippy::fn_to_numeric_cast_any,
	//clippy::format_empty_string, // doesnt exist :c
	clippy::only_used_in_recursion,
	clippy::println_empty_string,
	clippy::unnecessary_cast,
)]

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

	let zoom_default = 0.001; // TODO: calc from image and window sizes
	let cam_x_default: float = pixels.dimensions().0 as float / 2.;
	let cam_y_default: float = pixels.dimensions().1 as float / 2.;
	let mut zoom : float = zoom_default;
	let mut cam_x: float = cam_x_default; // TODO: make it centered by default
	let mut cam_y: float = cam_y_default; // TODO: make it centered by default
	let cam_move_step: float = 0.1;
	let cam_zoom_step: float = 1.1;

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

		let cam_move_step = cam_move_step / zoom;
		if window.is_key_pressed_repeat(Key::H) {
			cam_x -= cam_move_step;
			is_redraw_needed = true;
		}
		if window.is_key_pressed_repeat(Key::J) {
			cam_y += cam_move_step;
			is_redraw_needed = true;
		}
		if window.is_key_pressed_repeat(Key::K) {
			cam_y -= cam_move_step;
			is_redraw_needed = true;
		}
		if window.is_key_pressed_repeat(Key::L) {
			cam_x += cam_move_step;
			is_redraw_needed = true;
		}

		// Compute center world coords BEFORE changing zoom
		let scx = (w as float) / 2.; // screen center x
		let scy = (h as float) / 2.; // screen center y
		let center_world_before = screen_to_image(STW{x:scx, y:scy, wf: w as float, hf: h as float, cam_x, cam_y, zoom});

		if window.is_key_pressed_repeat(Key::I) {
			zoom *= cam_zoom_step;
			is_redraw_needed = true;
		}
		if window.is_key_pressed_repeat(Key::O) {
			zoom /= cam_zoom_step;
			is_redraw_needed = true;
		}

		let mut is_zoom_reset: bool = false;
		if window.is_key_pressed_repeat(Key::R) {
			is_zoom_reset = true;
			zoom  = zoom_default;
			cam_x = cam_x_default;
			cam_y = cam_y_default;
			if verbose { eprintln!("zoom & position reset") }
			is_redraw_needed = true;
		}
		if window.is_key_pressed_repeat(Key::Z) {
			is_zoom_reset = true;
			zoom  = zoom_default;
			if verbose { eprintln!("zoom reset") }
			is_redraw_needed = true;
		}

		if is_redraw_needed {
			frame_i += 1;
			if verbose { eprintln!("\nframe {frame_i}:") }

			if verbose { println!("cam xyz: {cam_x}, {cam_y}, zoom={zoom}") }
			if !is_zoom_reset {
				// Compute center world coords AFTER zoom
				let center_world_after = screen_to_image(STW{x:scx, y:scy, wf: w as float, hf: h as float, cam_x, cam_y, zoom});
				// Adjust camera so center remains fixed
				cam_x += center_world_before.0 - center_world_after.0;
				cam_y += center_world_before.1 - center_world_after.1;
			}

			buffer.fill(BG_COLOR.0);

			let (image_w, image_h) = pixels.dimensions();
			for buf_y in 0 .. h as u32 {
				for buf_x in 0 .. w as u32 {
					let (img_x, img_y) = screen_to_image(STW{x: buf_x as float, y: buf_y as float, wf: w as float, hf: h as float, cam_x, cam_y, zoom});
					let img_x: i32 = img_x as _;
					let img_y: i32 = img_y as _;
					if !(0 <= img_x && img_x < image_w as i32) { continue }
					if !(0 <= img_y && img_y < image_h as i32) { continue }
					let img_x: u32 = img_x as _;
					let img_y: u32 = img_y as _;
					if let Some(img_pixel) = pixels.get_pixel_checked(img_x, img_y) {
						buffer[xy_to_buf_index(buf_x, buf_y, w)] = Color::from(*img_pixel).0;
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



struct STW {
	x: float,
	y: float,
	/// screen_w
	wf: float,
	/// screen_h
	hf: float,
	cam_x: float,
	cam_y: float,
	zoom: float,
}
fn screen_to_image(STW { x, y, wf, hf, cam_x, cam_y, zoom }: STW) -> (float, float) {
	let aspect_ratio = hf / wf;
	let world_x = cam_x + (x - wf * 0.5) / (wf * 0.5) / zoom;
	let world_y = cam_y + (y - hf * 0.5) / (hf * 0.5) / zoom * aspect_ratio;
	(world_x, world_y)
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



#[allow(non_camel_case_types)]
type float = f32;



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

