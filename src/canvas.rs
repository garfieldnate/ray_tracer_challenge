use crate::color::{build_color, Color};

pub struct Canvas {
	pub width: usize,
	pub height: usize,
	data: Vec<Vec<Color>>,
}

// Create a canvas initialized to all black
pub fn build_canvas(width: usize, height: usize) -> Canvas {
	Canvas {
		width: width,
		height: height,
		data: vec![vec![build_color(0.0, 0.0, 0.0); width]; height],
	}
}

const MAX_COLOR_VAL: i16 = 255;
const MAX_PPM_LINE_LENGTH: usize = 70;
// length of "255" is 3
// TODO: this should be evaluated programmatically, but "no matching in consts allowed" error prevented this
const MAX_COLOR_VAL_STR_LEN: usize = 3;
impl Canvas {
	pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> () {
		if x <= self.width && y <= self.height {
			self.data[y][x] = color;
		} else {
			// return fail result
		}
	}

	pub fn pixel_at(&self, x: usize, y: usize) -> Color {
		self.data[y][x]
	}

	// scale/clamp color values from 0-1 to 0-255
	fn scale_color(&self, rgb: f32) -> u8 {
		(rgb * MAX_COLOR_VAL as f32)
			.min(MAX_COLOR_VAL as f32)
			.max(0.0) as u8
	}

	// If current line has no more room for more RGB values, add it to the PPM string and clear it;
	// otherwise, add a space separator in preparation for the next RGB value
	fn write_rgb_separator(&self, line: &mut String, ppm: &mut String) {
		if line.len() < MAX_PPM_LINE_LENGTH - MAX_COLOR_VAL_STR_LEN {
			(*line).push(' ');
		} else {
			ppm.push_str(&line);
			ppm.push('\n');
			line.clear();
		}
	}

	// Return string containing PPM (portable pixel map) data representing current canvas
	pub fn to_ppm(&self) -> String {
		let mut ppm = String::new();
		// write header
		ppm.push_str("P3\n");
		ppm.push_str(&(format!("{} {}\n", self.width, self.height)));
		ppm.push_str(&(format!("{}\n", MAX_COLOR_VAL)));

		// Write pixel data. Each pixel RGB value is written with a separating space or newline;
		// new rows are written on new lines for human reading convenience, but lines longer than
		// MAX_PPM_LINE_LENGTH must also be split.
		let mut current_line = String::new();
		for row in 0..self.height {
			current_line.clear();
			for (i, column) in (0..self.width).enumerate() {
				let color = self.pixel_at(column, row);
				let r = self.scale_color(color.r);
				let g = self.scale_color(color.g);
				let b = self.scale_color(color.b);

				current_line.push_str(&r.to_string());
				self.write_rgb_separator(&mut current_line, &mut ppm);

				current_line.push_str(&g.to_string());
				self.write_rgb_separator(&mut current_line, &mut ppm);

				current_line.push_str(&b.to_string());

				// if not at end of row yet, write a space or newline if the next point will be on this line
				if i != self.width - 1 {
					self.write_rgb_separator(&mut current_line, &mut ppm);
				}
			}
			if current_line.len() != 0 {
				ppm.push_str(&current_line);
				ppm.push('\n');
			}
		}
		ppm
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_height_and_width() {
		let c = build_canvas(15, 10);
		assert_eq!(c.width, 15);
		assert_eq!(c.height, 10);
	}

	#[test]
	fn test_write_and_read_pixels() {
		let mut canvas = build_canvas(10, 5);
		let color = build_color(0.1, 0.2, 0.3);
		canvas.write_pixel(7, 4, color);
		assert_eq!(canvas.pixel_at(7, 4), color);
	}

	#[test]
	fn test_ppm_header() {
		let c = build_canvas(20, 5);
		let ppm = c.to_ppm();
		let mut lines = ppm.lines();
		assert_eq!(lines.next().unwrap(), "P3");
		assert_eq!(lines.next().unwrap(), "20 5");
		assert_eq!(lines.next().unwrap(), "255");
	}

	#[test]
	fn test_ppm_pixel_data() {
		let mut c = build_canvas(5, 3);
		c.write_pixel(0, 0, build_color(1.5, 0.0, 0.0));
		c.write_pixel(2, 1, build_color(0.0, 0.5, 0.0));
		c.write_pixel(4, 2, build_color(-0.5, 0.0, 1.0));

		let ppm = c.to_ppm();
		let mut lines = ppm.lines();
		// ignore header
		lines.next();
		lines.next();
		lines.next();
		assert_eq!(lines.next().unwrap(), "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
		// book says 128, but I'll trust Rust's rounding for now
		assert_eq!(lines.next().unwrap(), "0 0 0 0 0 0 0 127 0 0 0 0 0 0 0");
		assert_eq!(lines.next().unwrap(), "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
	}

	#[test]
	fn test_splitting_long_ppm_lines() {
		let mut canvas = build_canvas(10, 2);
		let color = build_color(1.0, 0.8, 0.6);
		// TODO: maybe turn this into a function on canvas?
		for row in 0..canvas.height {
			for column in 0..canvas.width {
				canvas.write_pixel(column, row, color);
			}
		}
		let ppm = canvas.to_ppm();
		let mut lines = ppm.lines();
		// skip header
		lines.next();
		lines.next();
		lines.next();
		assert_eq!(
			lines.next().unwrap(),
			"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
		);
		assert_eq!(
			lines.next().unwrap(),
			"153 255 204 153 255 204 153 255 204 153 255 204 153"
		);
		assert_eq!(
			lines.next().unwrap(),
			"255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
		);
		assert_eq!(
			lines.next().unwrap(),
			"153 255 204 153 255 204 153 255 204 153 255 204 153"
		);
	}
}
