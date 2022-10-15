use rocket::response::{content, Stream};

use std::io::{repeat, Repeat, Read, Take};
use std::fs::File;

type LimitedRepeat = Take<Repeat>;

// Generate this file using: head -c BYTES /dev/random > big_file.dat
const FILENAME: &str = "panda.png";

#[get("/file")]
fn root() -> content::Plain<Stream<LimitedRepeat>> {
    content::Plain(Stream::from(repeat('a' as u8).take(25000)))
}

#[get("/big_file")]
fn file() -> Option<Stream<File>> {
    File::open(FILENAME).map(|file| Stream::from(file)).ok()
}