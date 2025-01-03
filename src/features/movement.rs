use crate::constants::TILE_SIZE;

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_delta(&self) -> (f32, f32) {
        match self {
            Direction::Up => (0.0, TILE_SIZE),
            Direction::Down => (0.0, -TILE_SIZE),
            Direction::Left => (-TILE_SIZE, 0.0),
            Direction::Right => (TILE_SIZE, 0.0),
        }
    }
}

pub enum DirectionAxis {
    Horizontal,
    Vertical,
}

impl From<Direction> for DirectionAxis {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up | Direction::Down => DirectionAxis::Vertical,
            Direction::Left | Direction::Right => DirectionAxis::Horizontal,
        }
    }
}
