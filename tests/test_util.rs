extern crate searcher;
use searcher::util::*;

#[test]
fn test_word_iterator() {
    let words = WordIterator::new("The <cat> is (in the) </bag>").collect::<Vec<&str>>();
    assert_eq!(vec!["The", "cat", "is", "in", "the", "bag"], words);
}
