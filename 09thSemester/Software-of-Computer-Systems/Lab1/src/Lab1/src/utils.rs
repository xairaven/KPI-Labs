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
