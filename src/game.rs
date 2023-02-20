use crate::frame::Frame;
use crate::graphics;
use anyhow::Result;

pub struct Game {
    pub player1: Paddle,
    pub player2: Paddle,
    pub ball: Ball
}

impl Game {
    pub fn new() -> Self {
        let player1 = Paddle {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 10
        };

        let player2 = Paddle {
            x1: 127,
            y1: 0,
            x2: 127,
            y2: 10
        };

        let ball = Ball {
            x: 64,
            y: 16,
            xdirection: 1,
            ydirection: 1
        };

        Self {
            player1,
            player2,
            ball
        }
    }
}

pub struct Ball {
    x: usize,
    y: usize,
    xdirection: isize,
    ydirection: isize
}

impl Ball {
    pub fn tick(&mut self) {
        self.x = (self.x as isize + self.xdirection) as usize;
        self.y = (self.y as isize + self.ydirection) as usize;
    }

    pub fn draw(&self, frame: &mut Frame) -> Result<()> {
        graphics::draw_rect(frame, self.x, self.y, 2, 2);
        Ok(())
    }
}

pub struct Paddle {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Paddle {
    pub fn move_up(&mut self) -> Result<()> {
        self.y1 -= 1;
        self.y2 -= 1;
        Ok(())
    }

    pub fn move_down(&mut self) -> Result<()> {
        self.y1 += 1;
        self.y2 += 1;
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
