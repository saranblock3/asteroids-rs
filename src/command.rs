use crate::direction::Direction;

#[derive(Debug)]
pub enum Command {
    Quit,
    Move(Direction),
    Shoot,
}
