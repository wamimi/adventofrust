use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParseError::*;
        match self {
            NoName => write!(f, "Name field is missing"),
            NoGoodDeeds => write!(f, "Good deeds field is missing"),
            NoBadDeeds => write!(f, "Bad deeds field is missing"),
            InvalidGoodDeeds => write!(f, "Good deeds value is invalid"),
            InvalidBadDeeds => write!(f, "Bad deeds value is invalid"),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Kid { name, niceness }
    }

    pub fn parse_row(csv_row: &str) -> Result<Kid, ParseError> {
        let mut fields = csv_row.split(',');

        // Name field validation
        let name = fields.next().ok_or(ParseError::NoName)?.trim();
        if name.is_empty() {
            return Err(ParseError::NoName);
        }

        // Good deeds field validation
        let good_deeds_str = fields.next().ok_or(ParseError::NoGoodDeeds)?.trim();
        if good_deeds_str.is_empty() {
            return Err(ParseError::NoGoodDeeds);
        }
        let good_deeds = good_deeds_str
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidGoodDeeds)?;

        // Bad deeds field validation
        let bad_deeds_str = fields.next().ok_or(ParseError::NoBadDeeds)?.trim();
        if bad_deeds_str.is_empty() {
            return Err(ParseError::NoBadDeeds);
        }
        let bad_deeds = bad_deeds_str
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidBadDeeds)?;

        Ok(Kid::new(name.to_string(), good_deeds, bad_deeds))
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);
        ratio >= 0.75
    }
}

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}
