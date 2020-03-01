use std::io::{BufRead, BufReader, Read};

pub struct ObjParseResults {
    num_ignored_lines: usize,
}

pub fn parse_obj<T>(reader: &mut T) -> ObjParseResults
where
    T: Read,
{
    let buf_reader = BufReader::new(reader);
    let mut num_ignored_lines = 0;
    for line in buf_reader.lines() {
        num_ignored_lines += 1;
    }
    ObjParseResults { num_ignored_lines }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignoring_unrecognized_files() {
        let mut text = "There was a young lady named Bright
            who traveled much faster than light.
            She set out one day
            in a relative way,
            and came back the previous night."
            .as_bytes();
        let results = parse_obj(&mut text);

        assert_eq!(results.num_ignored_lines, 5);
    }
}
