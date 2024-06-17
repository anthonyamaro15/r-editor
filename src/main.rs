// rows => top to bottom
// columns => left to right
//
// y => top to bottom
// x => left to right

mod buffer;

use std::io::{self, stdout, Write};

use buffer::Buffer;
use crossterm::{
    cursor,
    event::{self, KeyCode},
    execute,
    style::{self, Stylize},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    QueueableCommand,
};

#[derive(Debug, Clone, Copy)]
enum Mode {
    Normal,
    Insert,
}

// rows => top to bottom
// columns => left to right
//
// y => top to bottom
// x => left to right

struct Editor {
    mode: Mode,
    column_position: usize,
    row_position: usize,
}

impl Editor {
    fn new() -> Self {
        Self {
            mode: Mode::Normal,
            column_position: 0,
            row_position: 0,
        }
    }
    fn draw_editor(&mut self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        stdout.flush()?;
        stdout.queue(cursor::MoveTo(
            self.column_position as u16,
            self.row_position as u16,
        ))?;
        stdout.flush()?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    //let mut buffer = Buffer::new();
    let mut editor = Editor::new();

    let mut stdout = stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    loop {
        //print_buffer(&buffer)?;

        editor.draw_editor(&mut stdout)?;

        match editor.mode {
            Mode::Normal => match event::read()? {
                event::Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('i') => {
                        editor.mode = Mode::Insert;
                    }
                    KeyCode::Char('h') => {
                        if editor.column_position > 0 {
                            editor.column_position -= 1;
                        }
                    }
                    KeyCode::Char('l') => {
                        editor.column_position += 1;
                    }

                    KeyCode::Char('k') => {
                        if editor.row_position > 0 {
                            editor.row_position -= 1;
                        }
                    }
                    KeyCode::Char('j') => {
                        editor.row_position += 1;
                    }
                    _ => {}
                },
                _ => {}
            },
            Mode::Insert => {
                todo!()
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}

/* fn print_buffer(buffer: &Buffer) -> anyhow::Result<()> {
    let cursor_position = buffer.get_cursor_position();
    let content = buffer.get_content();

    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All))?;
    print!("{}", content);

    let (col, row) = calculate_position(cursor_position, content);
    execute!(
        io::stdout(),
        crossterm::cursor::MoveTo(col as u16, row as u16)
    )?;

    io::stdout().flush()?;
    Ok(())
}

fn calculate_position(cursor_position: usize, content: &str) -> (u16, u16) {
    let mut row = 0;
    let mut col = 0;

    for (index, c) in content.chars().enumerate() {
        if index == cursor_position {
            break;
        }

        if c == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (col, row)
} */
