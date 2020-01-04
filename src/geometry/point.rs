use std::f64;

pub struct Point{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new() -> Point {
        Point{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn unset() -> Point {
        Point{
            x: f64::MIN,
            y: f64::MIN,
            z: f64::MIN
        }
    }

    pub fn from_values(x: f64, y: f64, z: f64) -> Point {
        Point {
            x: x,
            y: y,
            z: z
        }
    }
}