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