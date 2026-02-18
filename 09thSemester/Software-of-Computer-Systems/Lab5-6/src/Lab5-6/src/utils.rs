pub trait StringExtension {
    fn replace_char(&mut self, index: usize, ch: char);
}

impl StringExtension for String {
    fn replace_char(&mut self, index: usize, ch: char) {
        if index < self.len() {
            let start = self
                .char_indices()
                .nth(index)
                .map(|(i, _)| i)
                .unwrap_or_else(|| panic!("Index ({}) out of bounds.", index));
            let end = self
                .char_indices()
                .nth(index + 1)
                .map(|(i, _)| i)
                .unwrap_or_else(|| panic!("Index ({}) out of bounds.", index + 1));
            self.replace_range(start..end, &ch.to_string());
        }
    }
}

#[derive(Default)]
pub struct StringBuffer {
    buffer: String,
}

impl StringBuffer {
    pub fn add(&mut self, str: String) {
        self.buffer.push_str(&str);
    }

    pub fn add_line(&mut self, line: String) {
        self.buffer.push_str(&line);
        self.buffer.push('\n');
    }

    pub fn get(self) -> String {
        self.buffer
    }
}
