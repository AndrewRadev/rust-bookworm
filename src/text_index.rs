use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use std::hash::Hash;
use std::fmt::Debug;

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
        let query_words = query.extract_words();
        let mut results = HashSet::new();

        for candidate in query_words.filter_map(|word| self.storage.get(&word)) {
            debug!("Working on: {:?}", candidate);
            for result in candidate {
                results.insert(result);
            }
        }

        results.into_iter().map(Rc::as_ref).collect()
    }
}
