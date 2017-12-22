extern crate searcher;

use std::ffi::OsString;
use std::path::Path;
use std::fs;
use std::io;

use searcher::book::Book;
use searcher::text_index::TextIndex;

fn main() {
    let mut books = vec![];
    find_books(Path::new("/home/andrew/books/calibre"), &mut books).
        expect("Couldn't read books from given directory");

    let mut book_index = TextIndex::new();
    for book in books {
        book_index.push(book);
    }

    let search_results = book_index.search("Alexandria");
    println!("{:?}", search_results);
}

fn find_books(dir: &Path, books: &mut Vec<Book>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find_books(&path, books)?;
            } else {
                let path = entry.path();
                if path.extension() == Some(&OsString::from("txt")) {
                    books.push(Book::from_path(&path))
                }
            }
        }
    }
    Ok(())
}
