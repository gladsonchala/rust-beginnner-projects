fn main(){
  for bottles in 1..101  {
    if bottles < 100 {
      println!("Take one down and pass it around,  {} of beer on the wall.", 100 - bottles);
    } else {
      println!("No more bottles of beer on the wall");
      println!("no more bottles of beer.");
      println!("We've taken them down");
      println!("and passed them around");
      println!("now  we're drunk and passed out!");
    }
  }
}
