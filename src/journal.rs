use std::collections::BTreeMap;
use crate::article::Article;


pub struct Journal {
    pub (self) name: String,
    pub (self) content: BTreeMap<u32, Article>,
    pub (self) length: u32
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
    
}