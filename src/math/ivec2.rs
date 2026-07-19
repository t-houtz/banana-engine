#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

// -----------------------------
// DEFINED CONSTANTS
// -----------------------------
impl IVec2 {
    pub const ZERO: Self = Self::new(0, 0);
    pub const ONE: Self = Self::new(1, 1);

    pub const UP: Self = Self::new(0, 1);
    pub const DOWN: Self = Self::new(0, -1);

    pub const RIGHT: Self = Self::new(1, 0);
    pub const LEFT: Self = Self::new(-1, 0);
}
impl Default for IVec2 {
    fn default() -> Self {
        Self::ZERO
    }
}

// -----------------------------
// STANDARD OPERATIONS
// -----------------------------
use std::ops::{Add, Sub, Mul, Neg};

impl Add for IVec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for IVec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<i32> for IVec2 {
    type Output = Self;

    fn mul(self, c: i32) -> Self {
        Self::new(self.x * c, self.y * c)
    }
}

impl Neg for IVec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

// -----------------------------
// STANDARD ASSIGNING OPERATIONS
// -----------------------------

use std::ops::{AddAssign, SubAssign, MulAssign};

impl AddAssign for IVec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for IVec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<i32> for IVec2 {
    fn mul_assign(&mut self, c: i32) {
        self.x *= c;
        self.y *= c;
    }
}

// -----------------------------
// GENERAL VECTOR OPERATIONS
// -----------------------------

impl IVec2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn length_squared(self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub fn manhattan_length(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn manhattan_distance(self, other: Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    pub fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

// -----------------------------
// INDEXING
// -----------------------------
use std::ops::{Index, IndexMut};

impl Index<usize> for IVec2 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("IVec index out of bounds: {}", index)
        }
    }
}

impl IndexMut<usize> for IVec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("IVec index out of bounds: {}", index),
        }
    }
}