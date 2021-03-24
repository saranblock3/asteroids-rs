use crate::direction::Direction;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn transform(&self, direction: Direction) -> Self {
        let transformation = match direction {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Nothing => (0, 0),
        };

        Self::new(
            Self::transform_value(self.x, transformation.0),
            Self::transform_value(self.y, transformation.1),
        )
    }

    fn transform_value(value: u16, by: i16) -> u16 {
        if by.is_negative() && by.abs() as u16 > value {
            panic!("Transforming {} by {} results in a negative number", value, by)
        } else {
            (value as i16 + by) as u16
        }
    }
}
