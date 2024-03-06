use chrono::{Date, DateTime, Utc};

struct Article {
    title: String,
    content: String,
    date: DateTime<Utc>,
}


impl Article {

    // constructor
    fn new(title: &str, content: &str) -> Article
    {
        return Article {
            title: String::from(title),
            content: String::from(content),
            date: Utc::now()
        };
    }

}