use crate::frame::{Frame, HEIGHT, WIDTH};
use crate::graphics;
use anyhow::Result;

#[derive(Debug)]
pub struct Game {
    pub player1: Paddle,
    pub player2: Paddle,
    pub ball: Ball,
}

impl Game {
    pub fn new() -> Self {
        let player1 = Paddle {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 10,
        };

        let player2 = Paddle {
            x1: 127,
            y1: 0,
            x2: 127,
            y2: 10,
        };

        let ball = Ball {
            x: 10,
            y: 10,
            xdirection: 1,
            ydirection: 1,
        };

        Self {
            player1,
            player2,
            ball,
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

impl Ball {
    pub fn tick(&mut self) {
        if self.x >= (WIDTH - 1) {
            self.xdirection = -1;
        }

        if self.x <= 1 {
            self.xdirection = 1;
        }

        if self.y >= HEIGHT {
            self.ydirection = -1;
        }
        if self.y <= 0 {
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
        if self.y1 > 0 {
            self.y1 -= 1;
            self.y2 -= 1;
        }
        Ok(())
    }

    pub fn move_down(&mut self) -> Result<()> {
        if self.y2 < HEIGHT {
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
