pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

pub struct Vector2D {
    x: f64,
    y: f64,
    length: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Vector2D {
    pub fn get_point(self) -> Point2D {
        Point2D {
            x: self.x,
            y: self.y,
        }
    }
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            length: (x * x + y * y).sqrt(),
        }
    }
    pub fn get_length(self) -> f64 {
        self.length
    }
    pub fn invert(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
    }
    pub fn from_polar(length: f64, degrees: f64) -> Self {
        let radians = degrees / 180.0 * std::f64::consts::PI;
        Self {
            x: radians.sin() * length,
            y: radians.cos() * length,
            length,
        }
    }
}
