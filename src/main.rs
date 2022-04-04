use std::{env, error::Error, time::Instant};

use csv::Position;
use types::{Book, IndexElement};

mod types;

const BOOKS_FILE_NAME: &str = "books.csv";
const INDEX_FILE_NAME: &str = "index.csv";

fn measure_and_print(
    func: &dyn Fn(String, u64) -> Result<Book, Box<dyn Error>>,
    book_id: String,
    offset: u64,
) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    func(book_id, offset)?;

    println!("{} OFFSET: took {} nanos", offset, now.elapsed().as_nanos());
    return Ok(());
}

fn find_book(book_id: String, offset: u64) -> Result<Book, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(BOOKS_FILE_NAME)?;

    if offset > 0 {
        let mut pos = Position::new();
        pos.set_byte(offset);
        reader.seek(pos)?;
    }

    for element in reader.deserialize() {
        let book: Book = element?;
        if book.bookId == book_id {
            return Ok(book);
        }
    }

    return Err("not found".into());
}

fn build_and_save_index() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(BOOKS_FILE_NAME)?;
    let mut writer = csv::Writer::from_path(INDEX_FILE_NAME)?;

    let mut raw_record = csv::StringRecord::new();
    let mut offset = 0;
    let headers = reader.headers()?.clone();

    while reader.read_record(&mut raw_record)? {
        let book: Book = raw_record.deserialize(Some(&headers))?;
        let index_element = IndexElement {
            bookId: book.bookId,
            offset: offset,
        };

        writer.serialize(index_element)?;
        offset = reader.position().byte();
    }

    Ok(())
}

fn find_in_index(book_id: String) -> Result<u64, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(INDEX_FILE_NAME)?;

    for record in reader.deserialize() {
        let element: IndexElement = record?;
        if element.bookId == book_id {
            return Ok(element.offset);
        }
    }

    return Err("not found".into());
}

fn main() {
    let mut args = Vec::new();

    for argument in env::args() {
        if argument == "--build" {
            println!("Building index of books.csv to index.csv");
            build_and_save_index().unwrap();
            println!("Done!");
            return;
        }
        args.push(argument);
    }

    if args.len() < 2 {
        println!("book id as argument is required!");
        return;
    }
    if args.len() > 2 {
        println!("Too much arguments! Needs only one");
        return;
    }

    let book_id = args.get(1).unwrap();

    measure_and_print(&find_book, book_id.to_string(), 0).unwrap();

    let book_offset = find_in_index(book_id.to_string()).unwrap();
    measure_and_print(&find_book, book_id.to_string(), book_offset).unwrap();
}
