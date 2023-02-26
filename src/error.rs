


pub enum SectorFileError {
    InvalidColour(String),
    IOError(std::io::Error),
}

impl From<std::io::Error> for SectorFileError {
    fn from(value: std::io::Error) -> Self {
        SectorFileError::IOError(value)
    }
}

