use threadpool::ThreadPool;

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

pub struct TextIndex<T: 'static + Indexable + Clone + Send + Sync> {
    storage: Arc<Mutex<HashMap<String, HashSet<Arc<T>>>>>,
    threadpool: ThreadPool,
}

impl<T: 'static + Indexable + Clone + Send + Sync> TextIndex<T> {
    pub fn new() -> Self {
        TextIndex {
            storage: Arc::new(Mutex::new(HashMap::new())),
            threadpool: ThreadPool::new(4),
        }
    }

    pub fn push(&mut self, indexable: T) {
        let storage = Arc::clone(&self.storage);

        self.threadpool.execute(move || {
            let indexable = Arc::new(indexable);
            debug!("Working on: {:?}", indexable);
            let words: HashSet<String> = HashSet::from_iter(indexable.extract_words());

            for word in words {
                let indexable_clone = Arc::clone(&indexable);
                let mut storage = storage.lock().unwrap();
                let entry = storage.entry(word).or_insert(HashSet::new());
                entry.insert(indexable_clone);
            }
        });
    }

    pub fn search(&self, query: &str) -> HashSet<T> {
        let query_words = query.extract_words();
        let mut results = HashSet::new();
        let storage = self.storage.lock().unwrap();

        for word in query_words {
            if let Some(candidate) = storage.get(&word) {
                debug!("Working on: {:?}", candidate);
                for result in candidate {
                    results.insert(result.as_ref().clone());
                }
            }
        }

        results.into_iter().collect()
    }

    pub fn join(&self) {
        self.threadpool.join();
    }
}
