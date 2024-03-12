use std::fmt;

use chrono::{DateTime, Utc};
use crate::tag::Tag;


#[derive(Debug)]
pub struct Article {
    pub (self) name: String,
    pub (self) content: String,
    pub (self) date: DateTime<Utc>,
    pub (self) tags: Vec<Tag>,
}


impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "<Article>\nName: {}\nDate: {}\nTags: {:?}\n\n{}", self.name, self.date, self.tags, self.content)
    }
}


impl Article {

    // constructor
    pub fn new(name: &str, content: &str, tags: Vec<Tag>) -> Self
    {
        return Article {
            name: String::from(name),
            content: String::from(content),
            date: Utc::now(),
            tags
        };
    }

}
