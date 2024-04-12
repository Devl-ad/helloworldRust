use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more..")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} , by {}  ", self.headline, self.author)
    }
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{} , by {}  ", self.content, self.username)
    // }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize())
}

pub fn noti_fy<T: Summary>(item: &T) {
    println!("Breaking news {}", item.summarize())
}
/*

pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news {}", item.summarize())
}

pub fn noti_fy<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news {}", item.summarize())
}


pub fn notify(item: (&impl Summary + Display)) {
   println!("Breaking news {}", item.summarize())
}
pub fn noti_fy<T: Summary +Display>(item: &T) {
   println!("Breaking news {}", item.summarize())
}

fn some_fun<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

 */

fn some_fun<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

fn main() {
    let tweet = Tweet {
        username: String::from("james"),
        content: String::from("The eclipse"),
        reply: false,
        retweet: true,
    };

    notify(&tweet);

    println!("{}", tweet.summarize());
}
