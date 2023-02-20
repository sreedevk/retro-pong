use crate::frame::Frame;
use crate::game::Game;
use anyhow::Result;
use std::io::{stdin, stdout, Read, StdoutLock, Write};
use std::process::exit;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct App {
    stout: RawTerminal<StdoutLock<'static>>,
    app_frame: Frame,
    game: Game,
}

impl App {
    pub fn new() -> Self {
        let stout = stdout();
        let stout: RawTerminal<StdoutLock> = stout.lock().into_raw_mode().unwrap();

        Self {
            stout,
            app_frame: Frame::new(),
            game: Game::new(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        write!(self.stout, "{}", termion::cursor::Hide)?;
        loop {
            self.tick()?;
            self.draw()?;
            self.render()?;
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        let stdin = stdin();
        let bytes = stdin.bytes().next();
        if let Some(b) = bytes {
            match b? {
                b'q' => {
                    write!(self.stout, "{}", termion::clear::All)?;
                    write!(self.stout, "{}", termion::style::Reset)?;
                    write!(self.stout, "{}", termion::cursor::Show)?;
                    exit(0)
                }
                b'j' => {
                    self.game.player2.move_down()?;
                }
                b'k' => {
                    self.game.player2.move_up()?;
                }
                b'a' => {
                    self.game.player1.move_down()?;
                }
                b's' => {
                    self.game.player1.move_up()?;
                }
                _ => return Ok(()),
            }
        }

        self.game.ball.tick();
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let mut new_frame = Frame::new();
        self.game.player1.draw(&mut new_frame)?;
        self.game.player2.draw(&mut new_frame)?;
        self.game.ball.draw(&mut new_frame)?;
        self.app_frame = new_frame;

        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        write!(self.stout, "{}", termion::clear::All)?;
        self.app_frame
            .memory
            .iter()
            .enumerate()
            .for_each(|(index, line)| {
                write!(
                    self.stout,
                    "{}{}{}{}",
                    termion::cursor::Goto(0, (index + 1) as u16),
                    String::from_iter(line),
                    termion::style::Bold,
                    termion::style::Reset,
                )
                .unwrap();
            });

        self.stout.flush().unwrap();
        Ok(())
    }
}
