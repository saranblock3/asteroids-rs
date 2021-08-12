use crate::point::Point;
use crate::direction::Direction;

// This is a test comment

#[derive(Debug)]
pub struct Asteroid {
    pub point: Point,
    pub direction: Direction,
}

impl Asteroid {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            point: Point::new(x, y),
            direction: Direction::Down,
        }
    }

    pub fn fall(&mut self) {
        self.point = self.point.transform(self.direction);
    }
}
