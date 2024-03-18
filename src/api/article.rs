use std::fmt;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use words_count::WordsCount;

use crate::api::{
    dreamtype::DreamType,
    tag::Tag
};


#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
    pub (self) name: String,
    pub (self) date: DateTime<Utc>,
    pub (self) dream_type: DreamType,
    pub (self) tags: Vec<Tag>,
    pub (self) content: String
}


impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{:#?}", self)
    }
}


impl Article {

    // constructor
    pub fn new(name: &str, date: DateTime<Utc>, dream_type: DreamType, tags: Vec<Tag>, content: &str) -> Self
    {
        return Article {
            name: String::from(name),
            date,
            dream_type,
            tags,
            content: String::from(content)
        };
    }

    pub fn get_words_count(self) -> WordsCount {
        return words_count::count(self.content);
    }

}
