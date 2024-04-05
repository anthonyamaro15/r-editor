use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, Print};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::{
    fs,
    io::{self, Write},
};

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
    column: u16,
    row: u16,
    mode: Mode,
    stdout: io::Stdout,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            column: 0,
            row: 0,
            mode: Mode::Normal,
            stdout: io::stdout(),
        }
    }

    fn generate_line(&mut self) -> anyhow::Result<()> {
        let str_mode = format!(" {:?}", self.mode);
        let positions = format!(" {:?}:{:?} ", self.row, self.column);

        let size = terminal::size().unwrap();
        let _ = self.stdout.queue(cursor::MoveTo(0, size.1 - 5));

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

    fn handle_event(&mut self, event: Event) -> anyhow::Result<Option<Action>> {
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
                KeyCode::Char(ch) => {
                    self.stdout.queue(style::Print(ch))?;
                    let next = self.column + 1;
                    self.column = next;
                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
    pub fn init(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;

        loop {
            self.generate_line()?;
            if let Some(action) = self.handle_event(read()?)? {
                match action {
                    Action::MoveUp => {
                        if self.row > 0 {
                            self.row -= 1;
                        }
                    }
                    Action::MoveDown => {
                        self.row += 1;
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
