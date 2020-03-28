use crate::color::Color;
use std::collections::VecDeque;
use std::io::{self, BufRead, BufReader, Read};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    data: Vec<Vec<Color>>,
}

const MAX_COLOR_VAL: u16 = 255;
const MAX_PPM_LINE_LENGTH: usize = 70;
// length of "255" is 3
// TODO: this should be evaluated programmatically, but "no matching in consts allowed" error prevented this
const MAX_COLOR_VAL_STR_LEN: usize = 3;
impl Canvas {
    // Create a canvas initialized to all black
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            data: vec![vec![color!(0, 0, 0); width]; height],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
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
            if !current_line.is_empty() {
                ppm.push_str(&current_line);
                ppm.push('\n');
            }
        }
        ppm
    }
}

// TODO: proper parsing errors should also contain the line and column number
#[derive(Debug)]
pub enum ParseError {
    IoError(io::Error),
    IncorrectFormat(String),
    ParseIntError(std::num::ParseIntError),
    MalformedDimensionHeader(String),
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::IoError(err)
    }
}
impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> ParseError {
        ParseError::ParseIntError(err)
    }
}

pub fn canvas_from_ppm<T: Read>(reader: T) -> Result<Canvas, ParseError> {
    let buf_reader = BufReader::new(reader);
    let mut line_iter = buf_reader.lines().enumerate().filter_map(clean_line);

    // TODO: these unwrap()'s are not great; should really fail properly if the file doesn't
    // contain this many lines
    let (_, line) = line_iter.next().unwrap();
    let line = line?;
    let line = line.trim();
    if line != "P3" {
        return Err(ParseError::IncorrectFormat(format!(
            "Incorrect magic number at line 1: expected P3, found {}",
            line
        )));
    }

    let (_, line) = line_iter.next().unwrap();
    let line = line?;
    let line = line.trim();
    let elements: Vec<&str> = line.split_whitespace().collect();
    if elements.len() != 2 {
        return Err(ParseError::MalformedDimensionHeader(format!(
            "Expected width and height at line 2; found {}",
            line
        )));
    }
    let width = elements[0].parse::<usize>()?;
    let height = elements[1].parse::<usize>()?;

    let (_, line) = line_iter.next().unwrap();
    let line = line?;
    let line = line.trim();
    let scale = line.parse::<u16>()?;
    if scale != MAX_COLOR_VAL {
        return Err(ParseError::IncorrectFormat(format!(
            "Incorrect scale at line 3: only 255 supported, found {}",
            line
        )));
    }

    let scale = scale as f32;
    let mut canvas = Canvas::new(width, height);
    // using u8 here because we only support a scale of 255
    let mut raw_rgb: VecDeque<u8> = VecDeque::new();
    let mut x = 0;
    let mut y = 0;
    for (_, (_index, line)) in line_iter.enumerate() {
        let line = line?;
        let line = line.trim();
        let line_rgb = line
            .split_whitespace()
            .map(|s| s.parse::<u8>())
            .collect::<Result<Vec<u8>, std::num::ParseIntError>>()?;
        raw_rgb.extend(line_rgb);
        while raw_rgb.len() >= 3 {
            let r = raw_rgb.pop_front().unwrap() as f32 / scale;
            let g = raw_rgb.pop_front().unwrap() as f32 / scale;
            let b = raw_rgb.pop_front().unwrap() as f32 / scale;
            canvas.write_pixel(x, y, color!(r, g, b));

            // move to next canvas pixel
            x += 1;
            if x >= width {
                x = 0;
                y += 1;
            }
        }
    }
    Ok(canvas)
}

