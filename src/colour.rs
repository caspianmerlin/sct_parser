use std::str::FromStr;
use crate::error::SectorFileError;

const MAX_COLOUR: u32 = 0xFFFFFF;
const RED_MASK: u32 = 0xFF;
const GREEN_MASK: u32 = 0xFF00;
const BLUE_MASK: u32 = 0xFF0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TryFrom<u32> for Colour {
    type Error = SectorFileError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > MAX_COLOUR {
            return Err(SectorFileError::InvalidColour(value.to_string()));
        }

        Ok(
            Colour {
                r: (value & RED_MASK) as u8,
                g: ((value & GREEN_MASK) >> 8) as u8,
                b: ((value & BLUE_MASK) >> 16) as u8,
            }
        )
    }
}

impl FromStr for Colour {
    type Err = SectorFileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: u32 = s.parse().map_err(|_| SectorFileError::InvalidColour(s.to_string()))?;
        Self::try_from(num)
    }
}