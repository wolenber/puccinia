use output::Output;

/// Enumeration of versions
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Version {
    /// Version 1.0
    V10,
}

impl Output for Version {
    fn output(&self) -> String {
        use self::Version::*;
        match *self {
            V10 => "PDF-1.0",
        }.to_owned()
    }
}
