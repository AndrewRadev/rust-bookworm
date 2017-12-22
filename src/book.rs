use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

use util;
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

impl Indexable for Book {
    fn extract_words(&self) -> Vec<String> {
        let file = File::open(&self.filename).
            expect(&format!("Couldn't open book: {}", self.filename.display()));
        let reader = BufReader::new(file);
        let mut words = HashSet::new();

        for line in reader.lines() {
            let line = line.expect(&format!("Couldn't read lines from book: {}", self.filename.display()));
            words.extend(util::WordIterator::new(&line).map(String::from))
        }

        words.into_iter().collect()
    }
}
