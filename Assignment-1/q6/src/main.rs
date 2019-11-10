fn main() {
 let x = 3;
 println!("Number {}", x);
  //x = 5; //this required shadowing
 //cannot assign twice to immutable variable
 let x = 5;
 println!("Number {}", x);
}