use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};
use std::hash::Hash;
use std::fmt::Debug;
use std::iter::FromIterator;

use util::WordIterator;

pub trait Indexable: Debug + Eq + Hash {
    type Words: Iterator<Item=String>;
    fn extract_words(&self) -> Self::Words;
}

impl<'a> Indexable for &'a str {
    type Words = WordIterator<'a>;

    fn extract_words(&self) -> Self::Words {
        WordIterator::new(self)
    }
}

pub struct TextIndex<T: Indexable + Clone> {
    storage: Mutex<HashMap<String, HashSet<Arc<T>>>>,
}

impl<T: Indexable + Clone> TextIndex<T> {
    pub fn new() -> Self {
        TextIndex { storage: Mutex::new(HashMap::new()) }
    }

    pub fn push(&mut self, indexable: T) {
        let indexable = Arc::new(indexable);
        let words: HashSet<String> = HashSet::from_iter(indexable.extract_words());

        for word in words {
            let mut storage = self.storage.lock().unwrap();
            let entry = storage.entry(word).or_insert(HashSet::new());
            entry.insert(indexable.clone());
        }
    }

    pub fn search(&self, query: &str) -> HashSet<T> {
        let query_words = query.extract_words();
        let mut results = HashSet::new();
        let storage = self.storage.lock().unwrap();

        for word in query_words {
            if let Some(candidate) = storage.get(&word) {
                debug!("Working on: {:?}", candidate);
                for result in candidate {
                    results.insert((**result).clone());
                }
            }
        }

        results.into_iter().collect()
    }
}
