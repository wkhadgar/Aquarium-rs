use std::ops::{Add, Mul, Rem, Sub};

#[derive(Debug)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    pub fn length_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(&self, other: Vector2) -> f64 {
        self.y * other.y + self.x * other.x
    }

    pub fn norm(self) -> Vector2 {
        let scale = 1.0 / self.length();
        self * scale
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vector2 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Rem<f64> for Vector2 {
    type Output = Vector2;

    fn rem(self, rhs: f64) -> Self::Output {
        let mag = self.length();
        if mag < rhs {
            return self;
        }

        self * (rhs / mag)
    }
}