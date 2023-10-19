use std::f64::consts::PI;

pub fn to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn to_degrees(radians: f64) -> f64 {
    radians / PI * 180.0
}
