#[derive(Debug)]
pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // self init
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    pub fn fit_inside(&self, other: &Circle) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        //(dx^2 + dy^2).sqrt() + other.radius <= self.radius
        let distance = (dx * dx + dy * dy).sqrt();
        distance + other.radius <= self.radius
    }
}
