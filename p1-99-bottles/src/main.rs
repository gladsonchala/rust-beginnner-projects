fn main() {
    let mut n = 100;
    loop {
        if n > 2 {
            n -= 1;
            println!("{} bottles of beer on the wall, {} bottles of beer.", n, n);
            println!("Take one down and pass it around, {} bottles of beer on the wall.",n-1);
        } else if n > 1 {
            n -= 1;
            println!("{} bottles of beer on the wall, {} bottles of beer.", n, n);
            print!("Take one down and pass it around, ");
            continue;
        } else {
            println!("No more bottles of beer on the wall,");
            println!("no more bottles of beer.");
            println!("We've taken them down and passed them around;");
            println!("now we're drunk and passed out!");
            break;
        }
    }
}
