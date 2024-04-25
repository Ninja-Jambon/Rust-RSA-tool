mod rsa;
use rsa::gen_keys;
use std::io::Write;

use std::io;

fn main() {
    let mut input = String::new();

    print!("Enter an integer: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let num: i32 = input.trim().parse().expect("Invalid input");

    let (e, d, n) = gen_keys(num.try_into().unwrap());

    println!("e : {}", e);
    println!("d : {}", d);
    println!("n : {}", n);
}