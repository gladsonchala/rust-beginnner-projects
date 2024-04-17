use std::io;


fn main() {

    welcome();
    println!("Enter your name: ");
    let i1: String = take_input().trim().parse().unwrap();
    println!("{:?}", i1.chars().nth(0));

}

fn welcome() {
    println!("*******************");
    println!("***** WELCOME *****");
    println!("*******************");
}

fn take_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
    input
}

