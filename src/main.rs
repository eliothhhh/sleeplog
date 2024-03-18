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
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    
    // placeholder
    let a = Article::new(
        "test", 
        Utc::now(), 
        DreamType::Normal, 
        vec![Tag::new("foo", [255, 0, 0])], 
        "abcdefghijklmnopqrstuvqxyz"
    );

    let ui = AppWindow::new()?;
    ui.run()
}