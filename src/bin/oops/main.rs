mod post;
use crate::post::*;

fn main() {
    let post = Post::new("my way of writting post oops");

    let post = post.request_review();

    let post = post.approve();

    println!("Hello, world! OOPS\n{} ", post.content());
}
