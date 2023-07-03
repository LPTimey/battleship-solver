#![allow(dead_code)]
use std::{
    collections::HashMap,
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;
use thiserror::Error;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos2 {
    x: usize,
    y: usize,
}

impl Pos2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn distance(&self, rhs: &Self) -> Vec2 {
        Vec2::new(
            rhs.x as isize - self.x as isize,
            rhs.y as isize - self.y as isize,
        )
    }
}
impl Add<Vec2> for Pos2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x - rhs.x as usize, self.y - rhs.y as usize)
    }
}
#[cfg(test)]
mod pos2_tests {
    use crate::structs::Vec2;

    use super::Pos2;

    #[test]
    fn neg_dist() {
        let pos1 = Pos2::new(3, 2);
        let pos2 = Pos2::new(1, 1);
        assert_eq!(Vec2::new(-2, -1), pos1.distance(&pos2))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2 {
    x: isize,
    y: isize,
}
impl From<Pos2> for Vec2 {
    fn from(value: Pos2) -> Self {
        Self {
            x: value.x as isize,
            y: value.y as isize,
        }
    }
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
    pub fn area_rect(&self) -> isize {
        self.x * self.y
    }
    pub fn area_tri(&self) -> f64 {
        (self.x * self.y) as f64 / 2.
    }
    pub fn turn_counter(self) -> Self {
        Self::new(-self.y, self.x)
    }
    pub fn turn_clock(self) -> Self {
        Self::new(self.y, -self.x)
    }
}

#[cfg(test)]
mod vec2_tests {
    use super::Vec2;

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
    #[test]
    fn turn_clock() {
        let vec = Vec2::new(2, 1);
        assert_eq!(Vec2::new(1, -2), vec.turn_clock());
        assert_eq!(Vec2::new(-2, -1), vec.turn_clock().turn_clock());
        assert_eq!(Vec2::new(-1, 2), vec.turn_clock().turn_clock().turn_clock());
    }
    #[test]
    fn turn_counter_clock() {
        let vec = Vec2::new(2, 1);
        assert_eq!(Vec2::new(-1, 2), vec.turn_counter());
        assert_eq!(Vec2::new(-2, -1), vec.turn_counter().turn_counter());
        assert_eq!(
            Vec2::new(1, -2),
            vec.turn_counter().turn_counter().turn_counter()
        );
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ship {
    shape: Vec<Vec2>,
}

#[derive(Debug, Error)]
pub enum ShipError {
    #[error("there needs to be a RootVector(Vec2{{0,0}})")]
    NoRootVec,
}

impl Ship {
    pub fn new(mut shape: Vec<Vec2>) -> Result<Self, ShipError> {
        if !shape.contains(&Vec2::new(0, 0)) {
            shape.push(Vec2::new(0, 0));
            //return Err(ShipError::NoRootVec);
        };
        let shape = shape.into_iter().unique().collect_vec();
        Ok(Self { shape })
    }
    pub fn get_new(shape: Vec<Vec2>) -> Result<[Self; 4], ShipError> {
        let shape = shape.into_iter().unique().collect_vec();
        let norm: Self = Self::new(shape.clone())?;
        let r90: Self = Self::new(
            shape
                .clone()
                .into_iter()
                .map(|vec| vec.turn_counter())
                .collect(),
        )?;
        let r180: Self = Self::new(
            shape
                .clone()
                .into_iter()
                .map(|vec| vec.turn_counter().turn_counter())
                .collect(),
        )?;
        let r270: Self = Self::new(
            shape
                .into_iter()
                .map(|vec| vec.turn_counter().turn_counter().turn_counter())
                .collect(),
        )?;
        Ok([norm, r180, r90, r270])
    }
    pub fn turn_clock(&self) -> Self {
        let Self { shape } = self;

        Self::new(
            shape
                .clone()
                .into_iter()
                .map(|vec| vec.turn_clock())
                .collect_vec(),
        )
        .unwrap()
    }
    pub fn turn_clock_mut(&mut self) {
        self.shape
            .iter_mut()
            .for_each(|vec| *vec = vec.turn_clock());
    }
    pub fn turn_counter(&self) -> Self {
        let Self { shape } = self;

        Self::new(
            shape
                .clone()
                .into_iter()
                .map(|vec| vec.turn_counter())
                .collect_vec(),
        )
        .unwrap()
    }
    pub fn turn_counter_mut(&mut self) {
        self.shape
            .iter_mut()
            .for_each(|vec| *vec = vec.turn_counter());
    }
    pub fn is_equivalent(&self, other: &Self) -> bool {
        let (Ship { shape }, Ship { shape: other }) = (self, other);
        if shape.len() != other.len() {
            return false;
        }
        let mut res: bool = true;
        shape.iter().for_each(|vec| {
            if !other.contains(vec) {
                res = false;
            }
        });
        res
    }
}

#[cfg(test)]
mod ship_tests {
    use itertools::Itertools;

    use super::{Ship, Vec2};

    #[test]
    fn test() {
        let ships = Ship::get_new(vec![
            Vec2::default(),
            Vec2::new(1, 0),
            Vec2::new(0, 1),
            Vec2::new(-1, 0),
            Vec2::new(0, -1),
        ])
        .unwrap();
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Board {
    ships: Vec<(Pos2, Ship)>,
    shape: Vec<Pos2>,
    shots: Vec<Pos2>,
}
#[derive(Debug, Error)]
pub enum BoardErrors {}

impl Board {
    pub fn new(
        ships: Vec<(Pos2, Ship)>,
        shape: Vec<Pos2>,
        shots: Vec<Pos2>,
    ) -> Result<Self, BoardErrors> {
        Ok(Self {
            ships,
            shape,
            shots,
        })
    }

    pub fn size(&self) -> Vec2 {
        let mut max_x = 0;
        let mut max_y = 0;
        self.shape.iter().for_each(|Pos2 { x, y }| {
            if x > &max_x {
                max_x = *x;
            }
            if y > &max_y {
                max_y = *y;
            }
        });
        Vec2::new(max_x as isize, max_y as isize)
    }
    pub fn get_grid(&self) -> Vec<Pos2> {
        self.ships
            .iter()
            .flat_map(|(pos, Ship { shape })| shape.into_iter().map(move |vec| *pos + *vec))
            .collect_vec()
    }
    pub fn get_completion(&self) -> f64 {
        let to_hit = self.get_grid();
        let hit = self
            .shots
            .iter()
            .filter(|pos| to_hit.contains(pos))
            .unique()
            .collect_vec();

        to_hit.len() as f64 / hit.len() as f64
    }
}

#[cfg(test)]
mod board_tests {
    #[test]
    fn test() {
        assert!(true)
    }
}

#[derive(Debug, Default)]
pub struct BoardBuilder {
    ships: Vec<Ship>,
    shape: Vec<Pos2>,
    whitespace: usize,
}

impl BoardBuilder {
    pub fn new(ships: Vec<Ship>, shape: Vec<Pos2>, whitespace: usize) -> Self {
        Self {
            ships,
            shape,
            whitespace,
        }
    }

    pub fn set_ships(&mut self, ships: Vec<Ship>) -> &mut Self {
        self.ships = ships;
        self
    }
    pub fn add_ship(&mut self, ship: Ship) -> &mut Self {
        self.ships.push(ship);
        self
    }

    pub fn set_shape(&mut self, shape: Vec<Pos2>) -> &mut Self {
        self.shape = shape;
        self
    }

    pub fn set_whitespace(&mut self, whitespace: usize) -> &mut Self {
        self.whitespace = whitespace;
        self
    }

    pub fn ships(&self) -> &[Ship] {
        self.ships.as_ref()
    }

    pub fn shape(&self) -> &[Pos2] {
        self.shape.as_ref()
    }

    pub fn whitespace(&self) -> usize {
        self.whitespace
    }

    pub fn ships_mut(&mut self) -> &mut Vec<Ship> {
        &mut self.ships
    }

    pub fn shape_mut(&mut self) -> &mut Vec<Pos2> {
        &mut self.shape
    }

    pub fn whitespace_mut(&mut self) -> &mut usize {
        &mut self.whitespace
    }
}

impl BoardBuilder {
    pub fn build(&mut self) -> Result<Vec<Board>, Vec<BoardErrors>> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum HeatMapErrors {}

#[derive(Debug, Clone)]
pub struct HeatMap(HashMap<Pos2, f64>);

impl TryFrom<Vec<Board>> for HeatMap {
    type Error = HeatMapErrors;

    fn try_from(value: Vec<Board>) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl HeatMap {
    pub fn size(&self) {
        let test = self.0.keys().collect_vec();
    }
}
