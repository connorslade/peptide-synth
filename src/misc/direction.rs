use std::ops::BitOr;

use engine::exports::nalgebra::Vector2;
use serde::Serialize;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Clone, Copy, Serialize)]
pub struct Directions {
    inner: u8,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn horizontal(&self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    pub fn delta(&self) -> Vector2<i32> {
        match self {
            Direction::Up => Vector2::new(0, 1),
            Direction::Down => Vector2::new(0, -1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        }
    }

    pub fn from_delta(delta: Vector2<i32>) -> Option<Self> {
        match delta.as_slice() {
            [1, 0] => Some(Direction::Right),
            [-1, 0] => Some(Direction::Left),
            [0, 1] => Some(Direction::Up),
            [0, -1] => Some(Direction::Down),
            _ => None,
        }
    }
}

impl Directions {
    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn contains(&self, direction: Direction) -> bool {
        self.inner & 1 << direction as u8 != 0
    }

    pub fn iter(self) -> impl Iterator<Item = Direction> + Clone {
        Direction::ALL
            .into_iter()
            .filter(move |&x| self.contains(x))
    }
}

impl BitOr<Direction> for Directions {
    type Output = Self;

    fn bitor(self, rhs: Direction) -> Self::Output {
        Self {
            inner: self.inner | 1 << rhs as u8,
        }
    }
}
