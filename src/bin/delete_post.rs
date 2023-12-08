use diesel::prelude::*;
use rust_mtg_json::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;

    let target = args()
        .nth(1)
        .expect("Expected delete target");

    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting post");

    println!("{} posts deleted.", num_deleted)
}