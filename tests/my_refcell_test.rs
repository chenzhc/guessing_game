#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{cell::{Ref, RefCell}, ops::Deref, rc::{Rc, Weak}};
use guessing_game::init;
use log::info;
use List::{Cons, Nil};


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

#[test]
fn it_weak_test02() {
    init();
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    info!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    info!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

}

#[test]
fn it_weak_test01() {
    init();

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    info!("{:?}", branch);
}


#[test]
fn it_cons_list_test01() {
    init();
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    
    info!("a initial rc count = {}", Rc::strong_count(&a));
    info!("a next time = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    info!("a rc count after b creation = {}", Rc::strong_count(&a));
    info!("b initial rc count = {}", Rc::strong_count(&b));
    info!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    info!("b rc count after changing a = {}", Rc::strong_count(&b));
    info!("a rc count after changing a = {}", Rc::strong_count(&a));
    
}

