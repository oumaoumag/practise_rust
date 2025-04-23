trait Summary {
    fn summary(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
    fn summary(&self) -> String {
    }
}

pub struct Tweet {
    username: String,
    content: String,
    retweet: bool,
    reply: bool,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn display_summary(item: &dyn Summary) {
    println!("Summary: {}", item.summary());
}