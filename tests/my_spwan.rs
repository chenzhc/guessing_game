#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{mem::take, rc::Rc, sync::{mpsc, Arc, Mutex}, thread, time::Duration};

use guessing_game::{average::AveragedCollection, gui::{Button, Screen, SelectBox}, init};
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

#[test]
fn it_spwan_test03() {
    init();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let recived = rx.recv().unwrap();
    info!("Got: {}", recived);

}

#[test]
fn it_spwan_test04() {
    init();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        info!("Got: {}", received);
    }
}

#[test]
fn it_spwan_test05() {
    init();
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals  = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        info!("Got: {}", received);
    }
}

#[test]
fn it_mutex_test01() {
    init();

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    info!("m = {:?}", m);
}

#[test]
fn it_mutex_test02() {
    init();
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    info!("Result: {}", *counter.lock().unwrap());
}


#[test]
fn it_averaged_test01() {
    init();
    let mut a1 = AveragedCollection::new();

    a1.add(10);
    let av1 = a1.average();
    info!("{}", av1);
    a1.add(20);
    a1.add(30);
    let av1 = a1.average();
    info!("{}", av1);

    let v1 = a1.remove();
    info!("{:?}", v1);
    let av1 = a1.average();
    info!("{}", av1);

}


#[test]
fn it_draw_test01() {
    init();
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                heigth: 10,
                options: vec![
                    String::from("yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

#[test]
fn it_draw_test02() {
    init();
    // let screen = Screen {
    //     components: vec![
    //         Box::new(String::from("Hi")),
    //     ],
    // };
    // screen.run();
}

#[test]
fn it_match_test01() {
    init();
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        info!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        info!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            info!("Using purple as the background color");
        } else {
            info!("Using orange as the background color");
        }
    } else {
        info!("Using blue as the background color");
    }
}

#[test]
fn it_while_let_test01() {
    init();

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        info!("{}", top);
    }
}

#[test]
fn it_for_test01() {
    init();
    let v = vec!['a','b','c'];
    
    for (index, value) in v.iter().enumerate() {
        info!("{} is at index {}", value, index);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    info!("current location: ({}, {})", x, y);
}

#[test]
fn it_print_coordinates_test01() {
    init();
    let point = (3, 5);
    print_coordinates(&point);

    let some_option_value: Option<i32> = Some(3);
    if let Some(x) = some_option_value {
        info!("{}", x);
    }

}

#[test]
fn it_match_test02() {
    init();
    let x = 1;
    match x {
        1 => info!("one"),
        2 => info!("two"),
        3 => info!("three"),
        _ => info!("anything"),
    }
    
}