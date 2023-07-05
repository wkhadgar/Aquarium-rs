use std::arch::x86_64::_rdrand64_step;
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

    pub fn from(vec: &Vector2) -> Self {
        Vector2::new(vec.x, vec.y)
    }

    pub fn random_in_radius(r: f64) -> Self {
        let mut bit_theta: u64 = 0;
        let mut bit_d: u64 = 0;
        let thetha;
        let d;
        unsafe {
            _rdrand64_step(&mut bit_theta);
            _rdrand64_step(&mut bit_d);
            thetha = (f64::from_bits(bit_theta) / f64::from_bits(u64::MAX)) * 2 * f64::PI;
            d = (f64::from_bits(bit_d) / f64::from_bits(u64::MAX)).sqrt() * r
        }
        Vector2::new(d * f64::cos(thetha), d * f64::sin(thetha))
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

    pub fn norm(self) -> Self {
        let scale = 1.0 / self.length();
        self * scale
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

impl Sub for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
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