use crate::color::*;

struct Canvas {
	width: usize,
	height: usize,
	data: Vec<Vec<Color>>,
}

fn build_canvas(width: usize, height: usize) -> Canvas {
	Canvas {
		width: width,
		height: height,
		data: vec![vec![build_color(0.0, 0.0, 0.0); width]; height],
	}
}

impl Canvas {
	fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> () {
		self.data[x][y] = color;
	}

	fn pixel_at(&self, x: usize, y: usize) -> Color {
		self.data[x][y]
	}

	fn to_ppm(&self) -> String {
		let mut ppm = String::new();
		// header is "P3", width/height, color max values
		ppm.push_str("P3\n");
		ppm.push_str(&(self.width as u32).to_string());
		ppm.push(' ');
		ppm.push_str(&(self.height as u32).to_string());
		ppm.push('\n');
		ppm.push_str("255\n");

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
		let mut canvas = build_canvas(10, 10);
		let color = build_color(0.1, 0.2, 0.3);
		canvas.write_pixel(3, 4, color);
		assert_eq!(canvas.pixel_at(3, 4), color);
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
}
