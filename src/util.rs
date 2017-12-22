pub struct WordIterator<'a> {
    source: &'a str,
}

impl<'a> WordIterator<'a> {
    pub fn new(text: &'a str) -> Self {
        WordIterator { source: text }
    }
}

impl<'a> Iterator for WordIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut on_word = false;
        let mut start = None;
        let mut end = None;
        let mut byte_index = 0;

        for c in self.source.chars() {
            if !on_word && c.is_alphabetic() {
                start = Some(byte_index);
                on_word = true;
            } else if on_word && !c.is_alphabetic() {
                end = Some(byte_index);
                on_word = false;
            }

            if let (Some(start), Some(end)) = (start, end) {
                let word = &self.source[start .. end];
                self.source = &self.source[end..];
                return Some(word);
            }

            byte_index += c.len_utf8();
        }

        let start = start?;
        let word = &self.source[start..];
        self.source = "";
        Some(word)
    }
}
