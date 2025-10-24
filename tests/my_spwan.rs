#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{thread, time::Duration};

use guessing_game::init;
use log::info;

#[test]
fn it_spwan_test01() {
    init();
    let handle = thread::spawn(|| {
        for i in 1..10 {
            info!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    handle.join().unwrap();

    for i in 1..5 {
        info!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
}

#[test]
fn it_spwan_test02() {
    init();
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        info!("Here's a vector: {:?}", v);
    });

    //drop(v);

    handle.join().unwrap();
}