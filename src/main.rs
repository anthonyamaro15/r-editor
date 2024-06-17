// rows => top to bottom
// columns => left to right
//
// y => top to bottom
// x => left to right

mod buffer;

use std::io::{self, Write};

use buffer::Buffer;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> anyhow::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut buffer = Buffer::new();

    loop {
        print_buffer(&buffer)?;

        if let event::Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char(c) => {
                    buffer.insert(c);
                }
                KeyCode::Backspace => {
                    //buffer.delete();
                }
                KeyCode::Left => {
                    buffer.move_cursor_left();
                }
                KeyCode::Right => {
                    buffer.move_cursor_right();
                }
                KeyCode::Enter => {
                    buffer.insert('\n');
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn print_buffer(buffer: &Buffer) -> anyhow::Result<()> {
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
}
