use aggregator::{self, Summary};

pub fn main(){
    let article = aggregator::NewsArticle {
        author: String::from("Agya Boat"),
        location: String::from("Asamankese, Ghana"),
        title: String::from("Rust 1.60 Released"),
        content: String::from("The latest version of Rust has been released with exciting new features."),
    };
    println!("The summary of the article is: {}", article.summarize());
}