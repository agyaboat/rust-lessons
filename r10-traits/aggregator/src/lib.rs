pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub title: String,
    pub content: String,
    pub author: String,
    pub location: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.title, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize (&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}