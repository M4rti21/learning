use std::io;

fn main() {
    // let x: u8 = 12;      0 <> 255
    // let y: i8 = 10;   -128 <> 127

    // let z: i32 = x + y; <-- error, not the same type

    // let x: u8 = 255;
    // let y: u8 = 255;

    // let z: u32 = (x as u16 + y as u16).into();
    // println!("result is {}", z);

    let mut input = String::new();
    println!("What is the value of X?");
    io::stdin().read_line(&mut input).expect("error");

    let x: i32 = input.trim().parse().unwrap();

    input = String::new();
    println!("What is the value of Y?");
    io::stdin().read_line(&mut input).expect("error");

    let y: i32 = input.trim().parse().unwrap();

    let z: i32 = x + y;
    println!("Result is {z}");
}
