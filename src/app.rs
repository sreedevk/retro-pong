use crate::frame::{Frame, HEIGHT};
use crate::game::Game;
use anyhow::Result;
use std::io::{stdin, stdout, StdoutLock, Write};
use std::process::exit;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

pub enum Signal {
    Kill,
    Player1Up,
    Player1Down,
    Player2Up,
    Player2Down,
    Tick,
}

pub struct App {
    stout: RawTerminal<StdoutLock<'static>>,
    app_frame: Frame,
    game: Game,
    tx: std::sync::mpsc::Sender<Signal>,
    rx: std::sync::mpsc::Receiver<Signal>,
}

impl App {
    pub fn new() -> Self {
        let stout = stdout();
        let stout: RawTerminal<StdoutLock> = stout.lock().into_raw_mode().unwrap();
        let (tx, rx) = mpsc::channel();

        Self {
            stout,
            app_frame: Frame::new(),
            game: Game::new(),
            tx,
            rx,
        }
    }

    pub fn kill(&mut self) {
        write!(self.stout, "{}", termion::clear::All).unwrap();
        write!(self.stout, "{}", termion::style::Reset).unwrap();
        write!(self.stout, "{}", termion::cursor::Show).unwrap();
        exit(0)
    }

    pub fn init(&mut self) -> Result<()> {
        write!(self.stout, "{}", termion::cursor::Hide)?;
        let tx_channel = self.tx.clone();
        thread::spawn(move || loop {
            for key in stdin().keys() {
                match key.unwrap() {
                    termion::event::Key::Char('q') => {
                        tx_channel.send(Signal::Kill).unwrap_or_default();
                    }
                    termion::event::Key::Char('j') => {
                        tx_channel.send(Signal::Player2Down).unwrap_or_default();
                    }
                    termion::event::Key::Char('k') => {
                        tx_channel.send(Signal::Player2Up).unwrap_or_default();
                    }
                    termion::event::Key::Char('a') => {
                        tx_channel.send(Signal::Player1Down).unwrap_or_default();
                    }
                    termion::event::Key::Char('s') => {
                        tx_channel.send(Signal::Player1Up).unwrap_or_default();
                    }
                    _ => {
                        tx_channel.send(Signal::Tick).unwrap_or_default();
                    }
                }
            }
        });

        loop {
            self.draw()?;
            self.tick()?;
            self.render()?;
            thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn process_signal(&mut self) -> Result<()> {
        match self.rx.try_recv() {
            Ok(signal) => match signal {
                Signal::Kill => {
                    self.kill();
                }
                Signal::Tick => {}
                Signal::Player1Up => {
                    self.game.player1.move_up()?;
                }
                Signal::Player1Down => {
                    self.game.player1.move_down()?;
                }
                Signal::Player2Up => {
                    self.game.player2.move_up()?;
                }
                Signal::Player2Down => {
                    self.game.player2.move_down()?;
                }
            },
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => self.kill(),
        }

        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        self.game.ball.tick();
        self.process_signal()?;
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

        write!(
            self.stout,
            "{}player1: ({},{}) ({}, {})",
            termion::cursor::Goto(0, (HEIGHT + 1) as u16),
            self.game.player1.x1,
            self.game.player1.y1,
            self.game.player1.x2,
            self.game.player1.y2
        )?;

        self.stout.flush().unwrap();
        Ok(())
    }
}
