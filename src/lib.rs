use std::{path::Path, io::{BufReader, BufRead}, fs::File};

use error::SectorFileError;

mod colour;
mod error;






#[derive(Default)]
pub struct Sector {
    errors: Vec<(usize, SectorFileError)>,
    title_text: Vec<String>,
}

impl Sector {
    pub fn try_read_from_file(path: impl AsRef<Path>) -> Result<Sector, SectorFileError> {
        let reader = BufReader::new(File::open(path)?);
        let mut sector = Sector::default();
        for (line_nr, line) in reader.lines().enumerate() {
            let fields: Vec<&str> = line?.split_whitespace().collect();
            if fields.len() == 0 {
                continue;
            }
            if let Err(error) = sector.handle_line(&fields) {
                sector.errors.push((line_nr + 1, error));
            }
        }

        Ok(sector)
    }


    fn handle_line(&mut self, fields: &[&str]) -> Result<(), SectorFileError> {
        if fields[0].starts_with(';') {
            self.
        }
    }
}




fn reassemble_with_spaces(fields: &[&str]) -> String {
    let mut string = String::new();
    let mut fields = fields.iter().peekable();
    while let Some(field) = fields.next() {
        string.push_str(field);
        if fields.peek().is_some() {
            string.push(' ');
        }
    }
    string
}