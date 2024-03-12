mod journal;
mod article;
mod tag;

use crate::journal::Journal;
use crate::article::Article;
use crate::tag::Tag;

fn main() {
    let j: Journal = Journal::new("test");
    println!("Hello, world!");
}
