use self::models::Post;
use diesel::prelude::*;
use rust_mtg_json::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("Post ID required for get_post")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(post)) => println!("Post ID {} has title {}", post.id, post.title),
        Ok(None) => println!("Unable to find Post ID {}", post_id),
        Err(_) => println!("Error fetching post {}", post_id),
    }
}