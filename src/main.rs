use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::{
    fs,
    io::{self, Write},
};

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

struct Editor {
    column: u16,
    row: u16,
    mode: Mode,
    stdout: io::Stdout,
}

impl Editor {
    fn new() -> Editor {
        Editor {
            column: 0,
            row: 0,
            mode: Mode::Normal,
            stdout: io::stdout(),
        }
    }

    fn init(&mut self) -> anyhow::Result<()> {
        terminal::enable_raw_mode()?;
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;

        loop {
            self.stdout.queue(cursor::MoveTo(self.column, self.row))?;

            self.stdout.flush()?;

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
                KeyCode::Char('i') => Ok(Some(Action::ModeType(Mode::Insert))),
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
    fn handle_insert_mode(&mut self, event: Event) -> anyhow::Result<Option<Action>> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Esc => Ok(Some(Action::ModeType(Mode::Normal))),
                KeyCode::Char(ch) => {
                    self.stdout.queue(style::Print(ch))?;
                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}

// rows => top to bottom
// columns => left to right
//
// y => top to bottom
// x => left to right

fn main() -> anyhow::Result<()> {
    let mut editor = Editor::new();
    let _ = editor.init();
    Ok(())
}
