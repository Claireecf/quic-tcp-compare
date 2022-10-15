#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Json;
use rocket::Request;
use rocket_contrib::templates::Template;
use rocket::fs::{FileServer, relative, TempFile};
use serde::Serialize;

mod utils;
use crate::utils::{download,upload};

use std::io::{Repeat, Take};
use std::io::Cursor;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

type LimitedRepeat = Take<Repeat>;

const FILENAME: &str = "panda.png";

#[derive(FromForm, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String
}

#[get("/")]
fn index() -> Template {
  #[derive(Serialize)]
  struct Context {
    first_name: String,
    last_name: String
  }
  let context = Context {
    first_name: String::from("Fan"),
    last_name: String::from("Liu")
  };
  Template::render("home", context)
}
#[get("/downloadpage")]
fn downloadpage() -> Template {
  #[derive(Serialize)]
  struct Context {
    first_name: String,
    last_name: String
  }
  let context = Context {
    first_name: String::from("Fan"),
    last_name: String::from("Liu")
  };
  Template::render("downloadpage", context)
}

#[get("/hi/<name>")]
fn hi(name: String) -> Template {
  #[derive(Serialize)]
  struct Context {
    first_name: String,
    // last_name: String
  }
  let context = Context {
    first_name: name.clone(),
    // last_name: name
  };
  Template::render("hi", context)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'success',
    'message': 'Hello API!'
  }")
}

#[tokio::main]
#[get("/file")]
async fn fetch_file() -> Result<()> {
  let file_name = "download.png".to_string();
  let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string();
  let response = reqwest::get(target).await?;
  println!("will be located under: '{:?}'", file_name);
  let mut file = std::fs::File::create(file_name)?;
  let mut content =  Cursor::new(response.bytes().await?);
  std::io::copy(&mut content, &mut file)?;
  Ok(())
}

#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner();
    let mut dummy_db: Vec<Book> = Vec::new();
    dummy_db.push(book);
    format!("Book added successfully: {:?}", dummy_db)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
    rocket::ignite()
      .register(catchers![not_found])
      .mount("/", routes![index])
      .mount("/api", routes![hello, new_book])
      .mount("/", routes![fetch_file])
      .mount("/", routes![hi])
      .mount("/", routes![downloadpage])
      .mount("/public", FileServer::from("/file"))
      .attach(Template::fairing())
      .launch();
}
