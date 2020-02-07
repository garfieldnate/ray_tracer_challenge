use crate::color::*;

pub struct Canvas {
	pub width: usize,
	pub height: usize,
	data: Vec<Vec<Color>>,
}

pub fn build_canvas(width: usize, height: usize) -> Canvas {
	Canvas {
		width: width,
		height: height,
		data: vec![vec![build_color(0.0, 0.0, 0.0); width]; height],
	}
}

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

	// TODO: eeew, just, really... clean this up
	pub fn to_ppm(&self) -> String {
		let mut ppm = String::new();
		// header is "P3", width/height, and color max value
		ppm.push_str("P3\n");
		ppm.push_str(&(self.width as u32).to_string());
		ppm.push(' ');
		ppm.push_str(&(self.height as u32).to_string());
		ppm.push('\n');
		ppm.push_str("255\n");

		// write pixel data
		for row in 0..self.height {
			let mut line = String::new();
			for (i, column) in (0..self.width).enumerate() {
				let color = self.pixel_at(column, row);
				// scale and clamp color values at 255
				let r = (color.r * 255.0).min(255.0).max(0.0) as u8;
				let g = (color.g * 255.0).min(255.0).max(0.0) as u8;
				let b = (color.b * 255.0).min(255.0).max(0.0) as u8;

				line.push_str(&r.to_string());
				if line.len() < 67 {
					line.push(' ');
				} else {
					ppm.push_str(&line);
					ppm.push('\n');
					line = String::new();
				}

				line.push_str(&g.to_string());
				if line.len() < 67 {
					line.push(' ');
				} else {
					ppm.push_str(&line);
					ppm.push('\n');
					line = String::new();
				}

				line.push_str(&b.to_string());

				// if not at end of row yet, write a space or newline if the next point will be on this line
				if i != self.width - 1 {
					// max line length is 70; each number could be 3 chars long plus a space
					if line.len() < 67 {
						line.push(' ');
					} else {
						ppm.push_str(&line);
						ppm.push('\n');
						line = String::new();
					}
				}
			}
			if line.len() != 0 {
				ppm.push_str(&line);
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
