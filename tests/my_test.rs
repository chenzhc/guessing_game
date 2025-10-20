#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::num;
use std::{collections::HashMap, fs::File, io::{self, ErrorKind, Read}, path::Ancestors};

use guessing_game::init;
use log::info;


#[test]
fn it_test01() {
    init();
    let mut  x = 5;
    info!("The value of x is: {}", x);
    x = 6;
    info!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    {
        let x = x *2;
        info!("The value of x in the inner scope is: {}", x);
    }
    info!("The value of x is: {}", x);

    let heart_eyed_cat = '😻';
    info!("{}", heart_eyed_cat);

    let a = [3; 5];
    info!("{:?}", a);

    let y = {
        let x = 3;
        x + 1
    };

    info!("The value of y is: {}", y);

    let  x= five();
    info!("The value of x is: {}", x);

    let x = plus_one(5);
    info!("The value of x is: {}", x);

    let number = 3;

    if number < 5 {
        info!("condition was true");
    } else {
        info!("condition was false");
    }

    if number != 0 {
        info!("number was something other than zero");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; 
        }
    };

    let a = [10,20,30,40,50];
    for element in a {
        info!("The value is : {}", element);
    }
    info!("The result is {}", result);

    for number in (1..4).rev() {
        info!("{}", number);
    }
    info!("LIFTOFF!!!");
    
    let s1 = gives_ownership();
    info!("{}", s1);

}

#[test]
fn it_test02() {
    init();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    info!("{}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    info!("The length of '{}' is {}.", s2, len);

    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    info!("The length of '{}' is {}", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    info!("{}", s);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    info!("{} and {}", r1, r2);
    let r3 = &mut s;
    info!("{}", r3);

    let s = String::from("hello world");
    let world = first_word(&s);
    
    info!("{}", world);


}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User { active: true, username: username, email: email, sign_in_count: 1 }
}

#[test]
fn it_test03() {
    init();
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    
    info!("{:?}", user1);

    let user2 = build_user("someone@email.com".to_string(), "test01".to_string());
    info!("{:?}", user2);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    info!("{:?}", user3);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    let subject = AlwaysEqual;

    let width1 = 30;
    let height1 = 50;
    info!("The area of the rectangle is {} square pixels.", area(width1, height1));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    info!("The area of the rectangle is {} square pixels.", area2(&rect1));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    info!("rect1 is {:?}", rect1);
    info!("{}", rect1.area());

    if rect1.width() {
        info!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    info!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    info!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    info!("{:?}", sq);

}

#[test]
fn it_match_test01() {
    init();
    let rs = value_in_cents(Coin::Dime);
    info!("{}", rs);

    let five = Some(5);
    let six = plus_one2(five);
    info!("{}",six.unwrap());
    let none = plus_one2(None);
    info!("{:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        info!("three")
    }
}

#[test]
fn it_vector_test01() {
    init();
    let v = vec![1,2,3];
    info!("{:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    info!("{:?}", v);
    
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    info!("The third element is {}", third);

    match v.get(2) {
        Some(third) => info!("The third element is {}", third),
        None => info!("There is no third element."),
    }

    let v = vec![100,32,57];
    for i in &v {
        info!("{}",i);
    }

    let mut v = vec![100,32,57];
    for i in &mut v {
        *i += 50;
    }
    info!("{:?}", v);
    let hello = String::from("こんにちは");
    info!("{}", hello);

    let mut s = String::from("hello");
    s.push_str("bar");
    info!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    info!("s2 is {}", s2);
    info!("s1 is {}", s1);

    let mut s = String::from("lo");
    s.push('l');
    info!("{}", s);

    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    info!("{}", s3);
    info!("{}", s2);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    info!(" s1 {}, s2: {}, s3: {}, s: {}", s1, s2, s3, s);

    for c in "नमस्ते".chars() {
        info!("{}", c);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    info!("{:?}", scores);

    let tems = vec![String::from("Blue"), String::from("yellow")];
    let initial_scores = vec![10,50];
    let scores: HashMap<_, _> = tems.iter().zip(initial_scores.iter()).collect();
    info!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    info!("score: {}", score.unwrap());

    for (key, value) in &scores {
        info!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    info!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    info!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    info!("{:?}", map);

}

#[test]
fn it_result_test01() {
    init();

    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

}

#[test]
fn it_largest_test01() {
    init();

    let number_list = vec![34,50,25,100,65];
    let result = largest(&number_list);
    info!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    info!("The largest number is {}", result);

    let result = largest_i32(&number_list);
    info!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    info!("The largest char is {}", result);

}

#[test]
fn it_point2_test01() {
    init();
    let p = Point2 { x: 5, y: 10 };
    info!("p.x = {}", p.x());

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    info!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    
}

struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    fn mixup<V,W>(self, other: Point3<V, W>) -> Point3<T,W> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn largest_gen<T>(list: &[T]) -> T 
    where T: PartialOrd + Copy 
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

#[test]
fn it_largest_gen_test01() {
    init();
    let number_list = vec![34,50,25,100,65];
    let result = largest_gen(&number_list);
    info!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];
    let result = largest_gen(&char_list);
    info!("The largest char is {}", result);
    
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn read_username_from_file3() -> anyhow::Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2() -> anyhow::Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn add_fancy_hat() {
    info!("add fancy hat");
}

fn remove_fancy_hat() {
    info!("remove fancy hat");
}

fn reroll() {
    info!("reroll");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            info!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            info!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one2(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0 
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height 
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

struct AlwaysEqual;
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn change(some_thing: &mut String) {
    some_thing.push_str(", world");
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    return some_string;
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn five() -> i32 {
    return 5;
}