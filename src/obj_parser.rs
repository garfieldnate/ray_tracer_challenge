use crate::shape::group::GroupShape;
use crate::shape::triangle::Triangle;
use crate::tuple::Tuple;
use std::fmt::{self, Display, Formatter};
use std::io::{self, BufRead, BufReader, Read};

pub struct ObjParseResults {
    num_ignored_lines: usize,
    vertices: Vec<Tuple>,
    default_group: GroupShape,
}

// TODO: proper parsing errors should also contain the line and column number
#[derive(Debug)]
pub enum ParseError {
    IoError(io::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
    MalformedVertex(String),
    MalformedFace(String),
}
impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> ParseError {
        ParseError::IoError(err)
    }
}
impl From<std::num::ParseFloatError> for ParseError {
    fn from(err: std::num::ParseFloatError) -> ParseError {
        ParseError::ParseFloatError(err)
    }
}
impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> ParseError {
        ParseError::ParseIntError(err)
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ParseError::IoError(ref e) => e.fmt(f),
            ParseError::ParseFloatError(ref e) => e.fmt(f),
            ParseError::ParseIntError(ref e) => e.fmt(f),
            ParseError::MalformedVertex(ref s) => f.write_str(s),
            ParseError::MalformedFace(ref s) => f.write_str(s),
        }
    }
}

pub fn parse_obj<T: Read>(reader: T) -> Result<ObjParseResults, ParseError> {
    let buf_reader = BufReader::new(reader);
    let mut num_ignored_lines = 0;
    // add one dummy point to simplify processing; OBJ files use 1-indexing
    let mut vertices = vec![point!(0, 0, 0)];
    let mut default_group = GroupShape::new();
    for (index, line) in buf_reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        let mut elements = line.split_whitespace();
        match elements.next() {
            // parse a vertex line: v f32 f32 f32
            Some("v") => {
                let coordinates = elements
                    .map(|x| x.parse::<f32>())
                    .collect::<Result<Vec<f32>, std::num::ParseFloatError>>()?;
                if coordinates.len() != 3 {
                    return Err(ParseError::MalformedVertex(format!(
                        "Wrong number of coordinates in vertex at line {}; expected 3, found {}",
                        index,
                        coordinates.len()
                    )));
                } else {
                    vertices.push(point!(coordinates[0], coordinates[1], coordinates[2]))
                }
            }
            // parse a triangle line: vf usize usize usize
            Some("f") => {
                let coordinates = elements
                    .map(|x| x.parse::<usize>())
                    .collect::<Result<Vec<usize>, std::num::ParseIntError>>()?;
                if coordinates.len() != 3 {
                    return Err(ParseError::MalformedFace(format!(
                        "Wrong number of vertices in face at line {}; expected 3, found {}",
                        index,
                        coordinates.len()
                    )));
                } else {
                    let t = Triangle::new(
                        vertices[coordinates[0]],
                        vertices[coordinates[1]],
                        vertices[coordinates[2]],
                    );
                    default_group.add_child(Box::new(t));
                }
            }
            // as-yet unknown command
            Some(_) => {}
            // blank line
            None => {}
        };

        num_ignored_lines += 1;
    }
    Ok(ObjParseResults {
        num_ignored_lines,
        vertices,
        default_group,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::shape::Shape;

    #[test]
    fn ignoring_unrecognized_files() {
        let text = "There was a young lady named Bright
            who traveled much faster than light.
            She set out one day
            in a relative way,
            and came back the previous night.";
        let results = parse_obj(text.as_bytes()).unwrap();

        assert_eq!(results.num_ignored_lines, 5);
    }

    #[test]
    fn vertex_records() {
        let text = "v -1 1 0
        v -1.0000 0.5000 0.0000
        v 1 0 0
        v 1 1 0";
        let results = parse_obj(text.as_bytes()).unwrap();

        assert_eq!(results.vertices.len(), 5);
        assert_eq!(results.vertices[1], point!(-1, 1, 0));
        assert_eq!(results.vertices[2], point!(-1, 0.5, 0));
        assert_eq!(results.vertices[3], point!(1, 0, 0));
        assert_eq!(results.vertices[4], point!(1, 1, 0));
    }

    #[test]
    fn triangle_faces() {
        let file = "
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0

        f 1 2 3
        f 1 3 4
        ";
        let results = parse_obj(file.as_bytes()).unwrap();

        let g_children = results.default_group.get_children().unwrap();
        let t1 = g_children[0].downcast_ref::<Triangle>().unwrap();
        let t2 = g_children[1].downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, results.vertices[1]);
        assert_eq!(t1.p2, results.vertices[2]);
        assert_eq!(t1.p3, results.vertices[3]);

        assert_eq!(t2.p1, results.vertices[1]);
        assert_eq!(t2.p2, results.vertices[3]);
        assert_eq!(t2.p3, results.vertices[4]);
    }
}