fn clean_line(
    (index, line): (usize, Result<String, std::io::Error>),
) -> Option<(usize, Result<String, std::io::Error>)> {
    match line {
        Ok(s) => {
            let s = s.trim();
            if s.starts_with("#") || s.is_empty() {
                None
            } else {
                Some((index, Ok(s.to_string())))
            }
        }
        Err(_) => Some((index, line)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_height_and_width() {
        let c = Canvas::new(15, 10);
        assert_eq!(c.width, 15);
        assert_eq!(c.height, 10);
    }

    #[test]
    fn test_write_and_read_pixels() {
        let mut canvas = Canvas::new(10, 5);
        let color = color!(0.1, 0.2, 0.3);
        canvas.write_pixel(7, 4, color);
        assert_eq!(canvas.pixel_at(7, 4), color);
    }

    #[test]
    fn test_ppm_header() {
        let c = Canvas::new(20, 5);
        let ppm = c.to_ppm();
        let mut lines = ppm.lines();
        assert_eq!(lines.next().unwrap(), "P3");
        assert_eq!(lines.next().unwrap(), "20 5");
        assert_eq!(lines.next().unwrap(), "255");
    }

    #[test]
    fn test_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, color!(1.5, 0, 0));
        c.write_pixel(2, 1, color!(0, 0.5, 0));
        c.write_pixel(4, 2, color!(-0.5, 0, 1));

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
        let mut canvas = Canvas::new(10, 2);
        let color = color!(1, 0.8, 0.6);
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

    #[test]
    fn reading_file_with_wrong_magic_number() {
        let ppm = "P32
        1 1
        255
        0 0 0";
        let result = canvas_from_ppm(ppm.as_bytes());
        match result {
            Err(ParseError::IncorrectFormat(msg)) => {
                assert!(msg.contains("Incorrect magic number"))
            }
            _ => assert!(false, "Should return IncorrectFormat error"),
        }
    }

    #[test]
    fn reading_ppm_returns_canvas_with_correct_size() {
        let ppm = "P3
        10 2
        255
        0 0 0  0 0 0  0 0 0  0 0 0  0 0 0
        0 0 0  0 0 0  0 0 0  0 0 0  0 0 0
        0 0 0  0 0 0  0 0 0  0 0 0  0 0 0
        0 0 0  0 0 0  0 0 0  0 0 0  0 0 0
        ";
        let canvas = canvas_from_ppm(ppm.as_bytes()).unwrap();
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 2);
    }

    #[test]
    fn reading_pixel_data_from_ppm_file() {
        let ppm = "P3
        4 3
        255
        255 127 0  0 127 255  127 255 0  255 255 255
        0 0 0  255 0 0  0 255 0  0 0 255
        255 255 0  0 255 255  255 0 255  127 127 127";
        let canvas = canvas_from_ppm(ppm.as_bytes()).unwrap();

        let test_data = vec![
            ("1", 0, 0, color!(1, 0.49803922, 0)),
            ("2", 1, 0, color!(0, 0.49803922, 1)),
            ("3", 2, 0, color!(0.49803922, 1, 0)),
            ("4", 3, 0, color!(1, 1, 1)),
            ("5", 0, 1, color!(0, 0, 0)),
            ("6", 1, 1, color!(1, 0, 0)),
            ("7", 2, 1, color!(0, 1, 0)),
            ("8", 3, 1, color!(0, 0, 1)),
            ("9", 0, 2, color!(1, 1, 0)),
            ("10", 1, 2, color!(0, 1, 1)),
            ("11", 2, 2, color!(1, 0, 1)),
            ("12", 3, 2, color!(0.49803922, 0.49803922, 0.49803922)),
        ];
        for (name, x, y, expected_color) in test_data {
            println!("Case {}", name);
            assert_abs_diff_eq!(canvas.pixel_at(x, y), expected_color);
        }
    }

    #[test]
    fn ppm_parsing_ignores_comment_lines() {
        let ppm = "P3
        # this is a comment
        2 1
        # this, too
        255
        # another comment
        255 255 255
        # oh, no, comments in the pixel data!
        255 0 255
        ";
        let canvas = canvas_from_ppm(ppm.as_bytes()).unwrap();
        assert_eq!(canvas.pixel_at(0, 0), color!(1, 1, 1));
        assert_eq!(canvas.pixel_at(1, 0), color!(1, 0, 1));
    }

    #[test]
    fn ppm_parsing_allows_rgb_triplet_to_span_lines() {
        let ppm = "P3
        1 1
        255
        51
        153
        204
        ";
        let canvas = canvas_from_ppm(ppm.as_bytes()).unwrap();
        assert_eq!(canvas.pixel_at(0, 0), color!(0.2, 0.6, 0.8));
    }

    #[test]
    fn ppm_parsing_skips_empty_lines() {
        let ppm = "
        P3

        1 1

        255

        51

        153
        204
        ";
        let canvas = canvas_from_ppm(ppm.as_bytes()).unwrap();
        assert_eq!(canvas.pixel_at(0, 0), color!(0.2, 0.6, 0.8));
    }
}
