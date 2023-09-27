use std::f64::consts::PI;

pub trait Area {
    fn area(&self) -> f64;
}

impl Area for f64 {
    fn area(&self) -> f64 {
        return self * self * PI;
    }
}
