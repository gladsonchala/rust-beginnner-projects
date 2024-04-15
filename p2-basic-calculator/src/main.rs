fn main() {

    println!("{}",add(3,2));
    println!("{}",multiply(-3,2));
    println!("{}",divide(3,2));
    println!("{}",subtract(3,2));

}

fn add(x:i32,y:i32) -> i32 {
    x + y
}

fn multiply(x:i32,y:i32) -> i32 {
    x * y
}

fn divide(x:i32,y:i32) -> i32 {
    x / y
}

fn subtract(x:i32,y:i32) -> i32 {
    x - y
}
