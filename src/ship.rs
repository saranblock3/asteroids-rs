use crate::point::Point;
use crate::direction::Direction;


#[derive(Debug)]
pub struct Ship {
    pub point: Point,
    pub shooting: bool,
    pub direction: Direction,
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
        self.point = self.point.transform(self.direction);
    }
}

#[derive(Debug)]
pub struct Bullet {
    pub point: Point,
    pub direction: Direction,
}

impl Bullet {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            point: Point::new(x, y),
            direction: Direction::Up,
        }
    }

    pub fn shoot(&mut self) {
        self.point = self.point.transform(self.direction);
    }
}

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
