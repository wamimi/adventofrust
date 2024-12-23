// the deref debacle
use std::ops::Deref;
const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

impl Deref for SnowKg {
   type Target = f64;
   
   fn deref(&self) -> &Self::Target {
       &self.0
   }
}

impl Deref for SnowLb {
   type Target = f64;
   
   fn deref(&self) -> &Self::Target {
       &self.0
   }
}

impl Deref for Snowball {
   type Target = i64;
   
   fn deref(&self) -> &Self::Target {
       &self.0
   }
}
// 1. Implement the `Deref` trait for `SnowKg`
// 2. Implement the `Deref` trait for `SnowLb`
// 3. Implement the `Deref` trait for `Snowball`

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl From<SnowKg> for Snowball {
    fn from(kg: SnowKg) -> Self {
        let snowballs = (*kg / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball(snowballs)
    }
}

impl From<SnowLb> for Snowball {
    fn from(lb: SnowLb) -> Self {
        let snowballs = (*lb / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball(snowballs)
    }
}
