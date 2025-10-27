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