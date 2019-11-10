use std::{io};

fn main(){
    let mut int = String::new();
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");

    let int: i32 = match int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    println!("{}",factorial(5));
}

fn factorial(number:i32)->i32{
    let mut fact : i32 = 1;
// Enter your code here.
    for x in 1..number + 1 {
        fact = fact * x;
    }
    fact
}