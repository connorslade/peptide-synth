use std::ops::BitOr;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
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

    pub fn horizontal(&self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }
}

impl Directions {
    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn contains(&self, direction: Direction) -> bool {
        self.inner & 1 << direction as u8 != 0
    }

    pub fn iter(self) -> impl Iterator<Item = Direction> {
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
