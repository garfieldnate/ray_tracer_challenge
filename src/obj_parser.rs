use crate::shape::group::GroupShape;
use crate::shape::triangle::Triangle;
use crate::tuple::Tuple;
use std::collections::hash_map::HashMap;
use std::fmt::{self, Display, Formatter};
use std::io::{self, BufRead, BufReader, Read};

pub struct ObjParseResults {
    num_ignored_lines: usize,
    vertices: Vec<Tuple>,
    groups: Option<HashMap<String, GroupShape>>,
}

impl ObjParseResults {
    pub fn get_default_group(&self) -> Option<&GroupShape> {
        match &self.groups {
            Some(groups) => groups.get(""),
            None => None,
        }
    }

    pub fn get_group(&self, group_name: &str) -> Option<&GroupShape> {
        match &self.groups {
            Some(groups) => groups.get(group_name),
            None => None,
        }
    }

    pub fn take_all_as_group(&mut self) -> Option<GroupShape> {
        match self.groups {
            Some(ref mut groups) => {
                let mut all_as_group = GroupShape::new();
                for (_k, v) in groups.drain() {
                    println!("adding as all_as_group child: {:?}", v);
                    all_as_group.add_child(Box::new(v));
                }
                self.groups = None;
                println!("all_as_group: {:?}", all_as_group);
                Some(all_as_group)
            }
            None => None,
        }
    }
}

// TODO: proper parsing errors should also contain the line and column number
#[derive(Debug)]
pub enum ParseError {
    IoError(io::Error),
    ParseFloatError(std::num::ParseFloatError),
    ParseIntError(std::num::ParseIntError),
    MalformedVertex(String),
    MalformedFace(String),
    MalformedGroupDeclaration(String),
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
            ParseError::MalformedGroupDeclaration(ref s) => f.write_str(s),
        }
    }
}

pub fn parse_obj<T: Read>(reader: T) -> Result<ObjParseResults, ParseError> {
    let buf_reader = BufReader::new(reader);
    let mut num_ignored_lines = 0;
    // add one dummy point to simplify processing; OBJ files use 1-indexing
    let mut vertices = vec![point!(0, 0, 0)];
    let mut groups: HashMap<String, GroupShape> = HashMap::new();
    let mut current_group: Option<&mut GroupShape> = None;
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
                if coordinates.len() < 3 {
                    return Err(ParseError::MalformedFace(format!(
                        "Not enough vertices to form a face at line {}; expected 3, found {}",
                        index,
                        coordinates.len()
                    )));
                } else {
                    // current_group = current_group.get_or_insert_with(||{});
                    match current_group {
                        None => {
                            // the default group. We use the empty string because it will be impossible to
                            // accidentally override while parsing the OBJ file.
                            groups.insert("".into(), GroupShape::new());
                            current_group = groups.get_mut("");
                        }
                        _ => {}
                    }
                    for triangle in fan_triangulation(&vertices, &coordinates) {
                        current_group = current_group.map(|g| {
                            g.add_child(Box::new(triangle));
                            g
                        });
                    }
                }
            }
            // parse a group declaration: g GroupName
            Some("g") => match elements.next() {
                Some(name) => {
                    groups.insert(name.to_string(), GroupShape::new());
                    current_group = groups.get_mut(&name.to_string());
                }
                None => {
                    return Err(ParseError::MalformedGroupDeclaration(format!(
                        "Missing group name on line {}",
                        index
                    )));
                }
            },
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
        groups: Some(groups),
    })
}

