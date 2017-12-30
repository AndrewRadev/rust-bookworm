use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::fmt::{self, Display};

use util::WordIterator;
use text_index::Indexable;

#[derive(Debug, PartialEq, Eq, Hash)]
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

impl<'a> Indexable<BookWordIterator<'a>> for Book {
    fn extract_words(&self) -> BookWordIterator {
        let file = File::open(&self.filename).
            expect(&format!("Couldn't open book: {}", self.filename.display()));
        let reader = BufReader::new(file);

        BookWordIterator::new(self.filename, reader)
    }
}

struct BookWordIterator<'a> {
    filename: PathBuf,
    reader: BufReader<File>,
    words: WordIterator<'a>,
    next_line: Option<String>
}

impl<'a> BookWordIterator<'a> {
    fn new(filename: PathBuf, reader: BufReader<File>) -> Self {
        BookWordIterator {
            filename, reader,
            words: WordIterator::new(""),
            next_line: None,
        }
    }
}

impl<'a> Iterator for BookWordIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        return None;
        //if let Some(word) = self.words.next() {
        //    return Some(word);
        //}

        //self.next_line = self.reader.lines().next()?.
        //    expect(&format!("Couldn't read lines from book: {}", self.filename.display()));
        //self.words = WordIterator::new(self.next_line);
        //self.words.next()
    }
}
