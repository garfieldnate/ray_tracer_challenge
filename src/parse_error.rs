use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    details: String,
    source: Option<Box<dyn Error>>,
}

impl ParseError {
    pub fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
            source: None,
        }
    }
}

impl From<Box<dyn Error>> for ParseError {
    fn from(err: Box<dyn Error>) -> Self {
        ParseError {
            details: err.as_ref().description().to_string(),
            source: Some(err),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)?;
        match &self.source {
            Some(err) => write!(f, ", source: {}", err.as_ref()),
            None => Ok(()),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(boxed_error) => Some(boxed_error.as_ref()),
            None => None,
        }
    }
    fn description(&self) -> &str {
        &self.details
    }
}
