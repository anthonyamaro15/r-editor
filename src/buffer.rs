use std::fs;

pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn new(file: &str) -> Buffer {
        let content: Vec<_> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect();

        Buffer { lines: content }
    }

    pub fn get_line(&mut self, index: usize) -> String {
        if self.lines.len() > index {
            let line = self.lines[index].clone();
            //println!("from get_line {}", line);
            return line;
        }
        String::from("")
    }

    pub fn len(&mut self) -> usize {
        self.lines.len()
    }

    pub fn insert_char(&mut self, index: usize, buffer_line: u16, value: char) {
        println!("index: {}, value: {}", index, value);
        if let Some(lines) = self.lines.get_mut(buffer_line as usize) {
            (*lines).insert(index, value)
        }
    }
}
