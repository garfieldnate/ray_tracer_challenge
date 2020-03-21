use crate::bounding_box::BoundingBox;
use crate::shape::group::GroupShape;
use crate::shape::shape::Shape;
use crate::shape::smooth_triangle::SmoothTriangle;
use crate::shape::triangle::Triangle;
use crate::tuple::Tuple;
use std::collections::hash_map::HashMap;
use std::fmt::{self, Display, Formatter};
use std::io::{self, BufRead, BufReader, Read};

pub struct ObjParseResults {
    num_ignored_lines: usize,
    vertices: Vec<Tuple>,
    normals: Vec<Tuple>,
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
                    all_as_group.add_child(Box::new(v));
                }
                self.groups = None;
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
    MalformedNormal(String),
    MalformedGroupDeclaration(String),
    UnexpectedSymbol(String),
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
            ParseError::MalformedNormal(ref s) => f.write_str(s),
            ParseError::MalformedGroupDeclaration(ref s) => f.write_str(s),
            ParseError::UnexpectedSymbol(ref s) => f.write_str(s),
        }
    }
}

pub fn parse_obj<T: Read>(reader: T) -> Result<ObjParseResults, ParseError> {
    let buf_reader = BufReader::new(reader);
    let mut num_ignored_lines = 0;
    // add one dummy point to simplify processing; OBJ files use 1-based indexing
    let mut vertices = vec![point!(0, 0, 0)];
    let mut normals = vec![point!(0, 0, 0)];
    let mut groups: HashMap<String, GroupShape> = HashMap::new();
    let mut current_group: Option<&mut GroupShape> = None;
    let mut normalization_finished = false;
    for (index, line) in buf_reader.lines().enumerate() {
        let line = line?;
        let line = line.trim();
        let mut elements = line.split_whitespace();
        match elements.next() {
            // parse a vertex line: v f32 f32 f32
            Some("v") => {
                if normalization_finished {
                    return Err(ParseError::UnexpectedSymbol(format!(
                        "Found vertex at line {}; vertices must all be specified before any faces are specified (so that they \
                            may be normalized before any faces are created)", index)));
                }
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
            // parse a normal line: vn f32 f32 f32
            Some("vn") => {
                let coordinates = elements
                    .map(|x| x.parse::<f32>())
                    .collect::<Result<Vec<f32>, std::num::ParseFloatError>>()?;
                if coordinates.len() != 3 {
                    return Err(ParseError::MalformedNormal(format!(
                        "Wrong number of coordinates in normal vector at line {}; expected 3, found {}",
                        index,
                        coordinates.len()
                    )));
                } else {
                    normals.push(vector!(coordinates[0], coordinates[1], coordinates[2]))
                }
            }
            // parse a triangle line: vf usize usize usize
            // Next: set flag that no more vertices may be read. Normalize all vertices, update tests. Then try making a scene with an OBJ file!
            Some("f") => {
                if !normalization_finished {
                    normalize_vertices(&mut vertices);
                    normalization_finished = true;
                }

                // TODO: throw useful error if normal is specified for some but not all faces in spec
                let face_specs = elements
                    .map(parse_face)
                    .collect::<Result<Vec<FaceParseResults>, ParseError>>()?;
                if face_specs.len() < 3 {
                    return Err(ParseError::MalformedFace(format!(
                        "Not enough vertices to form a face at line {}; expected 3, found {}",
                        index,
                        face_specs.len()
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
                    for triangle in fan_triangulation(&vertices, &normals, &face_specs) {
                        current_group = current_group.map(|g| {
                            g.add_child(triangle);
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
    if !normalization_finished {
        normalize_vertices(&mut vertices);
    }
    Ok(ObjParseResults {
        num_ignored_lines,
        vertices,
        normals,
        groups: Some(groups),
    })
}

struct FaceParseResults {
    vertex: usize,
    texture: Option<usize>,
    normal: Option<usize>,
}

fn parse_face(face_string: &str) -> Result<FaceParseResults, ParseError> {
    let elements = face_string
        .split('/')
        .map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(x.parse::<usize>())
            }
        })
        .map(Option::transpose)
        .collect::<Result<Vec<Option<usize>>, std::num::ParseIntError>>()?;
    match elements[0] {
        Some(vertex) => Ok(FaceParseResults {
            vertex,
            // get() returns an Option<&Option<usize>> here, unfortunately
            texture: elements.get(1).map(|x| *x).flatten(),
            normal: elements.get(2).map(|x| *x).flatten(),
        }),
        None => Err(ParseError::MalformedFace(
            "Missing vertex index".to_string(),
        )),
    }
}

// Modify the vertices so that their min/max values are -1/1 and they are centered at the origin
fn normalize_vertices(vertices: &mut Vec<Tuple>) {
    let mut bounds = BoundingBox::empty();
    // skip index 0, which is a dummy vertex
    for v in &vertices[1..] {
        bounds.add_point(*v);
    }
    println!("bounds: {:?}", bounds);
    let span = bounds.max - bounds.min;
    println!("span: {:?}", span);
    let scale = span.x.max(span.y.max(span.z)) / 2.;
    println!("scale: {:?}", scale);

    for v in vertices[1..].iter_mut() {
        v.x = (v.x - (bounds.min.x + span.x / 2.)) / scale;
        v.y = (v.y - (bounds.min.y + span.y / 2.)) / scale;
        v.z = (v.z - (bounds.min.z + span.z / 2.)) / scale;
    }
}

// Assumptons: chosen_vertices describes a convex polygon (interior angles all < PI/2).
fn fan_triangulation(
    all_vertices: &[Tuple],
    all_normals: &[Tuple],
    face_specs: &[FaceParseResults],
) -> Vec<Box<dyn Shape>> {
    debug_assert!(face_specs.len() > 2);
    let mut triangles: Vec<Box<dyn Shape>> = vec![];
    let using_smooth_triangles = face_specs[0].normal.is_some();

    // TODO: try replacing this with a fancy windowing function
    for index in 1..face_specs.len() - 1 {
        let v1 = all_vertices[face_specs[0].vertex];
        let v2 = all_vertices[face_specs[index].vertex];
        let v3 = all_vertices[face_specs[index + 1].vertex];

        let tri: Box<dyn Shape> = if using_smooth_triangles {
            let n1 = all_normals[face_specs[0].vertex];
            let n2 = all_normals[face_specs[index].vertex];
            let n3 = all_normals[face_specs[index + 1].vertex];
            Box::new(SmoothTriangle::new(v1, v2, v3, n1, n2, n3))
        } else {
            Box::new(Triangle::new(v1, v2, v3))
        };
        triangles.push(tri);
    }
    triangles
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::shape::Shape;
    use crate::shape::smooth_triangle::SmoothTriangle;
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
        v 1 -1 0";
        let results = parse_obj(text.as_bytes()).unwrap();

        assert_eq!(results.vertices.len(), 5);
        assert_eq!(results.vertices[1], point!(-1, 1, 0));
        assert_eq!(results.vertices[2], point!(-1, 0.5, 0));
        assert_eq!(results.vertices[3], point!(1, 0, 0));
        assert_eq!(results.vertices[4], point!(1, -1, 0));
    }

    #[test]
    fn vertices_are_normalized() {
        let text = "v -50 10 20
        v 30 -40 0
        v 10 -20 50
        v -10 30 10";
        let results = parse_obj(text.as_bytes()).unwrap();

        println!("{:?}", results.vertices);
        assert_eq!(results.vertices.len(), 5);
        assert_eq!(results.vertices[1], point!(-1., 0.375, -0.125));
        assert_eq!(results.vertices[2], point!(1., -0.875, -0.625));
        assert_eq!(results.vertices[3], point!(0.5, -0.375, 0.625));
        assert_eq!(results.vertices[4], point!(0., 0.875, -0.375));
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
        v 0 1 1

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
        // TODO: store order of keys in results manually instead
        assert_eq!(t1.p1, point!(-1, 1, 0));
        // assert_eq!(t1.p2, point!(-1, 0, 0));
        // assert_eq!(t1.p3, point!(1, 0, 0));

        assert_eq!(t2.p1, point!(-1, 1, 0));
        // assert_eq!(t2.p2, point!(1, 0, 0));
        // assert_eq!(t2.p3, point!(1, 1, 0));
    }

    #[test]
    fn vertex_normal_records() {
        let text = "vn 0 0 1
            vn 0.707 0 -0.707
            vn 1 2 3";
        let results = parse_obj(text.as_bytes()).unwrap();
        assert_eq!(results.normals.len(), 4);
        assert_eq!(results.normals[1], vector!(0, 0, 1));
        assert_eq!(results.normals[2], vector!(0.707, 0, -0.707));
        assert_eq!(results.normals[3], vector!(1, 2, 3));
    }

    #[test]
    fn faces_with_normals() {
        let text = "
            v 0 1 0
            v -1 0 0
            v 1 0 0

            vn -1 0 0
            vn 1 0 0
            vn 0 1 0

            f 1//3 2//1 3//2
            f 1/0/3 2/102/1 3/14/2";
        let results = parse_obj(text.as_bytes()).unwrap();
        let g = results.get_default_group().unwrap();
        let g_children = g.get_children().unwrap();
        let t1 = g_children[0].downcast_ref::<SmoothTriangle>().unwrap();
        let t2 = g_children[1].downcast_ref::<SmoothTriangle>().unwrap();

        let test_data = vec![("t1", t1), ("t2", t2)];
        for (name, triangle) in test_data {
            assert_eq!(triangle.base.p1, results.vertices[1], "{}", name);
            assert_eq!(triangle.base.p2, results.vertices[2], "{}", name);
            assert_eq!(triangle.base.p3, results.vertices[3], "{}", name);
            assert_eq!(triangle.n1, results.normals[1], "{}", name);
            assert_eq!(triangle.n2, results.normals[2], "{}", name);
            assert_eq!(triangle.n3, results.normals[3], "{}", name);
        }
    }
}
