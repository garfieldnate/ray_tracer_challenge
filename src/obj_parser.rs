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
        println!("{}", line.unwrap());
    }
    ObjParseResults { num_ignored_lines }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn ignoring_unrecognized_files() -> io::Result<()> {
        let text = "There was a young lady named Bright
            who traveled much faster than light.
            She set out one day
            in a relative way,
            and came back the previous night.";

        // Create a file inside of `std::env::temp_dir()`.
        let mut file1 = NamedTempFile::new()?;
        // Re-open it.
        let mut file2 = file1.reopen()?;

        // Write some test data to the first handle.
        file1.write_all(text.as_bytes())?;

        let results = parse_obj(&mut file2);

        assert_eq!(results.num_ignored_lines, 5);

        Ok(())
    }
}
