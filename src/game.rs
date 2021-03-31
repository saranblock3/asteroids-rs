use std::io::Stdout;
use crate::ship::Ship;
use crate::command::Command;
use crate::point::Point;
use crate::direction::Direction;
use std::time::{Duration, Instant};
use crossterm::terminal::size;
use rand::Rng;
use crossterm::terminal::ClearType;
use crossterm::terminal::*;
use crossterm::cursor::*;
use crossterm::style::*;
use crossterm::event::*;
use crossterm::ExecutableCommand;

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;



#[derive(Debug)]
pub struct Game {
    stdout: Stdout,
    width: u16,
    height: u16,
    ship: Ship,
    speed: u16,
    score: u16,
    lives: u16,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        Self {
            stdout,
            width,
            height,
            ship: Ship::new(width/2, height),
            speed: 0,
            score: 0,
            lives: 3,
        }
    }

    pub fn run(&mut self) {
        self.prepare_ui();
        self.render();

        let mut done = false;
        while !done {
            let interval = self.calculate_interval();
            let direction = self.ship.get_direction();
            let now = Instant::now();

            while now.elapsed() < interval {
                if let Some(command) = self.get_command(interval - now.elapsed()) {
                    match command {
                        Command::Quit => {
                            done = true;
                            break;
                        }
                        Command::Move(towards) => {
                            self.ship.set_direction(towards);
                            self.ship.slide();
                        }
                        Command::Shoot => {
                        }
                    }
                }
            }

            self.render();
        }
    }

    fn prepare_ui(&mut self) {
        enable_raw_mode().unwrap();
        self.stdout
            .execute(SetSize(self.width + 3, self.height + 3)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Hide).unwrap();
    }

    fn render(&mut self) {
        self.draw_borders();
        self.draw_background();
        self.draw_ship();
    }

    fn draw_borders(&mut self) {
        self.stdout.execute(SetForegroundColor(Color::DarkGrey)).unwrap();
        for y in 0..self.height + 2 {
            self.stdout
                .execute(MoveTo(0, y)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(self.width + 1, y)).unwrap()
                .execute(Print("#")).unwrap();
        }

        for x in 0..self.width + 2 {
            self.stdout
                .execute(MoveTo(x, 0)).unwrap()
                .execute(Print("#")).unwrap()
                .execute(MoveTo(x, self.height + 1)).unwrap()
                .execute(Print("#")).unwrap();
        }

        self.stdout
            .execute(MoveTo(0, 0)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(self.width + 1, self.height + 1)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(self.width + 1, 0)).unwrap()
            .execute(Print("#")).unwrap()
            .execute(MoveTo(0, self.height + 1)).unwrap()
            .execute(Print("#")).unwrap();
    }

    fn draw_background(&mut self) {
        self.stdout.execute(ResetColor).unwrap();
        
        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                self.stdout
                    .execute(MoveTo(x, y)).unwrap()
                    .execute(Print(" ")).unwrap();
            }
        }
    }


    pub fn draw_ship(&mut self) {
        let ship_point = self.ship.get_ship_point();

        let symbol = '^';

        self.stdout
            .execute(MoveTo(ship_point.x + 1, ship_point.y + 1)).unwrap()
            .execute(Print(symbol)).unwrap();
    }

    fn calculate_interval(&self) -> Duration {
        let speed = MAX_SPEED - self.speed;
        Duration::from_millis(
            (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64
        )
    }

    fn get_command(&self, wait_for: Duration) -> Option<Command> {
        let key_event = self.wait_for_key_event(wait_for)?;

        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Command::Quit),
            KeyCode::Char('c') | KeyCode::Char('C') =>
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Some(Command::Quit)
                } else {
                    None
                }
            KeyCode::Right => Some(Command::Move(Direction::Right)),
            KeyCode::Left => Some(Command::Move(Direction::Left)),
            _ => None
        }
    }

    fn wait_for_key_event(&self, wait_for: Duration) -> Option<KeyEvent> {
        if poll(wait_for).ok()? {
            let event = read().ok()?;
            if let Event::Key(key_event) = event {
                return Some(key_event);
            }
        }
        None
    }
}
