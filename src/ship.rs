use crate::point::Point;
use crate::direction::Direction;


#[derive(Debug)]
pub struct Ship {
    point: Point,
    shooting: bool,
    direction: Direction,
}

impl Ship {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            point: Point::new(x, y),
            shooting: false,
            direction: Direction::Nothing,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn get_ship_point(&self) -> Point {
        self.point.clone()
    }

    pub fn get_direction(&self) -> Direction {
        self.direction.clone()
    }

    pub fn slide(&mut self) {
        self.point.transform(self.direction);
    }
}
