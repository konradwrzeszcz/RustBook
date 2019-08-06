fn main() {
    let tweet = Tweet();
    tweet.summarize();

    let article = Article();
    article.summarize();
    println!("{}", article.test());

    notify(&tweet);
    notify(&article);
}

struct Tweet();

impl Summary for Tweet {
    fn author(&self) -> String {
        "Tweet author".to_string()
    }
}

struct Article();

impl Summary for Article {
    fn author(&self) -> String {
        "Article author".to_string()
    }
}

trait Summary{
    fn summarize(&self) {
        println!("Author: {}", self.author());
    }

    fn author(&self) -> String;
}

trait Test{
    fn test(&self) -> String {
        "Test".to_string()
    }
}

impl Test for Article{}

fn notify(item: &Summary) {
    println!("This is author: {}", item.author());
}