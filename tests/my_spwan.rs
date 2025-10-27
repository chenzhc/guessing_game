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


#[test]
fn it_match_test03() {
    init();
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => info!("Got 50"),
        Some(y) => info!("matched, y = {:?}", y),
        _ => info!("Default case, x = {:?}", x),
    }

    info!("at the end: x = {:?}, y = {:?}", x, y);
}


#[test]
fn it_match_test04() {
    init();
    let x = 1;

    match x {
        1 | 2 => info!("one or two"),
        3 => info!("three"),
        _ => info!("anything"),
    }
}

#[test]
fn it_match_test05() {
    init();
    let x = 5;

    match x {
        1..=5 => info!("one through five"),
        _ => info!("something else"),
    }
}

#[test]
fn it_match_test06() {
    init();
    let x = 'c';

    match x {
        'a'..='j' => info!("early ASCII letter"),
        'k'..='z' => info!("late ASCII letter"),
        _ => info!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[test]
fn it_match_test07() {
    init();
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b} = p;
    info!("{}", a);
    info!("{}", b);

}

#[test]
fn it_match_test08() {
    init();

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    info!("x: {}, y: {}", x, y);
}

#[test]
fn it_match_test09() {
    init();
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => info!("One the x axis at {}", x),
        Point { x: 0, y } => info!("One the y axis at {}", y),
        Point { x, y} => info!("One neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
fn it_match_test10() {
    init();

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            info!("The Quit variant has no data to destructure.");
        },
        Message::Move { x, y } => {
            info!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        },
        Message::Write(text) => info!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            info!(
                "Change the color to red {}, green {}, and blue {}",
                r, g, b
            );
        }
    }
}

fn foo(_: i32, y: i32) {
    info!("This code only uses the y parameter: {}", y);
}

#[test]
fn it_match_test11() {
    init();
    foo(3, 4);
}


#[test]
fn it_match_test12() {
    init();
    let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            info!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    info!("setting is {:?}", setting_value);

}

#[test]
fn it_amtch_test13() {
    init();

    let numbers = (2, 4, 8, 16,32);
    match numbers {
        (first, _, thired, _, fifth) => {
            info!("Some numbers: {}, {}, {}", first, thired, fifth);
        },
    }
}

#[test]
fn it_match_test14() {
    init();
    let _x = 5;
    let y = 10;
    info!("{}, {}", _x, y);

    let s = Some(String::from("hello"));
    if let Some(_) = s {
        info!("found a string");
    }
    info!("{:?}", s);

}

struct Point2 {
    x: i32, 
    y: i32,
    z: i32,
}

#[test]
fn it_match_test15() {
    init();
    let origin = Point2 {x: 10, y: 0, z: 0};
    match origin {
        Point2 { x, ..} => info!("x is {}", x),
    }

    let numbers = (2,4,8,16,32);

    match numbers {
        (first, .., last) => {
            info!("Some numbers: {}, {}", first, last);
        },
    }

    let num = Some(4);
    match num {
        Some(x) if x < 5 => info!("less than five: {}", x),
        Some(x) => info!("{}", x),
        None => (),
    }
}

#[test]
fn it_match_test16() {
    init();

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => info!("Got 50"),
        Some(n) if n == y => info!("Matched, n = {}", n),
        _ => info!("Default case, x = {:?}", x),
    }
    info!("at the end: x = {:?}, y = {}", x, y);
}

#[test]
fn it_match_test17() {
    init();
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => info!("yes"),
        _ => info!("no"),
    }
}

enum Message2 {
    Hello { id: i32 },
}

#[test]
fn it_match_test18() {
    init();
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => {
            info!("Found an id in range: {}", id_variable);
        },
        Message2::Hello { id: 10..=12 } => {
            info!("Found an id in another range");
        }, 
        Message2::Hello { id } => {
            info!("Found some other id: {}", id);
        }
    }
}
