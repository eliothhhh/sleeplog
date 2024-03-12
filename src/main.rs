mod journal;
mod article;
mod tag;

use chrono::Utc;

use crate::journal::Journal;
use crate::article::Article;


fn main() {
    
    let mut j = Journal::new("test");
    let a = Article::new("foo", vec![], Utc::now(), "abcd");
    
    println!("{}", a.to_string());
    j.add_article(a);
    println!("{}", j.to_string());
    
}
