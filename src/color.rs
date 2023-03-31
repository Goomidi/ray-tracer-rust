use crate::utils::clamp;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, MulAssign},
};

#[derive(PartialEq, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn write_color(&self, samples_per_pixel: u32) {
        let scale = 1.0 / samples_per_pixel as f64;
        println!(
            "{} {} {}",
            (256.0 * clamp(f64::sqrt(self.r * scale), 0.0, 0.999)) as u8,
            (256.0 * clamp(f64::sqrt(self.g * scale), 0.0, 0.999)) as u8,
            (256.0 * clamp(f64::sqrt(self.b * scale), 0.0, 0.999)) as u8
        );
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl MulAssign<Self> for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.0 * self.r) as u8,
            (255.0 * self.g) as u8,
            (255.0 * self.b) as u8
        )
    }
}
