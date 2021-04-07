use std::io::Stdout;
use crate::ship::{Ship, Bullet, Asteroid};
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
    original_terminal_size: (u16, u16),
    ship: Ship,
    speed: u16,
    score: u16,
    lives: u16,
}

impl Game {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let original_terminal_size: (u16, u16) = size().unwrap();
        Self {
            stdout,
            width,
            height,
            original_terminal_size,
            ship: Ship::new(0, height - 1),
            speed: 10,
            score: 0,
            lives: 3,
        }
    }

    pub fn run(&mut self) {
        self.prepare_ui();
        self.render();

        let mut rng = rand::thread_rng();
        let mut asteroid = Asteroid::new(rng.gen_range(0, self.width), 0);

        let mut bullet = Bullet::new(self.ship.point.x, self.height);

        let mut done = false;
        while !done {
            if self.ship.shooting == false {
                bullet.point.x = self.ship.point.x;
            }
            let interval = self.calculate_interval();
            let direction = self.ship.get_direction();
            let now = Instant::now();
            self.draw_asteroid(&asteroid);
            asteroid.fall();
            if asteroid.point.y == self.height {
                asteroid.point.x = rng.gen_range(0, self.width);
                asteroid.point.y = 0;
            }
            self.draw_bullet(&bullet);


            while now.elapsed() < interval {
                if let Some(command) = self.get_command(interval - now.elapsed()) {
                    match command {
                        Command::Quit => {
                            done = true;
                            break;
                        }
                        Command::Move(towards) => {
                            self.ship.set_direction(towards);
                            if self.ship.point.x > 0 && self.ship.point.x < self.width - 1 {
                                self.ship.slide();
                            }
                            else if self.ship.point.x == 0 && self.ship.direction == Direction::Right || self.ship.point.x == self.width - 1 && self.ship.direction == Direction::Left {
                                self.ship.slide();
                            }

                        }
                        Command::Shoot => {
                            self.ship.shooting = true;
                        }
                    }
                }
            }

            if self.ship.shooting == true {
                bullet.shoot();
            }


            self.render();
        }

        self.restore_ui();
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

    pub fn draw_asteroid(&mut self, asteroid: &Asteroid) {
        let asteroid_point = asteroid.point;

        let symbol = '*';

        self.stdout
            .execute(MoveTo(asteroid_point.x + 1, asteroid_point.y + 1)).unwrap()
            .execute(Print(symbol)).unwrap();
    }

    fn draw_bullet(&mut self, bullet: &Bullet) {
        let bullet_point = bullet.point;

        let symbol = '"';

        self.stdout
            .execute(MoveTo(bullet_point.x + 1, bullet_point.y + 1)).unwrap()
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
            KeyCode::Char('f') => Some(Command::Shoot),
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

    fn restore_ui(&mut self) {
        let (cols, rows) = self.original_terminal_size;
        self.stdout
            .execute(SetSize(cols, rows)).unwrap()
            .execute(Clear(ClearType::All)).unwrap()
            .execute(Show).unwrap()
            .execute(ResetColor).unwrap();
        disable_raw_mode().unwrap();
    }
}
