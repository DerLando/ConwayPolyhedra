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
}