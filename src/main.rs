mod api;

// crate imports
use api::{
    article::Article,
    dreamtype::DreamType,
    tag::Tag,
    journal::Journal
};

// dependencies
use chrono::Utc;


fn main() {
    
    // placeholder
    let a = Article::new(
        "test", 
        Utc::now(), 
        DreamType::Normal, 
        vec![Tag::new("foo", [255, 0, 0])], 
        "abcdefghijklmnopqrstuvqxyz"
    );

    let serialized = serde_json::to_string(&a).unwrap();
    println!("{}", serialized);
    
    let deserialized: Article = serde_json::from_str(&serialized).unwrap();
    println!("{}", deserialized.to_string());

    
}
