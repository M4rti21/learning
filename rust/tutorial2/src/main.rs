use std::io;

fn main() {
    println!("Hello, world!");
    let mut name = String::new();

    println!("What is your name?");
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read line");

    name.trim();

    for i in 1..10 {
        println!("{i} - Your name is {name}!");
    }
}
