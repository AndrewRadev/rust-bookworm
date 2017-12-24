extern crate time;
extern crate rustyline;
#[macro_use]
extern crate searcher;

use std::ffi::OsString;
use std::path::Path;
use std::fs;
use std::io;

use rustyline::error::ReadlineError;

use searcher::book::Book;
use searcher::naive_text_index::TextIndex;

fn main() {
    let mut books = vec![];
    find_books(Path::new("/home/andrew/test_books"), &mut books).
        expect("Couldn't read books from given directory");

    let mut book_index = TextIndex::new();
    measure!({
        println!("\n> Pushing into index...");
        for book in books {
            debug!("Working on: {}", book);
            book_index.push(book);
        }
    });

    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline("> ") {
            Ok(query) => {
                let search_results = measure!({
                    println!("\n> Searching for query: {}", query);
                    book_index.search(&query)
                });

                println!("\n> Results: ");
                for result in search_results {
                    println!("{}", result);
                }
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
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
