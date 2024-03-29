use std::fmt;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

use crate::api::article::Article;


#[derive(Serialize, Deserialize, Debug)]
pub struct Journal {
    pub (self) name: String,
    pub (self) length: u32,
    pub (self) content: BTreeMap<u32, Article>
}


impl Journal {

    // constructor
    pub fn new(name: &str) -> Self
    {
        return Journal {
            name: String::from(name),
            content: BTreeMap::new(),
            length: 0
        }
    }

    // get
    pub fn get_article(&self, key: &u32) -> Option<(&u32, &Article)>
    {
        return self.content.get_key_value(key);
    }

    // add
    pub fn add_article(&mut self, article: Article) -> u32
    {
        self.content.insert(
            self.length,
            article
        );
        self.length += 1;
        return self.length;
    }

    // remove
    pub fn remove_article(&mut self, key: &u32)
    {
        self.content.remove(key);
    }
    
}


impl fmt::Display for Journal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{:#?}", self)
    }
}
