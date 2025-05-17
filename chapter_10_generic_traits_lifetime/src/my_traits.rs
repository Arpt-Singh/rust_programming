struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username:String,
    content: String,
    reply: bool,
    retweet: bool,
}

pub trait summary {
    fn summarize(&self) -> String;
}

impl summary for Tweet {
    fn summarize(&self) -> String {
        let content = format!("Tweet by {}: {}",self.username,self.content);
        content
    }
}

impl summary for NewsArticle {
    fn summarize(&self) ->String {
        let content = format!("News by{}:{}",self.author,self.content);
        content
    }
}

fn news_aggregator(source:impl summary) {

    println!("There is a new news in market.");
    println!("{}", source.summarize());
}

pub fn my_tarit_function() {

    let tweet = Tweet {
        username: String::from("arpt"),
        content: String::from("Rust learning"),
        reply: false,
        retweet:false,
    };

    let news_article = NewsArticle {
        author: String::from("Dev arpt"),
        content: String::from("New series started"),
        headline: String::from("Rust land book"),
        location: String::from("U.P"),
    };

    news_aggregator(tweet);

    news_aggregator(news_article);

}