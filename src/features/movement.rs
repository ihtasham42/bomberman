use crate::constants::TILE_SIZE;

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
