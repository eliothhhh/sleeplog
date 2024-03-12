mod journal;
mod article;
mod tag;

use crate::journal::Journal;
use crate::article::Article;
use crate::tag::Tag;

fn main() {
    let mut j = Journal::new("test");
    let a = Article::new("foo", "abcd", vec![]);
    
    println!("{}", a.to_string());
    j.add_article(a);
    println!("{}", j.to_string());
    
}
