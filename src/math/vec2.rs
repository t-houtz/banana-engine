#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

// -----------------------------
// DEFINED CONSTANTS
// -----------------------------
impl Vec2 {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0);

    pub const UP: Self = Self::new(0.0, 1.0);
    pub const DOWN: Self = Self::new(0.0, -1.0);

    pub const RIGHT: Self = Self::new(1.0, 0.0);
    pub const LEFT: Self = Self::new(-1.0, 0.0);
}
impl Default for Vec2 {
    fn default() -> Self {
        Self::ZERO
    }
}

// -----------------------------
// STANDARD OPERATIONS
// -----------------------------
use std::ops::{Add, Sub, Mul, Div, Neg};

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, c: f32) -> Self {
        Self::new(self.x * c, self.y * c)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, c: f32) -> Self {
        Self::new(self.x / c, self.y / c)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

// -----------------------------
// STANDARD ASSIGNING OPERATIONS
// -----------------------------

use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, c: f32) {
        self.x *= c;
        self.y *= c;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, c: f32) {
        self.x /= c;
        self.y /= c;
    }
}

// -----------------------------
// GENERAL VECTOR OPERATIONS
// -----------------------------
use std::f32::consts::PI;

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn distance(self, other: Self) -> f32 {
        self.distance_squared(other).sqrt()
    }

    pub fn distance_squared(self, other: Self) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx * dx + dy * dy
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalized(self) -> Self {
        let length: f32 = self.length();
        if length == 0.0 {
            Self::ZERO
        }
        else {
            self / length
        }
    }

    pub fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() < 1e-6
    }

    pub fn projected_onto(self, other: Self) -> Self {
        let denom = other.dot(other);
        if denom == 0.0 {
            Self::ZERO
        }
        else {
            let c: f32 = self.dot(other) / denom;
            other * c
        }
    }

    pub fn perpendicular(self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }

    pub fn angle_rads(self) -> f32 {
        self.y.atan2(self.x)
    }
    pub fn angle_degs(self) -> f32 {
        self.angle_rads() * 180.0/PI
    }

    pub fn rotate_rads(self, rads: f32) -> Self {
        let cos: f32 = rads.cos();
        let sin: f32 = rads.sin();

        Self::new(
            self.x * cos - self.y * sin,
            self.x * sin + self.y * cos
        )
    }
    pub fn rotate_degs(self, degs: f32) -> Self {
        let rads: f32 = degs * PI/180.0;
        self.rotate_rads(rads)
    }
}

// -----------------------------
// INDEXING
// -----------------------------
use std::ops::{Index, IndexMut};

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 index out of bounds: {}", index)
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2 index out of bounds: {}", index),
        }
    }
}