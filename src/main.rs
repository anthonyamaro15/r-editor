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
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

impl Mode {}

fn handle_event(mode: &Mode, event: Event) -> Option<Action> {
    match mode {
        Mode::Normal => handle_normal_mode(event),
        Mode::Insert => handle_insert_mode(event),
    }
}

fn handle_normal_mode(event: Event) -> Option<Action> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Char('j') => Some(Action::MoveDown),
            KeyCode::Char('k') => Some(Action::MoveUp),
            KeyCode::Char('h') => Some(Action::MoveLeft),
            KeyCode::Char('l') => Some(Action::MoveRight),
            _ => None,
        },
        _ => None,
    }
}

fn handle_insert_mode(_event: Event) -> Option<Action> {
    todo!()
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mode = Mode::Normal;
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    loop {
        let hand_event = handle_event(&mode, read()?);

        if let Some(action) = hand_event {
            match action {
                Action::MoveUp => println!("Move up"),
                Action::MoveDown => println!("Move down"),
                Action::MoveLeft => println!("Move left"),
                Action::MoveRight => println!("Move right"),
                _ => (),
            }
        }
    }
    stdout.flush()?;
    Ok(())
}