// Assumptons: chosen_vertices describes a convex polygon (interior angles all < PI/2).
fn fan_triangulation(all_vertices: &[Tuple], chosen_vertices: &[usize]) -> Vec<Triangle> {
    debug_assert!(chosen_vertices.len() > 2);
    // TODO: try replacing this with a fancy windowing function
    let mut triangles = vec![];
    for index in 1..chosen_vertices.len() - 1 {
        let tri = Triangle::new(
            all_vertices[chosen_vertices[0]],
            all_vertices[chosen_vertices[index]],
            all_vertices[chosen_vertices[index + 1]],
        );
        triangles.push(tri);
    }
    triangles
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::shape::Shape;
    use std::fs::File;
    use std::path::PathBuf;

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
        let text = "
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0

        f 1 2 3
        f 1 3 4
        ";
        let results = parse_obj(text.as_bytes()).unwrap();

        let g_children = results.get_default_group().unwrap().get_children().unwrap();
        let t1 = g_children[0].downcast_ref::<Triangle>().unwrap();
        let t2 = g_children[1].downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, results.vertices[1]);
        assert_eq!(t1.p2, results.vertices[2]);
        assert_eq!(t1.p3, results.vertices[3]);

        assert_eq!(t2.p1, results.vertices[1]);
        assert_eq!(t2.p2, results.vertices[3]);
        assert_eq!(t2.p3, results.vertices[4]);
    }

    #[test]
    fn triangulating_polygons() {
        let text = "
        v -1 1 0
        v -1 0 0
        v 1 0 0
        v 1 1 0
        v 0 2 0

        f 1 2 3 4 5
        ";

        let results = parse_obj(text.as_bytes()).unwrap();
        let g_children = results.get_default_group().unwrap().get_children().unwrap();
        let t1 = g_children[0].downcast_ref::<Triangle>().unwrap();
        let t2 = g_children[1].downcast_ref::<Triangle>().unwrap();
        let t3 = g_children[2].downcast_ref::<Triangle>().unwrap();

        assert_eq!(t1.p1, results.vertices[1]);
        assert_eq!(t1.p2, results.vertices[2]);
        assert_eq!(t1.p3, results.vertices[3]);

        assert_eq!(t2.p1, results.vertices[1]);
        assert_eq!(t2.p2, results.vertices[3]);
        assert_eq!(t2.p3, results.vertices[4]);

        assert_eq!(t3.p1, results.vertices[1]);
        assert_eq!(t3.p2, results.vertices[4]);
        assert_eq!(t3.p3, results.vertices[5]);
    }

    fn parse_obj_test_file(file_name: &str) -> ObjParseResults {
        let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "resources/test", file_name]
            .iter()
            .collect();
        let file = File::open(&path).unwrap();
        parse_obj(file).unwrap()
    }

    #[test]
    fn triangles_in_groups() {
        let results = parse_obj_test_file("triangles.obj");
        let g1 = results.get_group("FirstGroup").unwrap();
        let t1 = g1.get_children().unwrap()[0]
            .downcast_ref::<Triangle>()
            .unwrap();
        let g2 = results.get_group("SecondGroup").unwrap();
        let t2 = g2.get_children().unwrap()[0]
            .downcast_ref::<Triangle>()
            .unwrap();

        assert_eq!(t1.p1, results.vertices[1]);
        assert_eq!(t1.p2, results.vertices[2]);
        assert_eq!(t1.p3, results.vertices[3]);

        assert_eq!(t2.p1, results.vertices[1]);
        assert_eq!(t2.p2, results.vertices[3]);
        assert_eq!(t2.p3, results.vertices[4]);
    }

    #[test]
    fn converting_obj_file_to_group() {
        let mut results = parse_obj_test_file("triangles.obj");

        let parent_group = results.take_all_as_group().unwrap();
        let child_groups = parent_group.get_children().unwrap();

        let g1 = child_groups[0].downcast_ref::<GroupShape>().unwrap();
        let g2 = child_groups[1].downcast_ref::<GroupShape>().unwrap();

        let t1 = g1.get_children().unwrap()[0]
            .downcast_ref::<Triangle>()
            .unwrap();
        let t2 = g2.get_children().unwrap()[0]
            .downcast_ref::<Triangle>()
            .unwrap();

        // can only test points the triangles have in common because
        // return ordering is random; TODO: switch to LinkedHashMap. Except LinkedHashMap
        // doesn't implement drain(), so you'll have to send a PR. Except the project
        // is no longer maintained, so you might have to ask for a commit bit.
        assert_eq!(t1.p1, point!(-1, 1, 0));
        // assert_eq!(t1.p2, point!(-1, 0, 0));
        // assert_eq!(t1.p3, point!(1, 0, 0));

        assert_eq!(t2.p1, point!(-1, 1, 0));
        // assert_eq!(t2.p2, point!(1, 0, 0));
        // assert_eq!(t2.p3, point!(1, 1, 0));
    }
}
