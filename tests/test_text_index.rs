extern crate bookworm;
use bookworm::text_index::*;

macro_rules! set {
    ($($item:expr),*) => {
        {
            let mut hash_set = ::std::collections::HashSet::new();
            $( hash_set.insert($item); );*
            hash_set
        }
    };
}

#[test]
fn test_search_word() {
    let mut index = TextIndex::new();
    index.push("one, two, three");
    index.push("two, neon");
    index.push("one/five/six");
    index.push("five, six, seven");

    assert_eq!(set!{&"one/five/six", &"one, two, three"}, index.search("one"));
    assert_eq!(set!{&"five, six, seven", &"one/five/six"}, index.search("six"));
}

#[test]
fn test_search_multiple_words() {
    let mut index = TextIndex::new();
    index.push("one, two");
    index.push("two, three");
    index.push("four, five");

    assert_eq!(set!{&"four, five", &"one, two"}, index.search("one + four"));
}
