#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]


use std::{fmt::{format, Display}, thread, time::Duration};

use guessing_game::init;
use log::info;

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

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

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: T) {
    info!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x, 
            y,
        }
    }
}

impl<T> Pair<T> 
    where T: Display + PartialOrd
{
    fn cmp_display(&self) {
        if self.x >= self.y {
            info!("The largest number is x = {}", self.x);
        } else {
            info!("The largest number is y = {}", self.y);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

fn simulated_expendsive_calculation(intensity: u32) -> u32 {
    info!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn genrate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expendsive_calculation(intensity);

    let mut expensive_closure = Cacher::new(|num| {
        info!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        info!("Today, do {} pushups!", expensive_closure.value(intensity));
        info!("Next, do {} situps!", expensive_closure.value(intensity));

    } else {
        if random_number == 3 {
            info!("Take a break today! Remember to stay hydrated!");
        } else {
            info!(
                "Today, run fo {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

struct Cacher<T>  
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32 
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation: calculation, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 { 
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn it_calling_next_directly_test01() {
    init();
    let mut counter = Counter::new();
    // info!("{:?}", counter.next());

    for next in counter.into_iter() {
        info!("{:?}", next);
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    info!("sum: {}", sum);
}

#[test]
fn it_filters_by_size_test01() {
    init();

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker")},
        Shoe { size: 13, style: String::from("sandal")},
        Shoe { size: 10, style: String::from("boot")},
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    info!("{:?}", in_my_size);

}

#[test]
fn it_iter_test01() {
    init();
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    info!("{:?}", v1);
    
    for val in v1_iter {
        info!("Got: {}", val);
    }

    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    info!("{:?}", v1_iter.next());

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    info!("{}", total);

    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    info!("{:?}", v2);

}

#[test]
fn it_close_test01() {
    init();
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    
    info!("{}", equal_to_x(y));
}

#[test]
fn it_generate_test01() {
    init();
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    genrate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

}

#[test]
fn it_longest_test01() {
    init();
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    info!("The longest string is {}", result);

}

#[test]
fn it_longest_test02(){
    init();
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        info!("The longest string is {}", result);
    }
}

#[test]
fn it_trait_test01() {
    init();

    let tweet = returns_summarizable();
    info!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    info!("New article avialble! {}", article.summarize());

    notify(article);

}