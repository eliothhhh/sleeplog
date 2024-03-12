use chrono::{DateTime, Utc};
use crate::tag::Tag;

pub struct Article {
    pub (self) name: String,
    pub (self) content: String,
    pub (self) date: DateTime<Utc>,
    pub (self) tags: Vec<Tag>,
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
