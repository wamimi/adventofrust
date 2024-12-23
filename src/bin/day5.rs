// Parsing naughty-list.csv
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
   Nice(u32),
   Naughty,
}

pub struct Kid {
   pub name: String, 
   pub niceness: Niceness,
}

impl Kid {
   pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
       let mut fields = csv_row.split(',');
       
       let name = fields
           .next()
           .ok_or("Missing name field")?
           .to_string();
       
       let good_deeds = fields
           .next()
           .ok_or("Missing good deeds field")?
           .parse()
           .map_err(|_| "Invalid good deeds")?;
       
       let bad_deeds = fields
           .next()
           .ok_or("Missing bad deeds field")?
           .parse()
           .map_err(|_| "Invalid bad deeds")?;
       
       Ok(Self::new(name, good_deeds, bad_deeds))
   }

   pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
       let niceness = if Self::is_nice(good_deeds, bad_deeds) {
           Niceness::Nice(good_deeds)
       } else {
           Niceness::Naughty
       };
       Self { name, niceness }
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