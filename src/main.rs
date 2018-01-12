extern crate time;
extern crate rustyline;
extern crate walkdir;
#[macro_use]
extern crate bookworm;

use rustyline::error::ReadlineError;
use walkdir::WalkDir;

use bookworm::book::Book;
use bookworm::text_index::TextIndex;

fn main() {
    let book_directory = match std::env::args().nth(1) {
        Some(dir) => dir,
        None => {
            println!("No directory given.");
            println!("");
            println!("USAGE: bookworm <book-directory>");
            std::process::exit(1);
        },
    };

    let walk_result: Result<Vec<_>, _> = WalkDir::new(&book_directory).into_iter().collect();
    let books: Vec<_> = match walk_result {
        Ok(entries) => {
            entries.into_iter().
                filter(|e| e.path().extension().map(|ext| ext == "txt").unwrap_or(false)).
                map(|e| Book::from_path(e.path())).
                collect()
        },
        Err(e) => {
            println!("Couldn't parse files from directory: {}", book_directory);
            println!("");
            println!("{}", e);
            std::process::exit(1);
        }
    };

    println!("> Found {} books", books.len());

    let mut book_index = TextIndex::new();
    measure!({
        println!("> Pushing into index...");
        for book in books {
            book_index.push(book);
        }
        book_index.join();
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
