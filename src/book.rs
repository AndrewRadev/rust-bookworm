use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{self, Display};

use util::WordIterator;
use text_index::Indexable;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Book {
    filename: PathBuf,
}

impl Book {
    pub fn from_path(path: &Path) -> Self {
        Book { filename: PathBuf::from(path) }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.filename.file_name().unwrap().to_str().unwrap())
    }
}

impl<'a> Indexable for Book {
    type Words = BookWordIterator;

    fn extract_words(&self) -> Self::Words {
        let file = File::open(&self.filename).
            expect(&format!("Couldn't open book: {}", self.filename.display()));
        let reader = BufReader::new(file);

        BookWordIterator::new(self.filename.clone(), reader)
    }
}

pub struct BookWordIterator {
    filename: PathBuf,
    reader: BufReader<File>,
    line_words: Vec<String>,
}

impl BookWordIterator {
    fn new(filename: PathBuf, reader: BufReader<File>) -> Self {
        BookWordIterator {
            filename, reader,
            line_words: vec![],
        }
    }

    fn read_next_line(&mut self) -> Option<Vec<String>> {
        let mut next_line = String::new();
        let bytes_read = self.reader.read_line(&mut next_line).
            expect(&format!("Couldn't read lines from book: {}", self.filename.display()));

        if bytes_read == 0 {
            None
        } else {
            Some(WordIterator::new(&next_line).collect())
        }
    }
}

impl Iterator for BookWordIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(word) = self.line_words.pop() {
            return Some(word);
        }

        self.line_words = self.read_next_line()?;
        while self.line_words.is_empty() {
            self.line_words = self.read_next_line()?;
        }

        self.line_words.pop()
    }
}
