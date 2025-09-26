use engine::exports::nalgebra::Vector2;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> Vector2<i32> {
        match self {
            Direction::Up => Vector2::new(0, 1),
            Direction::Down => Vector2::new(0, -1),
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
        }
    }

    pub fn horizontal(&self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }
}
