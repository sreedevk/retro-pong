use crate::frame::{Frame, HEIGHT, WIDTH};
use crate::graphics::{self, PIXON};
use anyhow::Result;

#[derive(Debug)]
pub struct Game {
    pub player1: Paddle,
    pub player2: Paddle,
    pub ball: Ball,
    pub container: Container,
}

impl Game {
    pub fn new() -> Self {
        let player1 = Paddle {
            x1: 2,
            y1: 2,
            x2: 2,
            y2: 12,
        };

        let player2 = Paddle {
            x1: WIDTH - 3,
            y1: 2,
            x2: WIDTH - 3,
            y2: 12,
        };

        let ball = Ball {
            x: 10,
            y: 10,
            xdirection: 1,
            ydirection: 1,
        };

        let container = Container::new();

        Self {
            player1,
            player2,
            ball,
            container
        }
    }
}

#[derive(Debug)]
pub struct Ball {
    x: usize,
    y: usize,
    xdirection: isize,
    ydirection: isize,
}

#[derive(Debug)]
pub struct Container {
    x: usize,
}

impl Container {
    pub fn new() -> Self {
        Self { x: WIDTH / 2 }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        (0..frame.memory.len())
            .filter(|x| x % 2 == 0)
            .into_iter()
            .for_each(|my| {
                frame.memory[my][self.x] = PIXON;
            });

        Ok(())
    }
}

impl Ball {
    pub fn tick(&mut self) {
        if self.x >= (WIDTH - 4) {
            self.xdirection = -1;
        }

        if self.x <= 3 {
            self.xdirection = 1;
        }

        if self.y >= HEIGHT - 2 {
            self.ydirection = -1;
        }
        if self.y <= 1 {
            self.ydirection = 1;
        }

        self.x = (self.x as isize + self.xdirection) as usize;
        self.y = (self.y as isize + self.ydirection) as usize;
    }

    pub fn draw(&self, frame: &mut Frame) -> Result<()> {
        graphics::draw_rect(frame, self.y, self.x, 1, 1);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Paddle {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl Paddle {
    pub fn move_up(&mut self) -> Result<()> {
        if self.y1 > 2 {
            self.y1 -= 1;
            self.y2 -= 1;
        }
        Ok(())
    }

    pub fn move_down(&mut self) -> Result<()> {
        if self.y2 < (HEIGHT - 2) {
            self.y1 += 1;
            self.y2 += 1;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) -> Result<()> {
        graphics::draw_line(
            frame,
            self.x1 as isize,
            self.y1 as isize,
            self.x2 as isize,
            self.y2 as isize,
        );

        Ok(())
    }
}
