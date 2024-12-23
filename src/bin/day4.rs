// Structifying the Naughty List
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    // Moved is_nice as an associated function
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }
        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;
        let ratio = good_deeds / (good_deeds + bad_deeds);
        ratio >= 0.75
    }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };
        Kid {
            name,
            niceness,
        }
    }
}