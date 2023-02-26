use std::num::ParseFloatError;

use crate::error::Error;



#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position {
    pub lat: f64,
    pub lon: f64,
}

impl Position {
    pub fn try_from_es_format(lat: &str, lon: &str) -> Result<Position, Error> {
        let lat = Self::try_coord_from_es_format(lat)?;
        let lon = Self::try_coord_from_es_format(lon)?;
        Self::new(lat, lon)
    }

    pub fn new(lat: f64, lon: f64) -> Result<Position, Error> {
        if !(-90.0..=90.0).contains(&lat) {
            Err(Error::InvalidLatitude(lat))
        } else if !(-180.0..=180.0).contains(&lon) {
            Err(Error::InvalidLongitude(lon))
        } else {
            Ok(Position {
                lat,
                lon
            })
        }
    }
    
    fn try_coord_from_es_format(coord: &str) -> Result<f64, Error> {
        let minus = coord.starts_with(['S', 'W']);
        let coord_cut = &coord[1..];
        let mut parts = coord_cut.splitn(3, '.');

        let degs: f64 = parts.next().ok_or_else(|| Error::NotEnoughParts)?.parse::<f64>()?;
        let mins: f64 = parts.next().ok_or_else(|| Error::NotEnoughParts)?.parse::<f64>()?;
        let secs: f64 = parts.next().ok_or_else(|| Error::NotEnoughParts)?.parse::<f64>()?;

        let coord = degs + mins / 60.0 + secs / 3600.0;
        if minus {
            Ok(coord * -1.0)
        } else {
            Ok(coord)
        }
    }
}

#[test]
fn test_coord_from_es_format() {
    use float_cmp::approx_eq;
    // OK
    let input = ("N054.39.27.000", "W006.12.57.000");
    let output = Position::try_from_es_format(input.0, input.1);
    let Position { lat, lon } = output.unwrap();
    assert!(approx_eq!(f64, lat, 54.6575, epsilon = 0.00001));
    assert!(approx_eq!(f64, lon, -6.215833, epsilon = 0.00001));

    // Not enough parts
    let input = ("N054.39", "W006.12.57.000");
    let output = Position::try_from_es_format(input.0, input.1);
    println!("{:?}", output);
    assert!(matches!(output, Err(Error::NotEnoughParts)));
    // Not a number
    let input = ("N054.39.27.000", "W006.1g.57.000");
    let output = Position::try_from_es_format(input.0, input.1);
    println!("{:?}", output);
    assert!(matches!(output, Err(Error::NotANumber(_))));
    // Invalid latitude
    let input = ("N091.39.27.000", "W006.12.57.000");
    let output = Position::try_from_es_format(input.0, input.1);
    println!("{:?}", output);
    assert!(matches!(output, Err(Error::InvalidLatitude(_))));

    // Invalid longitude
    let input = ("N054.39.27.000", "W181.12.57.000");
    let output = Position::try_from_es_format(input.0, input.1);
    println!("{:?}", output);
    assert!(matches!(output, Err(Error::InvalidLongitude(_))));
    
}


