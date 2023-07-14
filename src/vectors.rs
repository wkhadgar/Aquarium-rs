use rand::random;
use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Mul, Neg, Rem, RemAssign, Sub};

#[derive(Debug)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn random_in_radius(r: f64) -> Self {
        let d = (random::<f64>()).sqrt() * r;
        let thetha = (random::<f64>()) * 2.0 * PI;

        Vector2::new(d * f64::cos(thetha), d * f64::sin(thetha))
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x).to_degrees()
    }

    pub fn get_components(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn offset(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn length_sqr(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn dot(&self, other: Vector2) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn mag(self, magnitude: f64) -> Self {
        let scale = magnitude / self.length();
        self * scale
    }

    pub fn norm(self) -> Self {
        self.mag(1.0)
    }
}

impl Clone for Vector2 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Vector2 {}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for Vector2 {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Rem<f64> for Vector2 {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self::Output {
        let mag = self.length();
        if mag < rhs {
            return self;
        }

        self * (rhs / mag)
    }
}

impl RemAssign<f64> for Vector2 {
    fn rem_assign(&mut self, rhs: f64) {
        let mag = self.length();
        if mag < rhs {
            return;
        }

        self.x = self.x * (rhs / mag);
        self.y = self.y * (rhs / mag);
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}