use std::ops::{Add, Sub};
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn north(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    pub fn south(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    pub fn east(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    pub fn west(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    pub fn in_direction(direction: Direction) -> Self {
        match direction {
            Direction::North => Point::new(0, 1),
            Direction::South => Point::new(0, -1),
            Direction::East => Point::new(1, 0),
            Direction::West => Point::new(-1, 0), 
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}
