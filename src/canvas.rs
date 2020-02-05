use crate::color::*;

struct Canvas {
	width: usize,
	height: usize,
	data: Vec<Vec<Color>>,
}

fn build_canvas(height: usize, width: usize) -> Canvas {
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
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_height_and_width() {
		let c = build_canvas(10, 10);
		assert_eq!(c.height, 10);
		assert_eq!(c.width, 10);
	}

	#[test]
	fn test_write_and_read_pixels() {
		let mut canvas = build_canvas(10, 10);
		let color = build_color(0.1, 0.2, 0.3);
		canvas.write_pixel(3, 4, color);
		assert_eq!(canvas.pixel_at(3, 4), color);
	}
}
