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

    pub fn get_line(&mut self, index: usize) -> Option<String> {
        if self.lines.len() > index {
            let line = self.lines[index].clone();
            Some(line);
        }
        None
    }
}
