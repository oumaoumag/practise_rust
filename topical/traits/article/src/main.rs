use article::*;


fn main() {
    let article = NewArticle {
        headline: "Penguins win the Stanley Cup!".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "Larry King".to_string(),
        content: "The pittsburgh Penguins beat the Detroit Red Wings 4-3 in overtime to win the Stanley Cup Finals.".to_string(),
    };

    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "just setting up my twttr".to_string(),
        retweet: false,
        reply: false,
    };

    display_summary(&article);
    display_summary(&tweet);
}