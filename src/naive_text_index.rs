use std::collections::HashSet;
use text_index::Indexable;

pub struct TextIndex<T: Indexable> {
    storage: HashSet<Box<T>>,
}

impl<T: Indexable> TextIndex<T> {
    pub fn new() -> Self {
        TextIndex { storage: HashSet::new() }
    }

    pub fn push(&mut self, indexable: T) {
        self.storage.insert(Box::new(indexable));
    }

    pub fn search(&self, query: &str) -> HashSet<&T> {
        let mut result = HashSet::new();
        for word in query.extract_words() {
            let matches: HashSet<&T> = self.storage.
                iter().
                filter(|indexable| {
                    debug!("Working on: {:?}", indexable);
                    indexable.extract_words().contains(&word)
                }).
                map(Box::as_ref).
                collect();
            result.extend(matches);
        }
        result
    }
}
