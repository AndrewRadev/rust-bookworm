use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use std::hash::Hash;

struct WordIterator<'a> {
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

pub trait Indexable: Eq + Hash {
    fn extract_words(&self) -> Vec<String>;
}

impl Indexable for String {
    fn extract_words(&self) -> Vec<String> {
        WordIterator::new(self).map(String::from).collect()
    }
}

impl<'a> Indexable for &'a str {
    fn extract_words(&self) -> Vec<String> {
        WordIterator::new(self).map(String::from).collect()
    }
}

pub struct TextIndex<T: Indexable> {
    storage: HashMap<String, HashSet<Rc<T>>>,
}

impl<T: Indexable> TextIndex<T> {
    pub fn new() -> Self {
        TextIndex { storage: HashMap::new() }
    }

    pub fn push(&mut self, indexable: T) {
        let indexable = Rc::new(indexable);

        for word in indexable.extract_words() {
            let entry = self.storage.entry(word).or_insert(HashSet::new());
            entry.insert(indexable.clone());
        }
    }

    pub fn search(&self, query: &str) -> HashSet<&T> {
        let query_words = query.extract_words().into_iter();
        let mut results = HashSet::new();

        for candidate in query_words.filter_map(|word| self.storage.get(&word)) {
            for result in candidate {
                results.insert(result);
            }
        }

        results.into_iter().map(Rc::as_ref).collect()
    }
}
