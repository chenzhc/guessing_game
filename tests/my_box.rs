#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{cell::{Ref, RefCell}, ops::Deref, rc::Rc};

use guessing_game::init;
use log::info;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


use List::{Cons, Nil};

#[test]
fn it_ref_cell_test01() {
    init();
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    info!("a after = {:?}", a);
    info!("b after = {:?}", b);
    info!("c after = {:?}", c);

}

#[test]
fn it_rc_clone_test01() {
    init();
    let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    info!("{:?}", Rc::strong_count(&a));
}

pub trait Messenger {
    fn send(&self, msg: &str);    
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}


impl<'a, T> LimitTracker<'a, T> 
    where T: Messenger 
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger: messenger, value: 0, max: max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger { sent_messages: RefCell::new(vec![]) }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

#[test]
fn it_sends_an_over_75_percent_warning_message() {
    init();
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    info!("{}", mock_messenger.sent_messages.borrow().len());

}

#[test]
fn it_rc_clone_test02() {
    init();
    let a = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    info!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    info!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        info!("count after creating c = {}", Rc::strong_count(&a));
    }
    info!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[test]
fn it_box_test01() {
    init();
    let b = Box::new(5);
    info!("b = {}", b);

    // let list = Cons(1,
    //     Box::new(Cons(2,
    //         Box::new(Cons(3,
    //             Box::new(Nil))))));

    
}

#[test]
fn it_drop_test01() {
    init();
    let c = CustomSmartPointer { data: String::from("some data")} ;
    info!("CustomSmartPointer created.");
    drop(c);
    info!("CustomSmartPointer droppped before the end of main.");

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