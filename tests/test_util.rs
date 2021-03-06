extern crate bookworm;
use bookworm::util::*;

#[test]
fn test_word_iterator() {
    let words = WordIterator::new("The <cat> is (in the) </bag>").collect::<Vec<String>>();
    assert_eq!(vec!["The", "cat", "is", "in", "the", "bag"], words);
}
