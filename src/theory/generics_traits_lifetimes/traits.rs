// Traits known in many languages as interfaces

//! N.B look orphan rules and Trait coherence

use std::fmt::Debug;
use std::fmt::Display;

trait Summary {
    // declarig a Trait
    // method signatures that describe the behaviors of the types
    // that implement this Trait
    fn summarize(&self) -> String;

    fn read(&self) -> String {
        // default implementation of method, will be called if not overriden
        // String::from("Some text...")
        self.summarize() // default implementation can call other methods of the Trait
    }
}

struct Article {
    // declaring a type
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for Article {
    // implementing Trait for type
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    is_reply: bool,
    is_retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn read(&self) -> String {
        // override default implementation
        self.content.clone()
    }
}

struct Pair<T> {
    one: T,
    two: T,
}

impl<T> Pair<T> {
    fn new(one: T, two: T) -> Self {
        Self { one, two }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // this method will only work with Pairs created with a type T
    // that implements traits Display and PartialOrd
    fn cmp_display(&self) {
        if self.one >= self.two {
            println!("The largest of the pair is first = {}", self.one);
        } else {
            println!("The largest of the pair is second = {}", self.two);
        }
    }
}

// implementing trait ToString for all the types implementing Display trait
// impl<T: Display> ToString for T {
//     // --snip--
// }

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        is_reply: false,
        is_retweet: false,
    };

    let article = Article {
        headline: String::from("Headline"),
        location: String::from("Default city"),
        author: String::from("John Doe"),
        content: String::from("Article content"),
    };

    // println!("New tweet: {}", tweet.summarize());
    // println!("Default method: {}", article.read());
    // println!("Overriden method: {}", tweet.read());

    read_entry(&tweet);
    read_entry(&article);
    // read_entry("article"); -- won't work as &str doesn't implement Summary Trait
}

// declaring a function that works only with parameters
// whose type implement Summary Trait
// syntax sugar for Trait bound (see next example)
fn read_entry(entry: &impl Summary) {
    println!("Content of the entry is: {}", entry.read());
}

// using Trait bound syntax
// function accepts generic (any type, implementing Summary)
fn read_entry2<T: Summary>(entry: &T) {
    println!("Content of the entry is: {}", entry.read());
}

// Trait bound is preferable in complex cases: compare read_entry3 and read entry 4
fn read_entry3(entry: &impl Summary, entry2: &impl Summary) {
    println!("Content of the entry 1 is: {}", entry.read());
    println!("Content of the entry 2 is: {}", entry2.read());
}

fn read_entry4<T: Summary>(entry: &T, entry2: &impl Summary) {
    println!("Content of the entry 1 is: {}", entry.read());
    println!("Content of the entry 2 is: {}", entry2.read());
}

// multiple Trait bounds - fn only accept params implementing both Traits
fn read_entry5(entry: &(impl Summary + Display)) {
    println!("Content of the entry is: {}", entry.read());
}

fn read_entry6<T: Summary + Display>(entry: &T) {
    println!("Content of the entry is: {}", entry.read());
}

// clearer Trait bounds with where clauses
// complex Trait bounds for two params, hard to read
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    777
}

// exactly the same function, but Trait bounds are easy to read due to where clause
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    777
}

// returning types with Traits from function
fn returns_summarizable_tweet() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        is_reply: false,
        is_retweet: false,
    }
}

fn returns_summarizable_article() -> impl Summary {
    Article {
        author: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        location: String::from("location"),
        headline: String::from("Wow"),
    }
}

// return with Traits only works if returning a single type
// following wont work beacuse it can return either of two types
// fn returns_summarizable(want_tweet: bool) -> impl Summary {
//     if want_tweet {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             is_reply: false,
//             is_retweet: false,
//         }
//     } else {
//         Article {
//             author: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             location: String::from("location"),
//             headline: String::from("Wow"),
//         }
//     }
// }
