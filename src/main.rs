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

fn handle_event(stdout: &io::Stdout, mode: &Mode, event: Event) -> anyhow::Result<Option<Action>> {
    match mode {
        Mode::Normal => handle_normal_mode(event),
        Mode::Insert => handle_insert_mode(event, stdout),
    }
}

fn handle_normal_mode(event: Event) -> anyhow::Result<Option<Action>> {
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

// rows => top to bottom
// columns => left to right
//
// y => top to bottom
// x => left to right

fn handle_insert_mode(event: Event, mut stdout: &io::Stdout) -> anyhow::Result<Option<Action>> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Esc => Ok(Some(Action::ModeType(Mode::Normal))),
            KeyCode::Char(ch) => {
                stdout.queue(style::Print(ch))?;
                Ok(None)
            }
            _ => Ok(None),
        },
        _ => Ok(None),
    }
}

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();
    let mut mode = Mode::Normal;
    let mut column = 0;
    let mut row = 0;

    terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    loop {
        stdout.queue(cursor::MoveTo(column, row))?;

        stdout.flush()?;

        if let Some(action) = handle_event(&stdout, &mode, read()?)? {
            match action {
                Action::MoveUp => {
                    if row > 0 {
                        row -= 1;
                    }
                }
                Action::MoveDown => {
                    row += 1;
                }
                Action::MoveLeft => {
                    if column > 0 {
                        column -= 1;
                    }
                }
                Action::MoveRight => {
                    column += 1;
                }
                Action::ModeType(mode_type) => mode = mode_type,
                Action::Quit => break,
            }
        }
    }
    terminal::disable_raw_mode()?;
    stdout.flush()?;
    Ok(())
}
