#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Deref;

use guessing_game::init;
use log::info;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

#[test]
fn it_box_test01() {
    init();
    let b = Box::new(5);
    info!("b = {}", b);

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    
}

#[test]
fn it_deref_test01() {
    init();
    let x = 5;
    let y = &x;

    info!("{}", x);
    info!("{:p}", y);
    info!("{:p}", &x);

    let x = 5;
    let y = Box::new(x);
    info!("{:p}", &x);
    info!("{:p}", &y);

    let x = 5;
    let y = MyBox::new(x);
    info!("{:p}", &x);
    info!("{:p}", &*y);
    info!("{}", x);
    info!("{}", *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer { data: String::from("other stuff")};
    info!("CustomSmartPointers created.");
    
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        info!("Dropping CustomSmartPionter with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    info!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}