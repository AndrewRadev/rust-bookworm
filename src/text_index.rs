use std::collections::{HashSet, HashMap};
use std::rc::Rc;
use std::hash::Hash;
use std::fmt::Debug;
use std::marker::PhantomData;

use util::WordIterator;

pub trait Indexable<I: Iterator>: Debug + Eq + Hash {
    fn extract_words(&self) -> I;
}

impl<'a> Indexable<WordIterator<'a>> for String {
    fn extract_words(&self) -> WordIterator<'a> {
        WordIterator::new(self.as_ref())
    }
}

pub struct TextIndex<I: Iterator, T: Indexable<I>> {
    storage: HashMap<String, HashSet<Rc<T>>>,
    phantom: PhantomData<I>,
}

impl<I: Iterator, T: Indexable<I>> TextIndex<I, T> {
    pub fn new() -> Self {
        TextIndex { storage: HashMap::new() }
    }

    pub fn push(&mut self, indexable: T) {
        let indexable = Rc::new(indexable);

        for word in indexable.extract_words() {
            let entry = self.storage.entry(String::from(word)).or_insert(HashSet::new());
            entry.insert(indexable.clone());
        }
    }

    pub fn search(&self, query: &str) -> HashSet<&T> {
        let query_words = query.extract_words();
        let mut results = HashSet::new();

        for candidate in query_words.filter_map(|word| self.storage.get(word)) {
            debug!("Working on: {:?}", candidate);
            for result in candidate {
                results.insert(result);
            }
        }

        results.into_iter().map(Rc::as_ref).collect()
    }
}
