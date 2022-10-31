// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// Default implementastion
pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

// Implementing a Trait on a Type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// To use a default implementation to summarize instances of NewsArticle, we specify an empty impl block with impl Summary for NewsArticle {}.
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
