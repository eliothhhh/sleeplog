use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub (self) name: String,
    pub (self) color: [u8; 3]
}


impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "[{}]", self.name)
    }
}


impl Tag {

    // constructor
    pub fn new(name: &str, color: [u8; 3]) -> Tag
    {
        return Tag {
            name: String::from(name),
            color
        }
    }

}