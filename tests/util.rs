use std::io;

/// Parse a string as a usize type
pub fn parse_usize(s: &str) -> io::Result<usize> {
    s.parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, format!("{}: {:?}", s, e)))
}

/// Parser state
#[derive(PartialEq, Eq)]
pub enum State {
    Name,
    Descr,
    Fields,
}
