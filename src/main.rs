// fn main() {
// let x = 5;
// println!("x is {}", x);

// let mut y = 10;
// println!("y is {}", y);
// y = 20;
// println!("now y is {}", y);

//     let number = 7;

//     if number < 10 {
//         println!("Less than 10");
//     } else {
//         println!("10 or more");
//     }
// }

// fn greet(name: &str) {
//     println!("Hello, {}!", name);
// }

// fn main() {
//     greet("Rust");
// }

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1; // ownership moved
//     // println!("{}", s1); // error: s1 no longer valid
// }

// use std::fs::File;

// fn main() {
//     let file = File::open("hello.txt");

//     match file {
//         Ok(f) => println!("File opened successfully"),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// struct User {
//     username: String,
//     age: u8,
// }

// fn main() {
//     let user1 = User {
//         username: String::from("djamware"),
//         age: 30,
//     };

//     println!("Username: {}", user1.username);
// }

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
}

fn main() {
    let msg = Message::Write(String::from("Hello"));

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text: {}", text),
    }
}

trait Greet {
    fn greet(&self);
}

struct Person;
struct Robot;

impl Greet for Person {
    fn greet(&self) {
        println!("Hi! I'm a person.");
    }
}

impl Greet for Robot {
    fn greet(&self) {
        println!("Beep boop. I'm a robot.");
    }
}

fn say_hello<T: Greet>(greeter: T) {
    greeter.greet();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
