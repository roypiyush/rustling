pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // default behaviour
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
}


fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // for item in list {
    //     if item > largest {
    //         largest = item;
    //     }
    // }

    largest
}


fn compare<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result: &i32 = largest(&number_list);
    println!("The largest number is {}", result);

    let string1 = String::from("abcd");
    let string2 = "xyz";


    println!("The largest string is {}", compare(string1.as_str(), string2));
}
