#![allow(dead_code)]
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2 {
    x: isize,
    y: isize,
}
impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Vec2 {
    type Output = Self;

    /// Performs the `-` operation.
    ///
    /// # Example
    /// ```rust
    /// assert_eq!(12 - 1, 11);
    /// ```
    ///
    /// # Panics
    /// ```rust
    /// if x_1 - x_2 < 0 or y_1 - y_2 < 0
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul for Vec2 {
    type Output = isize;

    /// returns Dot-product of the 2 Vec's
    fn mul(self, rhs: Self) -> Self::Output {
        self.dot_prod(&rhs)
    }
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn len(&self) -> f64 {
        f64::sqrt(((self.x ^ 2) + (self.y ^ 2)) as f64)
    }
    pub fn normal(&self) -> Self {
        let len = self.len();
        Self::new(
            ((self.x as f64) / len) as isize,
            ((self.y as f64) / len) as isize,
        )
    }
    pub fn angle(&self, rhs: &Self) -> f64 {
        (self.dot_prod(rhs) as f64 / (self.len() * rhs.len())).acos()
    }
    pub fn dot_prod(&self, rhs: &Self) -> isize {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
    pub fn area(&self) -> isize {
        self.x * self.y
    }
}

#[cfg(test)]
mod vec2_tests {
    use super::*;

    #[test]
    fn add_vec2s() {
        let vec1 = Vec2 { x: 2, y: 3 };
        let vec2 = Vec2 { x: 3, y: 2 };

        let vec3 = vec1 + vec2;
        assert_eq!(vec3, Vec2 { x: 5, y: 5 });
    }
    #[test]
    fn sub_vec2s() {
        let vec1 = Vec2 { x: 5, y: 3 };
        let vec2 = Vec2 { x: 3, y: 1 };

        let vec3 = vec1 - vec2;
        assert_eq!(vec3, Vec2 { x: 2, y: 2 });
    }
    #[test]
    fn mul_vec2s() {
        let vec1 = Vec2 { x: 5, y: 3 };
        let vec2 = Vec2 { x: 3, y: 1 };

        let mul = vec1 * vec2;
        let dot = vec1.dot_prod(&vec2);
        assert_eq!(mul, dot);
    }
    #[test]
    fn dot_vec2s() {
        let vec1 = Vec2 { x: 5, y: 3 };
        let vec2 = Vec2 { x: 3, y: 1 };

        let dot = vec1.dot_prod(&vec2);
        assert_eq!(dot, 18);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Ship {
    shape: Vec<Vec2>,
    pos: Vec2,
}

impl Ship {
    pub fn new(shape: Vec<Vec2>, pos: Vec2) -> Self {
        Self { shape, pos }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Board {
    ships: Vec<Ship>,
    size: Vec2,
}

impl Board {
    pub fn new(ships: Vec<Ship>, size: Vec2) -> Self {
        Self { ships, size }
    }
}
