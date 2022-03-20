use csv::Position;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::time::Instant;
use types::{Index, IndexRecord};

use crate::types::Book;

mod types;

fn build_index_from_data() -> Result<Index, Box<dyn Error>> {
    let file = File::open("books.csv")?;
    let buffer = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(buffer);
    let mut raw_record = csv::StringRecord::new();
    let headers = reader.headers()?.clone();

    let mut index = Vec::new();
    let mut start_position = 0;

    while reader.read_record(&mut raw_record)? {
        let book: Book = raw_record.deserialize(Some(&headers))?;
        let index_record = IndexRecord {
            offset: start_position,
            bookId: book.bookId,
        };

        index.push(index_record);
        start_position = reader.position().byte();
    }

    Ok(index)
}

fn save_index() -> Result<(), Box<dyn Error>> {
    let index = build_index_from_data().unwrap();

    let file = File::create("index.csv")?;
    let buffer = BufWriter::new(file);
    let mut writer = csv::Writer::from_writer(buffer);

    for record in index {
        writer.serialize(record)?;
    }

    Ok(())
}

fn build_index_from_file() -> Result<Index, Box<dyn Error>> {
    let file = File::open("index.csv")?;
    let buffer = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(buffer);

    let mut index = Vec::new();
    for result in reader.deserialize() {
        let record: IndexRecord = result?;
        index.push(record);
    }

    Ok(index)
}

fn get_element_from_file(offset: Option<u64>, book_id: String) -> Result<Book, Box<dyn Error>> {
    let file = File::open("books.csv")?;
    let buffer = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(buffer);
    let mut raw_record = csv::StringRecord::new();
    let headers = reader.headers()?.clone();

    match offset {
        None => {}
        Some(offset) => {
            let mut pos = Position::new();
            pos.set_byte(offset);
            reader.seek(pos)?;
        }
    }

    while reader.read_record(&mut raw_record)? {
        let book: Book = raw_record.deserialize(Some(&headers))?;

        if book.bookId == book_id {
            return Ok(book);
        }
    }

    Err("not found".into())
}

fn find_using_index(book_id: String) -> Result<Book, Box<dyn Error>> {
    let index = build_index_from_file().unwrap();
    let mut offset = 0;
    for i in index {
        if i.bookId == book_id {
            offset = i.offset
        }
    }

    get_element_from_file(Some(offset), book_id)
}

fn find_without_index(book_id: String) -> Result<Book, Box<dyn Error>> {
    get_element_from_file(None, book_id)
}

fn main() {
    let mut args = Vec::new();

    for argument in env::args() {
        if argument == "--build" {
            println!("Building index of books.csv to index.csv");
            save_index().unwrap();
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

    let id = args.get(1).unwrap();

    println!("Finding {}...\n", id);

    let now = Instant::now();
    let res = find_without_index(id.to_string()).unwrap();
    println!(
        "WITHOUT INDEX RESULTS: passed as {:?} seconds too far",
        now.elapsed().as_secs()
    );
    println!("===================================================");
    println!("{:?}\n", res);

    let now = Instant::now();
    let res = find_using_index(id.to_string()).unwrap();
    println!(
        "WITH INDEX RESULTS: passed as {:?} seconds too far",
        now.elapsed().as_secs()
    );
    println!("===============================================");
    println!("{:?}\n", res);
}
