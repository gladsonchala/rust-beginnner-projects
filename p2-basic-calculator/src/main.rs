use std::io;

fn main() {
    welcome();
    println!("Enter your 1st number: ");
    let i1: f64 = take_input().trim().parse().unwrap();
    
    println!("Enter your 2nd number: ");
    let i2: f64 = take_input().trim().parse().unwrap();
    
    println!("Addition of {} + {} = {}",i1.to_string(), i2.to_string(), add(i1,i2).to_string());
    println!("Subtraction of {} - {} = {}",i1.to_string(), i2.to_string(), subtract(i1,i2).to_string());
    println!("Multiplication of {} * {} = {}",i1.to_string(), i2.to_string(), multiply(i1,i2).to_string());
    println!("Division of {} / {} = {}",i1.to_string(), i2.to_string(), divide(i1,i2).to_string());

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

fn add(x:f64,y:f64) -> f64 {
    x + y
}

fn multiply(x:f64,y:f64) -> f64 {
    x * y
}

fn divide(x:f64,y:f64) -> f64 {
    x / y
}

fn subtract(x:f64,y:f64) -> f64 {
    x - y
}
