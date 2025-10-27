#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{ops::Add, sync::atomic::AtomicU32};
use guessing_game::init;
use log::info;

#[test]
fn it_unsafe_test01() {
    init();
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe  {
        info!("r1 is: {}", *r1);
        info!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {

}

#[test]
fn it_call_unsafe_fn_test01() {
    init();
    unsafe {
        dangerous();
    }
}

unsafe extern "C" {
    unsafe fn abs(input: i32) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    info!("Just called a Rust function from C!");
}

#[test]
fn it_abs_test01() {
    init();
    unsafe {
        info!("Absolute value of -3 according to C: {}", abs(-3));
        call_from_c();
    }
}

static HELLO_WORLD: &str = "Hello, world!";

#[test]
fn it_static_test01() {
    init();
    info!("name is: {}", HELLO_WORLD);
}

static COUNTER: AtomicU32  = AtomicU32::new(0);

fn add_to_count(inc: u32) {
    COUNTER.fetch_add(inc, std::sync::atomic::Ordering::Relaxed);
}

#[test]
fn it_static_test02() {
    init();

    add_to_count(3);
    info!("COUNTER: {}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn it_add_point_test01() {
    init();
    let p1 = Point {x: 1, y: 0};
    let p2 = Point { x: 2, y: 3};

    let rs_p = p1 + p2;
    info!("{:?}", rs_p);

}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[test]
fn it_add_test01() {
    init();

    let m1 = Millimeters(20);
    let m2 = Meters(10);

    let rs = m1 + m2;
    info!("{:?}", rs);

}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        info!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        info!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        info!("*waving arms furiously*");
    }
}

#[test]
fn it_human_fly_test01() {
    init();
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);
    person.fly();

}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


#[test]
fn it_animal_test01() {
    init();
    info!("A baby dog is called a {}", Dog::baby_name());
    info!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

type Kilometers = i32;

#[test]
fn it_type_test01() {
    init();
    let x: i32 = 5;
    let y: Kilometers = 5;

    info!("x + y = {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn it_do_twice_test01() {
    init();
    let answer = do_twice(add_one, 5);
    info!("The answer is: {}", answer);

}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
