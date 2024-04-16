use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, Print};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::fs::File;
use std::io::BufReader;
use std::{
    fs,
    io::{self, Write},
};

use crate::buffer::Buffer;

#[derive(Debug)]
enum Mode {
    Normal,
    Insert,
}

enum Action {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    ModeType(Mode),
}

impl Mode {}

pub struct Editor {
    buffer: Buffer,
    column: u16,
    row: u16,
    terminal_top: u16,
    terminal_left: u16,
    mode: Mode,
    stdout: io::Stdout,
    window_size: (u16, u16),
}

impl Editor {
    pub fn new(buffer: Buffer) -> Editor {
        Editor {
            buffer,
            column: 0,
            row: 0,
            terminal_top: 0,
            terminal_left: 0,
            mode: Mode::Normal,
            stdout: io::stdout(),
            window_size: terminal::size().unwrap(),
        }
    }
    fn terminal_height(&mut self) -> u16 {
        self.window_size.1 - 2
    }
    fn terminal_width(&mut self) -> u16 {
        self.window_size.0
    }

    fn generate_line(&mut self) -> anyhow::Result<()> {
        let str_mode = format!(" {:?}", self.mode);
        let positions = format!(" {:?}:{:?} ", self.row, self.column);

        let size = terminal::size().unwrap();
        let _ = self.stdout.queue(cursor::MoveTo(0, size.1 - 2));

        self.stdout.queue(style::PrintStyledContent(
            str_mode
                .bold()
                .with(Color::Rgb { r: 0, g: 0, b: 0 })
                .on(Color::Rgb {
                    r: 180,
                    g: 144,
                    b: 244,
                }),
        ))?;
        self.stdout.queue(style::PrintStyledContent(
            positions
                .with(Color::Rgb { r: 0, g: 0, b: 0 })
                .on(Color::Rgb {
                    r: 184,
                    g: 144,
                    b: 244,
                }),
        ))?;

        self.stdout.queue(cursor::MoveTo(self.column, self.row))?;
        let _ = self.stdout.flush();
        Ok(())
    }

    fn v_line(&mut self, index: u16) -> Option<String> {
        let line = self.terminal_top + index;

        self.buffer.get_line(line.into())
    }
    fn display_file(&mut self) -> anyhow::Result<()> {
        //let width = self.terminal_width() as usize;
        let buffer_lines = self.buffer.lines.clone();
        for (i, line) in buffer_lines.iter().enumerate() {
            self.stdout.queue(cursor::MoveTo(0, i as u16))?;
            self.stdout.queue(style::Print(line))?;
        }

        Ok(())
    }

    fn get_line_length(&mut self) -> u16 {
        let line = self.terminal_top + self.row;
        let line = self.buffer.get_line(line as usize);

        match line {
            Some(line) => line.len() as u16,
            None => 0,
        }
    }

    fn handle_event(&mut self, event: Event) -> anyhow::Result<Option<Action>> {
        if matches!(event, Event::Resize(_, _)) {
            self.window_size = terminal::size()?;
        }
        match self.mode {
            Mode::Normal => self.handle_normal_mode(event),
            Mode::Insert => self.handle_insert_mode(event),
        }
    }
    fn handle_normal_mode(&mut self, event: Event) -> anyhow::Result<Option<Action>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('j') | KeyCode::Down => Ok(Some(Action::MoveDown)),
                KeyCode::Char('k') | KeyCode::Up => Ok(Some(Action::MoveUp)),
                KeyCode::Char('h') | KeyCode::Left => Ok(Some(Action::MoveLeft)),
                KeyCode::Char('l') | KeyCode::Right => Ok(Some(Action::MoveRight)),
                KeyCode::Char('q') => Ok(Some(Action::Quit)),
                KeyCode::Char('i') => {
                    self.stdout.queue(cursor::SetCursorStyle::BlinkingBar)?;
                    Ok(Some(Action::ModeType(Mode::Insert)))
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
    fn handle_insert_mode(&mut self, event: Event) -> anyhow::Result<Option<Action>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => {
                    self.stdout.queue(cursor::SetCursorStyle::SteadyBlock)?;
                    Ok(Some(Action::ModeType(Mode::Normal)))
                }

                KeyCode::Backspace => {
                    if self.column > 0 {
                        self.column -= 1;
                        self.stdout
                            .queue(cursor::MoveLeft(1))?
                            .queue(style::Print(" "))?;
                        self.stdout.flush()?;
                    }
                    Ok(None)
                }
                KeyCode::Delete => {
                    println!("delete");
                    Ok(None)
                }
                KeyCode::Enter => {
                    self.row += 1;
                    self.column = 0;
                    Ok(None)
                }
                KeyCode::Char(ch) => {
                    self.stdout.queue(style::Print(ch))?;
                    let next = self.column + 1;
                    self.column = next;
                    Ok(None)
                }
                KeyCode::Up => Ok(Some(Action::MoveUp)),
                KeyCode::Right => Ok(Some(Action::MoveRight)),
                KeyCode::Down => Ok(Some(Action::MoveDown)),
                KeyCode::Left => Ok(Some(Action::MoveLeft)),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn handle_broundries(&mut self) {
        if self.column >= self.get_line_length() {
            if self.get_line_length() > 0 {
                self.column = self.get_line_length() - 1;
            } else {
                self.column = 0;
            }
        }
        if self.column >= self.terminal_width() {
            self.column = self.terminal_width() - 1;
        }
    }

    pub fn init(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;

        loop {
            //self.handle_broundries();
            self.display_file()?;
            self.generate_line()?;
            // println!("what is row {} ", self.row);
            if let Some(action) = self.handle_event(read()?)? {
                match action {
                    Action::MoveUp => {
                        if self.row > 0 {
                            self.row -= 1;
                        }
                    }
                    Action::MoveDown => {
                        self.row += 1;
                        /* if self.row >= self.terminal_height() {
                            println!("is this even runnninggg");
                            self.terminal_top += 1;
                            self.row -= 1;
                        } */
                    }
                    Action::MoveLeft => {
                        if self.column > 0 {
                            self.column -= 1;
                        }
                    }
                    Action::MoveRight => {
                        self.column += 1;
                    }
                    Action::ModeType(mode_type) => self.mode = mode_type,
                    Action::Quit => break,
                }
            }
        }
        terminal::disable_raw_mode()?;
        self.stdout.flush()?;
        Ok(())
    }
}
