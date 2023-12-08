use rust_mtg_json::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("Enter title:\n");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("Enter a body for {}:\nPress {} when finished.", title, EOF);
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("\nSaved draft: {} (ID: {})", title, post.id);
}

#[cfg(windows)]
const EOF: &str = "CTRL+Z";